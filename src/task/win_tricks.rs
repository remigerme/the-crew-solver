use std::collections::HashSet;

use crate::{
    player::n_tricks_total,
    task::{Task, TaskStatus},
};

#[derive(Debug)]
pub struct TaskWinTricks {
    indexes: HashSet<usize>,
    last: bool,
    strict: bool,
}

impl TaskWinTricks {
    pub fn new<I>(indexes: I, last: bool, strict: bool) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let indexes: HashSet<usize> = indexes.into_iter().collect();
        assert!(
            indexes.len() > 0 || last,
            "at least one required index should be provided"
        );
        Self {
            indexes,
            last,
            strict,
        }
    }
}

impl Task for TaskWinTricks {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let mut indexes = self.indexes.clone();
        if self.last {
            let n_last = n_tricks_total(state.n_players()) - 1;
            indexes.insert(n_last);
        }

        let tricks_idx: HashSet<usize> = state
            .get_player(ip)
            .get_tricks()
            .iter()
            .map(|t| t.idx())
            .collect();
        let won_all = indexes.iter().all(|i| tricks_idx.contains(i));
        let current = state.get_current_trick().idx();
        let first_missing = self
            .indexes
            .iter()
            .filter(|&i| !tricks_idx.contains(i))
            .min();
        let won_another = tricks_idx.iter().any(|i| !indexes.contains(i));
        if won_all && (!self.strict || (state.game_is_over() && indexes.len() == tricks_idx.len()))
        {
            return TaskStatus::Done;
        }

        if (!won_all && *first_missing.unwrap() < current) || (won_another && self.strict) {
            return TaskStatus::Failed;
        }

        TaskStatus::Unknown
    }
}
