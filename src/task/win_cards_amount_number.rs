use std::collections::HashMap;

use crate::{
    task::{BaseTask, TaskDifficulty, TaskStatus},
    trick::Trick,
};

#[derive(Debug, Clone)]
pub struct TaskWinCardsAmountNumber {
    difficulty: Option<TaskDifficulty>,
    constraints: HashMap<usize, usize>,
    exactly: bool,
}

fn count_won_except_submarine(tricks: &[Trick], value: usize) -> usize {
    tricks.iter().fold(0, |acc, trick| {
        acc + trick
            .iter()
            .filter(|c| c.val() == value && !c.is_submarine())
            .count()
    })
}

impl TaskWinCardsAmountNumber {
    pub fn new<I>(difficulty: Option<TaskDifficulty>, exactly: bool, constraints: I) -> Self
    where
        I: IntoIterator<Item = (usize, usize)>,
    {
        Self {
            difficulty,
            exactly,
            constraints: constraints.into_iter().collect(),
        }
    }
}

impl BaseTask for TaskWinCardsAmountNumber {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let mut done = true;
        for (&value, &amount) in &self.constraints {
            let won_by_ip = count_won_except_submarine(state.get_player(ip).get_tricks(), value);

            if self.exactly && won_by_ip > amount {
                return TaskStatus::Failed;
            }

            if !self.exactly && won_by_ip >= amount {
                continue;
            }

            let missing = amount - won_by_ip;
            let mut won_by_others = 0;
            for i in 0..state.n_players() {
                if i != ip {
                    won_by_others +=
                        count_won_except_submarine(state.get_player(i).get_tricks(), value);
                }
            }
            let available = 4 - (won_by_ip + won_by_others);
            if available < missing {
                return TaskStatus::Failed;
            }
            done &= (self.exactly && available == 0) || (!self.exactly && won_by_ip >= amount);
        }

        if done {
            return TaskStatus::Done;
        }

        TaskStatus::Unknown
    }

    impl_get_difficulty!();
}
