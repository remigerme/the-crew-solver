use crate::{
    card::{COLOR_RANGE, Card},
    task::{BaseTask, TaskDifficulty, TaskStatus},
};

#[derive(Debug, Clone)]
pub struct TaskWinAllCardsColor {
    difficulty: Option<TaskDifficulty>,
}

impl TaskWinAllCardsColor {
    pub fn new(difficulty: Option<TaskDifficulty>) -> Self {
        Self { difficulty }
    }
}

impl BaseTask for TaskWinAllCardsColor {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let mut n_pink = 0;
        let mut n_green = 0;
        let mut n_blue = 0;
        let mut n_yellow = 0;
        for trick in state.get_player(ip).get_tricks() {
            for card in trick.iter() {
                match card {
                    Card::Pink(_) => n_pink += 1,
                    Card::Green(_) => n_green += 1,
                    Card::Blue(_) => n_blue += 1,
                    Card::Yellow(_) => n_yellow += 1,
                    Card::Submarine(_) => (),
                }
            }
        }

        let n_goal = COLOR_RANGE.len();
        if n_pink == n_goal || n_green == n_goal || n_blue == n_goal || n_yellow == n_goal {
            return TaskStatus::Done;
        }

        let mut doable_pink = true;
        let mut doable_green = true;
        let mut doable_blue = true;
        let mut doable_yellow = true;
        for i in 0..state.n_players() {
            if i == ip {
                continue;
            }

            for trick in state.get_player(i).get_tricks() {
                for card in trick.iter() {
                    match card {
                        Card::Pink(_) => doable_pink = false,
                        Card::Green(_) => doable_green = false,
                        Card::Blue(_) => doable_blue = false,
                        Card::Yellow(_) => doable_yellow = false,
                        Card::Submarine(_) => (),
                    }
                }
            }
        }

        if !doable_pink && !doable_green && !doable_blue && !doable_yellow {
            return TaskStatus::Failed;
        }

        TaskStatus::Unknown
    }

    impl_get_difficulty!();
}
