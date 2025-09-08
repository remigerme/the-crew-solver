use std::collections::HashSet;

use crate::{
    player,
    task::{Task, TaskStatus},
};

#[derive(Debug)]
pub struct TaskWinTricks {
    indexes: HashSet<usize>,
    strict: bool,
}

impl TaskWinTricks {
    pub fn new<I>(indexes: I, strict: bool) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let indexes: HashSet<usize> = indexes.into_iter().collect();
        assert!(
            indexes.len() > 0,
            "at least one required index should be provided"
        );
        Self { indexes, strict }
    }

    pub fn new_n_first_tricks(n: usize, strict: bool) -> Self {
        Self::new(0..n, strict)
    }

    pub fn new_last_trick(n_players: usize, strict: bool) -> Self {
        player::check_valid_n_players(n_players).unwrap();
        Self::new([player::n_tricks_total(n_players) - 1], strict)
    }

    pub fn new_first_and_last(n_players: usize) -> Self {
        player::check_valid_n_players(n_players).unwrap();
        Self::new([player::n_tricks_total(n_players)], false)
    }
}

impl Task for TaskWinTricks {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let tricks_idx: HashSet<usize> = state
            .get_player(ip)
            .get_tricks()
            .iter()
            .map(|t| t.idx())
            .collect();
        let won_all = self.indexes.iter().all(|i| tricks_idx.contains(i));
        let current = state.get_current_trick().idx();
        let first_missing = self
            .indexes
            .iter()
            .filter(|&i| !tricks_idx.contains(i))
            .min();
        let won_another = tricks_idx.iter().any(|i| !self.indexes.contains(i));

        if won_all
            && (!self.strict || (state.game_is_over() && self.indexes.len() == tricks_idx.len()))
        {
            return TaskStatus::Done;
        }

        if (!won_all && *first_missing.unwrap() < current) || (won_another && self.strict) {
            return TaskStatus::Failed;
        }

        TaskStatus::Unknown
    }
}
