use std::{
    collections::HashSet,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trick {
    idx: usize,
    first_player: usize,
    cards: Vec<Card>,
}

impl Deref for Trick {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl DerefMut for Trick {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cards
    }
}

impl<I> From<(usize, usize, I)> for Trick
where
    I: IntoIterator<Item = Card> + Debug,
{
    fn from(value: (usize, usize, I)) -> Self {
        let cards: Vec<Card> = value.2.into_iter().collect();

        for c in &cards {
            if !c.is_valid() {
                panic!("Creating a trick with an invalid card: {:?}", c);
            }
        }

        let unique_cards: HashSet<Card> = cards.iter().copied().collect();
        if cards.len() != unique_cards.len() {
            panic!("Creating a trick with duplicate cards: {:?}", cards);
        }

        Trick {
            idx: value.0,
            first_player: value.1,
            cards,
        }
    }
}

impl Trick {
    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn incr(&mut self) {
        self.idx += 1
    }

    pub fn get_first_player(&self) -> usize {
        self.first_player
    }

    pub fn set_first_player(&mut self, first_player: usize) {
        self.first_player = first_player;
    }

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

    pub fn winner(&self) -> usize {
        let n_players = self.cards.len();
        if self.first_player >= n_players {
            panic!(
                "Unexpected value of first_player: {} (should be < {})",
                self.first_player, n_players
            );
        }
        (self.winner_rel() + self.first_player) % n_players
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_trick_with_invalid_card() {
        let _trick: Trick = (0, 0, vec![Card::Blue(0)]).into();
    }

    #[test]
    #[should_panic]
    fn test_trick_with_duplicates() {
        let _trick: Trick = (0, 0, vec![Card::Blue(1), Card::Blue(1)]).into();
    }

    #[test]
    fn test_winner_all_same_color() {
        let mut trick: Trick = (
            0,
            0,
            vec![Card::Blue(1), Card::Blue(7), Card::Blue(4), Card::Blue(5)],
        )
            .into();

        assert_eq!(trick.winner(), 1);
        trick.set_first_player(1);
        assert_eq!(trick.winner(), 2);
        trick.set_first_player(2);
        assert_eq!(trick.winner(), 3);
        trick.set_first_player(3);
        assert_eq!(trick.winner(), 0);
    }

    #[test]
    fn test_winner_others_discarded() {
        let mut trick: Trick = (
            0,
            0,
            vec![Card::Blue(1), Card::Red(2), Card::Red(9), Card::Yellow(8)],
        )
            .into();

        assert_eq!(trick.winner(), 0);
        trick.set_first_player(1);
        assert_eq!(trick.winner(), 1);
        trick.set_first_player(2);
        assert_eq!(trick.winner(), 2);
        trick.set_first_player(3);
        assert_eq!(trick.winner(), 3);
    }

    #[test]
    fn test_winner_some_trumped() {
        let mut trick: Trick = (
            0,
            0,
            vec![Card::Blue(1), Card::Blue(7), Card::Trump(2), Card::Trump(3)],
        )
            .into();

        assert_eq!(trick.winner(), 3);
        trick.set_first_player(1);
        assert_eq!(trick.winner(), 0);
        trick.set_first_player(2);
        assert_eq!(trick.winner(), 1);
        trick.set_first_player(3);
        assert_eq!(trick.winner(), 2);
    }

    #[test]
    #[should_panic]
    fn test_winner_invalid_first_player() {
        let trick: Trick = (0, 2, vec![Card::Blue(1), Card::Blue(2)]).into();
        trick.winner();
    }
}
