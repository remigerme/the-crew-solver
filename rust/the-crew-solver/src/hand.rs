use std::ops::Deref;

use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Deref for Hand {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl Hand {
    pub fn playable_cards(&self, first_card: &Card) -> Vec<Card> {
        let same_color = |c: &Card| first_card.same_color(c);
        if self.cards.iter().any(same_color) {
            self.cards.iter().copied().filter(same_color).collect()
        } else {
            self.cards.clone()
        }
    }
}
