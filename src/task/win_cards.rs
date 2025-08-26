//! Tasks where a player is required to win specific cards.
//!
//! For example, "win the 1, 2, and 3 of blue" falls in this category.
//! This also serves as an example implementation to refer to when creating a new task.
use std::collections::HashSet;

use crate::{card::Card, task::Task};

/// First, we define a **public** struct, deriving [`Debug`].
#[derive(Debug)]
pub struct TaskWinCards {
    /// All fields should be private (ie without a `pub` specifier).
    /// They are used to store the internal logic of the task.
    ///
    /// Here, we just store the cards that must be won by the player.
    cards: HashSet<Card>,
}

impl TaskWinCards {
    /// We provide a `new` method to construct the task.
    ///
    /// It should be as generic as possible, as we really don't care about the internal logic.
    pub fn new<I>(cards: I) -> Self
    where
        I: IntoIterator<Item = Card>,
    {
        TaskWinCards {
            cards: cards.into_iter().collect(),
        }
    }

    // You can define internal methods if needed.
}

/// It is required to implement the [`Task`] trait.
impl Task for TaskWinCards {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        // Checking if the player has won all cards - in that case task is done
        let player = state.current_player();
        let mut found_cards = HashSet::new();
        for trick in player.get_tricks() {
            for c in trick.iter() {
                if self.cards.contains(c) {
                    found_cards.insert(*c);
                }
            }
        }

        if self.cards == found_cards {
            return super::TaskStatus::Done;
        }

        // Checking if another player has won a relevant card - in that case task is failed
        for i in 0..state.n_players() {
            // Skipping if this is the player who should do the task
            if i == ip {
                continue;
            }

            let p = state.get_player(i);
            for trick in p.get_tricks() {
                for c in trick.iter() {
                    if self.cards.contains(c) {
                        return super::TaskStatus::Failed;
                    }
                }
            }
        }

        super::TaskStatus::Unknown
    }
}

/// You can also provide some tests to check the implementation is working as expected.
/// This is a good practice, but is optional.
#[cfg(test)]
mod test {
    use crate::{player::Player, state::State, task::TaskStatus};

    use super::*;

    #[test]
    fn done_2_cards_1_trick() {
        let task = TaskWinCards::new([Card::Blue(1), Card::Blue(2)]);
        let mut p1 = Player::new(vec![Card::Trump(4)].into());
        p1.add_task(task);
        p1.add_trick((0, vec![Card::Blue(1), Card::Blue(2)]).into())
            .unwrap();

        let p2 = Player::new(vec![Card::Trump(1)].into());
        let state = State::new(vec![p1, p2]);

        assert_eq!(
            TaskWinCards::new([Card::Blue(1), Card::Blue(2)]).eval(&state, 0),
            TaskStatus::Done
        );
    }

    #[test]
    fn done_2_cards_2_tricks() {
        let task = TaskWinCards::new([Card::Blue(1), Card::Blue(2)]);
        let mut p1 = Player::new(vec![].into());
        p1.add_task(task);
        p1.add_trick((0, vec![Card::Blue(1), Card::Trump(4)]).into())
            .unwrap();
        p1.add_trick((1, vec![Card::Blue(2), Card::Red(8)]).into())
            .unwrap();

        let p2 = Player::new(vec![].into());
        let state = State::new(vec![p1, p2]);

        assert_eq!(
            TaskWinCards::new([Card::Blue(1), Card::Blue(2)]).eval(&state, 0),
            TaskStatus::Done
        );
    }

    #[test]
    fn failed_2_cards_2_tricks() {
        let task = TaskWinCards::new([Card::Blue(1), Card::Blue(2)]);
        let mut p1 = Player::new(vec![].into());
        p1.add_task(task);
        p1.add_trick((0, vec![Card::Blue(1), Card::Trump(4)]).into())
            .unwrap();

        let mut p2 = Player::new(vec![].into());
        p2.add_trick((1, vec![Card::Blue(2), Card::Red(1)]).into())
            .unwrap();
        let state = State::new(vec![p1, p2]);

        assert_eq!(
            TaskWinCards::new([Card::Blue(1), Card::Blue(2)]).eval(&state, 0),
            TaskStatus::Failed
        );
    }

    #[test]
    fn unknown_2_cards_1_trick() {
        let task = TaskWinCards::new([Card::Blue(1), Card::Blue(2)]);
        let mut p1 = Player::new(vec![].into());
        p1.add_task(task);
        p1.add_trick((0, vec![Card::Blue(1), Card::Trump(4)]).into())
            .unwrap();

        let p2 = Player::new(vec![].into());
        let state = State::new(vec![p1, p2]);

        assert_eq!(
            TaskWinCards::new([Card::Blue(1), Card::Blue(2)]).eval(&state, 0),
            TaskStatus::Unknown
        );
    }
}
