use std::collections::HashSet;

use crate::{
    card::{COLOR_RANGE, Card},
    task::{BaseTask, TaskDifficulty, TaskStatus},
};

#[derive(Debug, Clone)]
pub struct TaskDontOpenTrickWith {
    difficulty: Option<TaskDifficulty>,
    cards: HashSet<Card>,
}

impl TaskDontOpenTrickWith {
    pub fn new<I>(difficulty: Option<TaskDifficulty>, colors: I) -> Self
    where
        I: IntoIterator<Item = fn(usize) -> Card>,
    {
        let mut cards = HashSet::new();
        for color in colors {
            for i in COLOR_RANGE {
                cards.insert(color(i));
            }
        }

        TaskDontOpenTrickWith { difficulty, cards }
    }
}

impl BaseTask for TaskDontOpenTrickWith {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        // Checking if current trick fails the task
        if ip == state.first_player()
            && let Some(first_card) = state.get_current_trick().get(0)
            && self.cards.contains(first_card)
        {
            return TaskStatus::Failed;
        }

        // Checking if any relevant card is left in the hand
        let mut all_played = true;
        for c in state.get_player(ip).get_hand().iter() {
            if self.cards.contains(c) {
                all_played = false;
            }
        }

        if all_played {
            TaskStatus::Done
        } else {
            TaskStatus::Unknown
        }
    }

    impl_difficulty!();
}
