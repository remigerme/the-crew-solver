use std::fmt::Debug;

use crate::{
    card::{BLUE, GREEN, PINK, SUBMARINE, YELLOW, blue, green, pink, submarine, yellow},
    state::State,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Done,
    Unknown,
    Failed,
}

pub trait Task: Debug {
    fn eval(&self, state: &State, ip: usize) -> TaskStatus;
}

// Add your task module here.

pub mod dont_open_trick_with;
pub mod dont_win_cards;
pub mod dont_win_consecutive_tricks;
pub mod dont_win_tricks;
pub mod win_all_cards_color;
pub mod win_cards;
pub mod win_cards_amount_color;
pub mod win_cards_amount_number;
pub mod win_consecutive_tricks;
pub mod win_more_cards_color;
pub mod win_more_tricks;
pub mod win_nb_tricks;
pub mod win_nb_tricks_compared_captain;
pub mod win_specific_submarine;
pub mod win_trick_with;
pub mod win_trick_with_pred;
pub mod win_tricks;

// Concrete task instances

macro_rules! decl_win_cards {
    ($name:ident, $($card:expr),*) => {
        pub fn $name() -> win_cards::TaskWinCards {
            win_cards::TaskWinCards::new([$($card),*])
        }
    };
}

macro_rules! decl_dont_win_cards_colors {
    ($name:ident, $($color:expr),*) => {
        pub fn $name() -> dont_win_cards::TaskDontWinCards {
            dont_win_cards::TaskDontWinCards::new_from_colors([$($color),*])
        }
    };
}

macro_rules! decl_dont_win_cards_values {
    ($name:ident, $($value:expr),*) => {
        pub fn $name() -> dont_win_cards::TaskDontWinCards {
            dont_win_cards::TaskDontWinCards::new_from_values([$($value),*])
        }
    };
}

macro_rules! decl_dont_open_with {
    ($name:ident, $($color:expr),*) => {
        pub fn $name() -> dont_open_trick_with::TaskDontOpenTrickWith {
            dont_open_trick_with::TaskDontOpenTrickWith::new([$($color),*])
        }
    };
}

macro_rules! decl_win_trick_with {
    ($name:ident, $value:expr) => {
        pub fn $name() -> win_trick_with::TaskWinTrickWith {
            win_trick_with::TaskWinTrickWith::new($value, None)
        }
    };
    ($name:ident, $value:expr, $must_win:expr) => {
        pub fn $name() -> win_trick_with::TaskWinTrickWith {
            win_trick_with::TaskWinTrickWith::new($value, Some($must_win))
        }
    };
}

macro_rules! decl_win_nb_tricks_compared_captain {
    ($name:ident, $ord:expr) => {
        pub fn $name() -> win_nb_tricks_compared_captain::TaskWinNbTricksComparedCaptain {
            win_nb_tricks_compared_captain::TaskWinNbTricksComparedCaptain::new($ord)
        }
    };
}

// TODO

macro_rules! decl_dont_win_first_tricks {
    ($name:ident, $n:expr) => {
        pub fn $name() -> dont_win_tricks::TaskDontWinTricks {
            dont_win_tricks::TaskDontWinTricks::new_n_first_tricks($n)
        }
    };
}

macro_rules! decl_win_tricks {
    ($name:ident, $idx: expr, $last:expr, $strict:expr) => {
        pub fn $name() -> win_tricks::TaskWinTricks {
            win_tricks::TaskWinTricks::new($idx, $last, $strict)
        }
    };
}
macro_rules! decl_win_nb_tricks {
    ($name:ident, $n:expr) => {
        pub fn $name() -> win_nb_tricks::TaskWinNbTricks {
            win_nb_tricks::TaskWinNbTricks::new($n)
        }
    };
}

macro_rules! decl_win_cards_amount_color {
    ($name:ident, $exact:expr, $($constr:expr),*) => {
        pub fn $name() -> win_cards_amount_color::TaskWinCardsAmountColor {
            win_cards_amount_color::TaskWinCardsAmountColor::new($exact, [$($constr),*])
        }
    }
}

macro_rules! decl_win_cards_amount_number {
    ($name:ident, $exact:expr, $($constr:expr),*) => {
        pub fn $name() -> win_cards_amount_number::TaskWinCardsAmountNumber {
            win_cards_amount_number::TaskWinCardsAmountNumber::new($exact, [$($constr),*])
        }
    }
}

macro_rules! decl_win_consecutive_tricks {
    ($name:ident, $amount:expr, $exact:expr) => {
        pub fn $name() -> win_consecutive_tricks::TaskWinConsecutiveTricks {
            win_consecutive_tricks::TaskWinConsecutiveTricks::new($amount, $exact)
        }
    };
}

macro_rules! decl_win_specific_submarine {
    ($name:ident, $v:expr) => {
        pub fn $name() -> win_specific_submarine::TaskWinSpecificSubmarine {
            win_specific_submarine::TaskWinSpecificSubmarine::new($v)
        }
    };
}

macro_rules! decl_win_more_cards_color {
    ($name:ident, $c1:expr, $c2:expr, $eq:expr) => {
        pub fn $name() -> win_more_cards_color::TaskWinMoreCardsColor {
            win_more_cards_color::TaskWinMoreCardsColor::new($c1, $c2, $eq)
        }
    };
}

macro_rules! decl_win_more_tricks {
    ($name:ident, $together:expr, $fewer:expr) => {
        pub fn $name() -> win_more_tricks::TaskWinMoreTricks {
            win_more_tricks::TaskWinMoreTricks::new($together, $fewer)
        }
    };
}

decl_win_cards!(win_pink_1, pink(1));
decl_win_cards!(win_yellow_1, yellow(1));
decl_win_cards!(win_blue_4, blue(4));
decl_win_cards!(win_green_6, green(6));
decl_win_cards!(win_all_3s, pink(3), blue(3), green(3), yellow(3));
decl_win_cards!(win_all_9s, pink(9), blue(9), green(9), yellow(9));
decl_win_cards!(win_blue_1_2_3, blue(1), blue(2), blue(3));
decl_win_cards!(win_blue_6_yellow_7, blue(6), yellow(7));
decl_win_cards!(win_pink_5_yellow_6, pink(5), yellow(6));
decl_win_cards!(win_green_5_blue_8, green(5), blue(8));
decl_win_cards!(win_blue_5_pink_8, blue(5), pink(8));
decl_win_cards!(win_pink_9_yellow_8, pink(9), yellow(8));
decl_win_cards!(win_pink_1_green_7, pink(1), green(7));
decl_win_cards!(win_yellow_9_blue_7, yellow(9), blue(7));
decl_win_cards!(win_green_3_yellow_4_5, green(3), yellow(4), yellow(5));
decl_win_cards!(win_3_submarine, submarine(3));

decl_dont_win_cards_colors!(dont_win_pink, PINK);
decl_dont_win_cards_colors!(dont_win_submarine, SUBMARINE);
decl_dont_win_cards_colors!(dont_win_green, GREEN);
decl_dont_win_cards_colors!(dont_win_yellow, YELLOW);
decl_dont_win_cards_colors!(dont_win_pink_blue, PINK, BLUE);
decl_dont_win_cards_colors!(dont_win_yellow_green, YELLOW, GREEN);

decl_dont_win_cards_values!(dont_win_8_9, 8, 9);
decl_dont_win_cards_values!(dont_win_9, 9);
decl_dont_win_cards_values!(dont_win_5, 5);
decl_dont_win_cards_values!(dont_win_1, 1);
decl_dont_win_cards_values!(dont_win_1_2_3, 1, 2, 3);

decl_dont_open_with!(dont_open_with_pink_yellow_blue, PINK, YELLOW, BLUE);
decl_dont_open_with!(dont_open_with_pink_green, PINK, GREEN);

decl_win_trick_with!(win_trick_with_6, 6);
decl_win_trick_with!(win_trick_with_5, 5);
decl_win_trick_with!(win_trick_with_3, 3);
decl_win_trick_with!(win_trick_containing_5_with_7, 7, 5);
decl_win_trick_with!(win_trick_containing_8_with_4, 4, 8);
decl_win_trick_with!(win_trick_containing_6_with_6, 6, 6);
decl_win_trick_with!(win_trick_with_2, 2);

decl_win_nb_tricks_compared_captain!(win_more_tricks_than_captain, std::cmp::Ordering::Greater);
decl_win_nb_tricks_compared_captain!(win_same_nb_tricks_that_captain, std::cmp::Ordering::Equal);
decl_win_nb_tricks_compared_captain!(win_less_tricks_than_captain, std::cmp::Ordering::Less);

use win_trick_with_pred::TaskWinTrickWithPred;
// TODO

decl_dont_win_first_tricks!(dont_win_first_three_tricks, 3);
decl_dont_win_first_tricks!(dont_win_first_four_tricks, 4);
decl_dont_win_first_tricks!(dont_win_first_five_tricks, 5);
pub fn dont_win_any_tricks() -> dont_win_tricks::TaskDontWinTricks {
    dont_win_tricks::TaskDontWinTricks::new_any()
}

decl_win_tricks!(win_first_trick, [0], false, false);
decl_win_tricks!(win_first_trick_only, [0], false, true);
decl_win_tricks!(win_last_trick, [], true, false);
decl_win_tricks!(win_last_trick_only, [], true, true);
decl_win_tricks!(win_first_and_last_trick, [0], true, false);
decl_win_tricks!(win_first_two_tricks, [0, 1], false, false);
decl_win_tricks!(win_first_three_tricks, [0, 1, 2], false, false);

decl_win_nb_tricks!(win_exactly_one_trick, 1);
decl_win_nb_tricks!(win_exactly_two_tricks, 2);
decl_win_nb_tricks!(win_exactly_four_tricks, 4);
// TODO WIN X TRICKS

decl_win_cards_amount_color!(win_exactly_1_pink_1_green, true, (PINK, 1), (GREEN, 1));
decl_win_cards_amount_color!(win_at_least_7_yellows, false, (YELLOW, 7));
decl_win_cards_amount_color!(win_at_least_5_pinks, false, (PINK, 5));
decl_win_cards_amount_color!(win_exactly_2_greens, true, (GREEN, 2));
decl_win_cards_amount_color!(win_exactly_2_blues, true, (BLUE, 2));
decl_win_cards_amount_color!(win_exactly_1_pink, true, (PINK, 1));
decl_win_cards_amount_color!(
    win_at_least_one_each_color,
    false,
    (PINK, 1),
    (GREEN, 1),
    (BLUE, 1),
    (YELLOW, 1)
);
decl_win_cards_amount_color!(win_exactly_1_submarine, true, (SUBMARINE, 1));
decl_win_cards_amount_color!(win_exactly_2_submarines, true, (SUBMARINE, 2));
decl_win_cards_amount_color!(win_exactly_3_submarines, true, (SUBMARINE, 3));

decl_win_cards_amount_number!(win_at_least_three_5s, false, (5, 3));
decl_win_cards_amount_number!(win_at_least_three_9s, false, (9, 3));
decl_win_cards_amount_number!(win_at_least_two_7s, false, (7, 2));
decl_win_cards_amount_number!(win_exactly_three_6s, true, (6, 3));
decl_win_cards_amount_number!(win_exactly_two_9s, true, (9, 2));

decl_win_consecutive_tricks!(win_two_consecutive_tricks, 2, false);
decl_win_consecutive_tricks!(win_three_consecutive_tricks, 3, false);
decl_win_consecutive_tricks!(win_exactly_three_consecutive_tricks, 3, true);
decl_win_consecutive_tricks!(win_exactly_two_consecutive_tricks, 2, true);

use dont_win_consecutive_tricks::TaskDontWinConsecutiveTricks;
pub fn dont_win_consecutive_tricks() -> TaskDontWinConsecutiveTricks {
    TaskDontWinConsecutiveTricks::new()
}

decl_win_specific_submarine!(win_submarine_only_1, 1);
decl_win_specific_submarine!(win_submarine_only_2, 2);

use win_all_cards_color::TaskWinAllCardsColor;
pub fn win_all_cards_color() -> TaskWinAllCardsColor {
    TaskWinAllCardsColor::new()
}

decl_win_more_cards_color!(win_same_amount_pink_and_yellow, pink, yellow, true);
decl_win_more_cards_color!(win_more_yellow_than_blue, yellow, blue, false);
decl_win_more_cards_color!(win_more_pink_than_green, pink, green, false);

decl_win_more_tricks!(win_more_tricks_than_everyone_else, false, false);
decl_win_more_tricks!(win_more_tricks_than_everyone_else_together, true, false);
decl_win_more_tricks!(win_fewer_tricks_than_everyone_else, false, true);
