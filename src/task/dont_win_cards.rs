use std::collections::HashSet;

use crate::{
    card::{COLOR_RANGE, COLORS, Card},
    task::{Task, TaskStatus},
};

#[derive(Debug)]
pub struct TaskDontWinCards {
    cards: HashSet<Card>,
}

impl TaskDontWinCards {
    pub fn new<I>(cards: I) -> Self
    where
        I: IntoIterator<Item = Card>,
    {
        TaskDontWinCards {
            cards: cards.into_iter().collect(),
        }
    }

    pub fn new_from_colors<I>(colors: I) -> Self
    where
        I: IntoIterator<Item = fn(usize) -> Card>,
    {
        let mut cards = HashSet::new();
        for color in colors {
            for i in COLOR_RANGE {
                cards.insert(color(i));
            }
        }

        TaskDontWinCards { cards }
    }

    pub fn new_from_values<I>(values: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let mut cards = HashSet::new();
        for i in values {
            for color in COLORS {
                cards.insert(color(i));
            }
        }

        TaskDontWinCards { cards }
    }
}

impl Task for TaskDontWinCards {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        // Checking if the player has won one of the cards - in that case the task is failed
        let player = state.get_player(ip);
        for trick in player.get_tricks() {
            for card in trick.iter() {
                if self.cards.contains(card) {
                    return TaskStatus::Failed;
                }
            }
        }

        // Checking if other players have won all relevant cards - in that case the task is won
        let mut found_cards: HashSet<Card> = HashSet::new();
        for i in 0..state.n_players() {
            // Skipping if this is the player who should do the task
            if i == ip {
                continue;
            }

            for trick in state.get_player(i).get_tricks() {
                found_cards.extend(trick.iter());
            }
        }

        if self.cards == found_cards.intersection(&self.cards).copied().collect() {
            TaskStatus::Done
        } else {
            TaskStatus::Unknown
        }
    }
}
