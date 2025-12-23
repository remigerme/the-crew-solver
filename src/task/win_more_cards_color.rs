use crate::{
    card::Card,
    task::{Task, TaskStatus},
    trick::Trick,
};

#[derive(Debug)]
pub struct TaskWinMoreCardsColor {
    more_of: fn(usize) -> Card,
    fewer_of: fn(usize) -> Card,
    equal: bool,
}

impl TaskWinMoreCardsColor {
    pub fn new(more_of: fn(usize) -> Card, fewer_of: fn(usize) -> Card, equal: bool) -> Self {
        Self {
            more_of,
            fewer_of,
            equal,
        }
    }

    pub fn new_same_amount_pink_and_yellow() -> Self {
        Self::new(Card::Pink, Card::Yellow, true)
    }

    pub fn new_more_yellow_than_blue() -> Self {
        Self::new(Card::Yellow, Card::Blue, false)
    }

    pub fn new_more_pink_than_green() -> Self {
        Self::new(Card::Pink, Card::Green, false)
    }
}

fn count_won(tricks: &[Trick], color: fn(usize) -> Card) -> usize {
    tricks.iter().fold(0, |acc, trick| {
        acc + trick.iter().filter(|c| c.same_color(&color(1))).count()
    })
}

impl Task for TaskWinMoreCardsColor {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let tricks = state.get_player(ip).get_tricks();
        let won_more_of = count_won(tricks, self.more_of);
        let won_fewer_of = count_won(tricks, self.fewer_of);

        let mut left_more_of = 0;
        let mut left_fewer_of = 0;
        for i in 0..state.n_players() {
            if i == ip {
                continue;
            }

            let tricks = state.get_player(i).get_tricks();
            left_more_of += count_won(tricks, self.more_of);
            left_fewer_of += count_won(tricks, self.fewer_of);
        }

        if !self.equal {
            if won_more_of > won_fewer_of + left_fewer_of {
                return TaskStatus::Done;
            }

            if won_more_of + left_more_of <= won_fewer_of {
                return TaskStatus::Failed;
            }
        } else {
            if won_more_of == won_fewer_of && left_more_of == 0 && left_fewer_of == 0 {
                return TaskStatus::Done;
            }

            if won_more_of > won_fewer_of + left_fewer_of {
                return TaskStatus::Failed;
            }

            if won_fewer_of > won_more_of + left_more_of {
                return TaskStatus::Failed;
            }
        }

        TaskStatus::Unknown
    }
}
