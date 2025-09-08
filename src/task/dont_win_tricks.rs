use std::collections::HashSet;

use crate::{
    player,
    task::{Task, TaskStatus},
};

#[derive(Debug)]
pub struct TaskDontWinTricks {
    indexes: HashSet<usize>,
}

impl TaskDontWinTricks {
    pub fn new<I>(indexes: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let indexes: HashSet<usize> = indexes.into_iter().collect();
        assert!(
            indexes.len() > 0,
            "at least one forbidden index should be provided"
        );
        Self { indexes }
    }

    pub fn new_n_first_tricks(n: usize) -> Self {
        Self::new(0..n)
    }

    pub fn new_last_trick(n_players: usize) -> Self {
        player::check_valid_n_players(n_players).unwrap();
        Self::new([player::n_tricks_total(n_players) - 1])
    }

    pub fn new_any(n_players: usize) -> Self {
        player::check_valid_n_players(n_players).unwrap();
        Self::new(0..player::n_tricks_total(n_players))
    }
}

impl Task for TaskDontWinTricks {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        // Checking if we won a relevant trick
        if state
            .get_player(ip)
            .get_tricks()
            .iter()
            .any(|t| self.indexes.contains(&t.idx()))
        {
            return TaskStatus::Failed;
        }

        // Checking if we are done (no risk we win a relevant trick)
        let m = self.indexes.iter().max().unwrap();
        if state.get_current_trick().idx() > *m {
            return TaskStatus::Done;
        }

        TaskStatus::Unknown
    }
}
