use std::collections::HashMap;

use crate::{
    task::{Task, TaskStatus},
    trick::Trick,
};

#[derive(Debug)]
pub struct TaskWinCardsAmountNumber {
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
    pub fn new(exactly: bool, constraints: HashMap<usize, usize>) -> Self {
        Self {
            exactly,
            constraints,
        }
    }

    pub fn new_at_least_three_5s() -> Self {
        let constraints = HashMap::from([(5, 3)]);
        Self::new(false, constraints)
    }

    pub fn new_at_least_three_9s() -> Self {
        let constraints = HashMap::from([(9, 3)]);
        Self::new(false, constraints)
    }

    pub fn new_at_least_two_7s() -> Self {
        let constraints = HashMap::from([(7, 2)]);
        Self::new(false, constraints)
    }

    pub fn new_exactly_three_6s() -> Self {
        let constraints = HashMap::from([(6, 3)]);
        Self::new(true, constraints)
    }

    pub fn new_exactly_two_9s() -> Self {
        let constraints = HashMap::from([(9, 2)]);
        Self::new(true, constraints)
    }
}

impl Task for TaskWinCardsAmountNumber {
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
}
