use crate::{
    card::Card,
    task::{BaseTask, TaskDifficulty, TaskStatus},
    trick::Trick,
};

#[derive(Debug, Clone)]
pub struct TaskWinMoreCardsColor {
    difficulty: Option<TaskDifficulty>,
    more_of: fn(usize) -> Card,
    fewer_of: fn(usize) -> Card,
    equal: bool,
}

impl TaskWinMoreCardsColor {
    pub fn new(
        difficulty: Option<TaskDifficulty>,
        more_of: fn(usize) -> Card,
        fewer_of: fn(usize) -> Card,
        equal: bool,
    ) -> Self {
        Self {
            difficulty,
            more_of,
            fewer_of,
            equal,
        }
    }
}

fn count_color(cards: &[Card], color: fn(usize) -> Card) -> usize {
    cards.iter().filter(|c| c.same_color(&color(1))).count()
}

fn count_won(tricks: &[Trick], color: fn(usize) -> Card) -> usize {
    tricks
        .iter()
        .fold(0, |acc, trick| acc + count_color(trick, color))
}

impl BaseTask for TaskWinMoreCardsColor {
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

            let hand = state.get_player(i).get_hand();
            left_more_of += count_color(hand, self.more_of);
            left_fewer_of += count_color(hand, self.fewer_of);
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

    impl_difficulty!();
}
