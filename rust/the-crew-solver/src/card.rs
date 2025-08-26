const NB_CARDS: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Card {
    Red(usize),
    Green(usize),
    Blue(usize),
    Yellow(usize),
    Trump(usize),
}

impl Card {
    pub fn is_valid(&self) -> bool {
        match *self {
            Card::Red(x) | Card::Green(x) | Card::Blue(x) | Card::Yellow(x) => 1 <= x && x <= 9,
            Card::Trump(x) => 1 <= x && x <= 4,
        }
    }

    pub fn val(&self) -> usize {
        match *self {
            Card::Red(x) | Card::Green(x) | Card::Blue(x) | Card::Yellow(x) | Card::Trump(x) => x,
        }
    }

    pub fn is_trump(&self) -> bool {
        matches!(self, Card::Trump(_))
    }

    pub fn same_color(&self, other: &Card) -> bool {
        matches!(
            (self, other),
            (Card::Red(_), Card::Red(_))
                | (Card::Green(_), Card::Green(_))
                | (Card::Blue(_), Card::Blue(_))
                | (Card::Yellow(_), Card::Yellow(_))
                | (Card::Trump(_), Card::Trump(_))
        )
    }
}
