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

#[cfg(test)]
mod test {
    use crate::{player::Player, state::State};

    use super::*;

    #[test]
    fn done_blue_3_tricks() {
        let task = TaskDontWinCards::new_from_colors([Card::Blue as fn(usize) -> Card]);
        let p1 = Player::new([Card::Submarine(4)].into());
        let mut p2 = Player::new([Card::Green(4)].into());
        p2.add_task(task);
        let mut p3 = Player::new([Card::Pink(8)].into());
        p3.add_trick((0, 0, [Card::Blue(7), Card::Blue(8), Card::Blue(9)]).into())
            .unwrap();
        p3.add_trick((1, 2, [Card::Blue(6), Card::Blue(5), Card::Blue(4)]).into())
            .unwrap();
        p3.add_trick((2, 2, [Card::Blue(3), Card::Blue(1), Card::Blue(2)]).into())
            .unwrap();
        let state = State::new([p1, p2, p3]);

        assert_eq!(
            TaskDontWinCards::new_from_colors([Card::Blue as fn(usize) -> Card]).eval(&state, 1),
            TaskStatus::Done
        )
    }

    #[test]
    fn failed_5s_1_trick() {
        let task = TaskDontWinCards::new_from_values([5]);
        let p1 = Player::new([].into());
        let mut p2 = Player::new([].into());
        p2.add_trick((0, 1, [Card::Submarine(4), Card::Pink(5)]).into())
            .unwrap();
        p2.add_task(task);
        let state = State::new([p1, p2]);

        assert_eq!(
            TaskDontWinCards::new_from_values([5]).eval(&state, 1),
            TaskStatus::Failed
        );
    }

    #[test]
    fn unknown_green_pink_2_tricks() {
        let task =
            TaskDontWinCards::new_from_colors([Card::Green as fn(usize) -> Card, Card::Pink]);
        let mut p1 = Player::new([Card::Pink(2)].into());
        p1.add_task(task);
        let mut p2 = Player::new([Card::Yellow(6)].into());
        p2.add_trick((0, 1, [Card::Submarine(4), Card::Green(5)]).into())
            .unwrap();
        p2.add_trick((1, 1, [Card::Pink(9), Card::Blue(2)]).into())
            .unwrap();
        let state = State::new([p1, p2]);

        assert_eq!(
            TaskDontWinCards::new_from_colors([Card::Green as fn(usize) -> Card, Card::Pink])
                .eval(&state, 0),
            TaskStatus::Unknown
        );
    }
}
