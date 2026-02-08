use std::collections::HashSet;

use crate::task::{BaseTask, TaskDifficulty, TaskStatus};

#[derive(Debug, Clone)]
pub struct TaskDontWinTricks {
    difficulty: Option<TaskDifficulty>,
    indexes: HashSet<usize>,
    any: bool,
}

impl TaskDontWinTricks {
    fn new<I>(difficulty: Option<TaskDifficulty>, indexes: I, any: bool) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let indexes: HashSet<usize> = indexes.into_iter().collect();
        assert!(
            indexes.len() > 0 || any,
            "at least one forbidden index should be provided"
        );
        Self {
            difficulty,
            indexes,
            any,
        }
    }

    pub fn new_n_first_tricks(difficulty: Option<TaskDifficulty>, n: usize) -> Self {
        Self::new(difficulty, 0..n, false)
    }

    pub fn new_any(difficulty: Option<TaskDifficulty>) -> Self {
        Self::new(difficulty, [], true)
    }
}

impl BaseTask for TaskDontWinTricks {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        // Checking if we won a relevant trick
        if state
            .get_player(ip)
            .get_tricks()
            .iter()
            .any(|t| self.indexes.contains(&t.idx()) || self.any)
        {
            return TaskStatus::Failed;
        }

        // Checking if we are done (no risk we win a relevant trick)
        // We add an extra check for the any case
        let m = self.indexes.iter().max();
        let done = match m {
            Some(m) => state.get_current_trick().idx() > *m || state.game_is_over(),
            None => state.game_is_over(),
        };
        if done {
            return TaskStatus::Done;
        }

        TaskStatus::Unknown
    }

    impl_difficulty!();
}
