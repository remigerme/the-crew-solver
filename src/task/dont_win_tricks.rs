use std::collections::HashSet;

use crate::task::{Task, TaskStatus};

#[derive(Debug)]
pub struct TaskDontWinTricks {
    indexes: HashSet<usize>,
    any: bool,
}

impl TaskDontWinTricks {
    fn new<I>(indexes: I, any: bool) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let indexes: HashSet<usize> = indexes.into_iter().collect();
        assert!(
            indexes.len() > 0,
            "at least one forbidden index should be provided"
        );
        Self { indexes, any }
    }

    pub fn new_n_first_tricks(n: usize) -> Self {
        Self::new(0..n, false)
    }

    pub fn new_any() -> Self {
        Self::new([], true)
    }
}

impl Task for TaskDontWinTricks {
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
        let m = self.indexes.iter().max().unwrap();
        if state.get_current_trick().idx() > *m || state.game_is_over() {
            return TaskStatus::Done;
        }

        TaskStatus::Unknown
    }
}
