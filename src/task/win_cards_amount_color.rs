use std::collections::HashMap;

use crate::{
    card::{COLOR_RANGE, COLORS, Card},
    task::{Task, TaskStatus},
    trick::Trick,
};

#[derive(Debug)]
pub struct TaskWinCardsAmountColor {
    constraints: HashMap<fn(usize) -> Card, usize>,
    exactly: bool,
}

impl TaskWinCardsAmountColor {
    pub fn new(exactly: bool, constraints: HashMap<fn(usize) -> Card, usize>) -> Self {
        Self {
            exactly,
            constraints,
        }
    }

    pub fn new_exactly_1_pink_1_green() -> Self {
        let constraints = HashMap::from([(Card::Red as fn(usize) -> Card, 1), (Card::Green, 1)]);
        Self::new(true, constraints)
    }

    pub fn new_at_least_7_yellow() -> Self {
        let constraints = HashMap::from([(Card::Yellow as fn(usize) -> Card, 7)]);
        Self::new(false, constraints)
    }

    pub fn new_at_least_5_pink() -> Self {
        let constraints = HashMap::from([(Card::Red as fn(usize) -> Card, 5)]);
        Self::new(false, constraints)
    }

    pub fn new_exactly_1_pink() -> Self {
        let constraints = HashMap::from([(Card::Red as fn(usize) -> Card, 1)]);
        Self::new(true, constraints)
    }

    pub fn new_exactly_2_green() -> Self {
        let constraints = HashMap::from([(Card::Green as fn(usize) -> Card, 2)]);
        Self::new(true, constraints)
    }

    pub fn new_exactly_2_blue() -> Self {
        let constraints = HashMap::from([(Card::Blue as fn(usize) -> Card, 2)]);
        Self::new(true, constraints)
    }
}

fn count_won(tricks: &[Trick], color: fn(usize) -> Card) -> usize {
    tricks.iter().fold(0, |acc, trick| {
        acc + trick.iter().filter(|c| c.same_color(&color(1))).count()
    })
}

impl Task for TaskWinCardsAmountColor {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let mut done = true;
        for (&color, &v) in &self.constraints {
            assert!(COLORS.contains(&color));
            let won_by_ip = count_won(state.get_player(ip).get_tricks(), color);

            if self.exactly && won_by_ip > v {
                return TaskStatus::Failed;
            }

            if !self.exactly && won_by_ip >= v {
                continue;
            }

            let missing = v - won_by_ip;
            let mut won_by_others = 0;
            for i in 0..state.n_players() {
                if i != ip {
                    won_by_others += count_won(state.get_player(i).get_tricks(), color);
                }
            }
            let available = COLOR_RANGE.len() - (won_by_ip + won_by_others);
            if available < missing {
                return TaskStatus::Failed;
            }
            done &= (self.exactly && available == 0) || (!self.exactly && won_by_ip >= v);
        }

        if done {
            return TaskStatus::Done;
        }

        TaskStatus::Unknown
    }
}
