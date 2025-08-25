use std::ops::Deref;

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

    pub fn winner(&self, first_player: usize, n_players: usize) -> usize {
        (self.winner_rel() + first_player) % n_players
    }
}
