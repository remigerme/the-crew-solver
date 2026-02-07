use crate::task::{BaseTask, TaskDifficulty, TaskStatus};

#[derive(Debug, Clone)]
pub struct TaskWinSpecificSubmarine {
    difficulty: Option<TaskDifficulty>,
    value: usize,
}

impl TaskWinSpecificSubmarine {
    pub fn new(difficulty: Option<TaskDifficulty>, value: usize) -> Self {
        Self { difficulty, value }
    }
}

impl BaseTask for TaskWinSpecificSubmarine {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let player = state.get_player(ip);

        let mut won_target = false;
        for trick in player.get_tricks() {
            for card in trick.iter() {
                if card.is_submarine() {
                    if card.val() == self.value {
                        won_target = true;
                    } else {
                        return TaskStatus::Failed;
                    }
                }
            }
        }

        let mut target_in_hand = false;
        let mut biggest_other_in_hand = None;
        for card in player.get_hand().iter() {
            if card.is_submarine() {
                if card.val() == self.value {
                    target_in_hand = true;
                } else {
                    biggest_other_in_hand = std::cmp::max(biggest_other_in_hand, Some(card.val()));
                }
            }
        }

        if won_target && biggest_other_in_hand.is_none() {
            return TaskStatus::Done;
        }

        if !won_target && !target_in_hand {
            return TaskStatus::Failed;
        }

        let mut biggest_other_submarine_left = None;
        for i in 0..state.n_players() {
            if i == ip {
                continue;
            }

            let mut biggest_submarine = None;
            for card in state.get_player(i).get_hand().iter() {
                if card.is_submarine() {
                    biggest_submarine = std::cmp::max(biggest_submarine, Some(card.val()));
                }
            }

            biggest_other_submarine_left =
                std::cmp::max(biggest_other_submarine_left, biggest_submarine);
        }

        match (biggest_other_in_hand, biggest_other_submarine_left) {
            (Some(_), None) => return TaskStatus::Failed,
            (Some(x), Some(y)) => {
                if x > y {
                    return TaskStatus::Failed;
                }
            }
            (None, _) => (),
        }

        TaskStatus::Unknown
    }

    impl_get_difficulty!();
}
