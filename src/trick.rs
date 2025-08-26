use std::{collections::HashSet, ops::Deref};

use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trick {
    idx: usize,
    cards: Vec<Card>,
}

impl Deref for Trick {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl From<(usize, Vec<Card>)> for Trick {
    fn from(value: (usize, Vec<Card>)) -> Self {
        for c in &value.1 {
            if !c.is_valid() {
                panic!("Creating a trick with an invalid card: {:?}", c);
            }
        }

        if value.1.len() != value.1.iter().copied().collect::<HashSet<Card>>().len() {
            panic!("Creating a trick with duplicate cards: {:?}", value.1);
        }

        Trick {
            idx: value.0,
            cards: value.1,
        }
    }
}

impl Trick {
    fn argmax<F>(&self, f: F) -> usize
    where
        F: Fn(&Card) -> bool,
    {
        let mut best_i = 0;
        for (i, c) in self.cards.iter().enumerate() {
            if f(c) && c.val() > self.cards[best_i].val() {
                best_i = i;
            }
        }

        best_i
    }

    fn winner_rel(&self) -> usize {
        if self.cards.iter().any(Card::is_trump) {
            self.argmax(Card::is_trump)
        } else {
            let first_card = self.cards[0];
            self.argmax(|c| c.same_color(&first_card))
        }
    }

    pub fn winner(&self, first_player: usize) -> usize {
        let n_players = self.cards.len();
        if first_player >= n_players {
            panic!(
                "Unexpected value of first_player: {} (should be < {})",
                first_player, n_players
            );
        }
        (self.winner_rel() + first_player) % n_players
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_trick_with_invalid_card() {
        let _trick: Trick = (0, vec![Card::Blue(0)]).into();
    }

    #[test]
    #[should_panic]
    fn test_trick_with_duplicates() {
        let _trick: Trick = (0, vec![Card::Blue(1), Card::Blue(1)]).into();
    }

    #[test]
    fn test_winner_all_same_color() {
        let trick: Trick = (
            0,
            vec![Card::Blue(1), Card::Blue(7), Card::Blue(4), Card::Blue(5)],
        )
            .into();

        assert_eq!(trick.winner(0), 1);
        assert_eq!(trick.winner(1), 2);
        assert_eq!(trick.winner(2), 3);
        assert_eq!(trick.winner(3), 0);
    }

    #[test]
    fn test_winner_others_discarded() {
        let trick: Trick = (
            0,
            vec![Card::Blue(1), Card::Red(2), Card::Red(9), Card::Yellow(8)],
        )
            .into();

        assert_eq!(trick.winner(0), 0);
        assert_eq!(trick.winner(1), 1);
        assert_eq!(trick.winner(2), 2);
        assert_eq!(trick.winner(3), 3);
    }

    #[test]
    fn test_winner_some_trumped() {
        let trick: Trick = (
            0,
            vec![Card::Blue(1), Card::Blue(7), Card::Trump(2), Card::Trump(3)],
        )
            .into();

        assert_eq!(trick.winner(0), 3);
        assert_eq!(trick.winner(1), 0);
        assert_eq!(trick.winner(2), 1);
        assert_eq!(trick.winner(3), 2);
    }

    #[test]
    #[should_panic]
    fn test_winner_invalid_first_player() {
        let trick: Trick = (0, vec![Card::Blue(1), Card::Blue(2)]).into();
        trick.winner(2);
    }
}
