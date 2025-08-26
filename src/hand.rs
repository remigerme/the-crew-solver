use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

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

impl DerefMut for Hand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cards
    }
}

impl From<Vec<Card>> for Hand {
    fn from(value: Vec<Card>) -> Self {
        for c in &value {
            if !c.is_valid() {
                panic!("Creating a hand with an invalid card: {:?}", c);
            }
        }

        if value.len() != value.iter().copied().collect::<HashSet<Card>>().len() {
            panic!("Creating a hand with duplicate cards: {:?}", value);
        }

        Hand { cards: value }
    }
}

impl Hand {
    pub fn playable_cards(&self, first_card: Option<&Card>) -> Vec<Card> {
        if let Some(first_card) = first_card
            && self.cards.iter().any(|c| first_card.same_color(c))
        {
            self.cards
                .iter()
                .copied()
                .filter(|c| first_card.same_color(c))
                .collect()
        } else {
            self.cards.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_hand_with_invalid_card() {
        let _hand: Hand = vec![Card::Blue(0)].into();
    }

    #[test]
    #[should_panic]
    fn test_hand_with_duplicates() {
        let _hand: Hand = vec![Card::Blue(1), Card::Blue(1)].into();
    }

    #[test]
    fn test_playable_cards() {
        let hand: Hand = vec![Card::Blue(1), Card::Blue(2), Card::Red(5), Card::Trump(3)].into();

        assert_eq!(
            hand.playable_cards(Some(&Card::Blue(5))),
            vec![Card::Blue(1), Card::Blue(2)]
        );
        assert_eq!(hand.playable_cards(Some(&Card::Red(3))), vec![Card::Red(5)]);
        assert_eq!(
            hand.playable_cards(Some(&Card::Trump(4))),
            vec![Card::Trump(3)]
        );

        assert_eq!(hand.playable_cards(Some(&Card::Yellow(5))), hand.cards);
        assert_eq!(hand.playable_cards(None), hand.cards);
    }
}
