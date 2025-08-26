pub const NB_CARDS: usize = 40;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid_valid() {
        for i in 1..10 {
            for variant in [Card::Red, Card::Green, Card::Blue, Card::Yellow] {
                assert!(variant(i).is_valid());
            }
        }
        for i in 1..5 {
            assert!(Card::Trump(i).is_valid());
        }
    }

    #[test]
    fn test_is_valid_invalid() {
        for i in [0, 10, 11, 12] {
            for variant in [Card::Red, Card::Green, Card::Blue, Card::Yellow] {
                assert!(!variant(i).is_valid());
            }
        }
        for i in [0, 5, 6, 7, 8, 9, 10] {
            assert!(!Card::Trump(i).is_valid());
        }
    }

    #[test]
    fn test_val() {
        for i in 1..10 {
            for variant in [Card::Red, Card::Green, Card::Blue, Card::Yellow] {
                assert_eq!(variant(i).val(), i);
            }
        }
        for i in 1..5 {
            assert_eq!(Card::Trump(i).val(), i);
        }
    }

    #[test]
    fn test_is_trump() {
        for i in 1..10 {
            for variant in [Card::Red, Card::Green, Card::Blue, Card::Yellow] {
                assert!(!variant(i).is_trump());
            }
        }
        for i in 1..5 {
            assert!(Card::Trump(i).is_trump());
        }
    }

    #[test]
    fn test_is_same_color_same() {
        for i in 1..10 {
            for j in 1..10 {
                for variant in [Card::Red, Card::Green, Card::Blue, Card::Yellow] {
                    assert!(variant(i).same_color(&variant(j)));
                }
            }
        }
        for i in 1..5 {
            for j in 1..5 {
                assert!(Card::Trump(i).same_color(&Card::Trump(j)));
            }
        }
    }

    #[test]
    fn test_is_same_color_different() {
        let variants = [
            (10, Card::Red as fn(usize) -> Card),
            (10, Card::Green),
            (10, Card::Blue),
            (10, Card::Yellow),
            (5, Card::Trump),
        ];

        for (i1, (b1, v1)) in variants.iter().enumerate() {
            for (i2, (b2, v2)) in variants.iter().enumerate() {
                if i1 == i2 {
                    continue;
                }
                for i in 1..*b1 {
                    for j in 1..*b2 {
                        assert!(!v1(i).same_color(&v2(j)));
                    }
                }
            }
        }
    }
}
