use std::fmt::Debug;

use enum_dispatch::enum_dispatch;

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

#[derive(Debug, Clone, Copy)]
pub struct TaskDifficulty(usize, usize, usize);

impl From<[usize; 3]> for TaskDifficulty {
    fn from(value: [usize; 3]) -> Self {
        TaskDifficulty(value[0], value[1], value[2])
    }
}

#[enum_dispatch]
pub trait BaseTask: Debug + Clone {
    fn eval(&self, state: &State, ip: usize) -> TaskStatus;

    fn difficulty(&self) -> Option<TaskDifficulty>;

    fn get_difficulty(&self, n_players: usize) -> Option<usize> {
        assert!(3 <= n_players && n_players <= 5);
        self.difficulty().map(|d| match n_players {
            3 => d.0,
            4 => d.1,
            5 => d.2,
            _ => panic!("invalid n_players"),
        })
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch(BaseTask)]
pub enum Task {
    DontOpenTrickWith(dont_open_trick_with::TaskDontOpenTrickWith),
    DontWinCards(dont_win_cards::TaskDontWinCards),
    DontWinConsecutiveTricks(dont_win_consecutive_tricks::TaskDontWinConsecutiveTricks),
    DontWinTricks(dont_win_tricks::TaskDontWinTricks),
    WinAllCardsColor(win_all_cards_color::TaskWinAllCardsColor),
    WinCardsAmountColor(win_cards_amount_color::TaskWinCardsAmountColor),
    WinCardsAmountNumber(win_cards_amount_number::TaskWinCardsAmountNumber),
    WinCards(win_cards::TaskWinCards),
    WinConsecutiveTricks(win_consecutive_tricks::TaskWinConsecutiveTricks),
    WinMoreCardsColor(win_more_cards_color::TaskWinMoreCardsColor),
    WinMoreTricks(win_more_tricks::TaskWinMoreTricks),
    WinNbTricksComparedCaptain(win_nb_tricks_compared_captain::TaskWinNbTricksComparedCaptain),
    WinNbTricks(win_nb_tricks::TaskWinNbTricks),
    WinSpecificSubmarine(win_specific_submarine::TaskWinSpecificSubmarine),
    WinTrickWithPred(win_trick_with_pred::TaskWinTrickWithPred),
    WinTrickWith(win_trick_with::TaskWinTrickWith),
    WinTricks(win_tricks::TaskWinTricks),
}

/// All tasks should have a field [`difficulty`] and the implementation of [`Task::get_difficulty`]
/// is straightforward using the following macro - but we need a macro cause we have no way to
/// know that a field difficulty will be available.
macro_rules! impl_difficulty {
    () => {
        fn difficulty(&self) -> Option<crate::task::TaskDifficulty> {
            self.difficulty
        }
    };
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
    ($name:ident, $diff:expr, $($card:expr),*) => {
        pub fn $name() -> win_cards::TaskWinCards {
            win_cards::TaskWinCards::new(Some($diff.into()), [$($card),*])
        }
    };
}

macro_rules! decl_dont_win_cards_colors {
    ($name:ident, $diff:expr, $($color:expr),*) => {
        pub fn $name() -> dont_win_cards::TaskDontWinCards {
            dont_win_cards::TaskDontWinCards::new_from_colors(Some($diff.into()), [$($color),*])
        }
    };
}

macro_rules! decl_dont_win_cards_values {
    ($name:ident, $diff:expr, $($value:expr),*) => {
        pub fn $name() -> dont_win_cards::TaskDontWinCards {
            dont_win_cards::TaskDontWinCards::new_from_values(Some($diff.into()), [$($value),*])
        }
    };
}

macro_rules! decl_dont_open_with {
    ($name:ident, $diff:expr, $($color:expr),*) => {
        pub fn $name() -> dont_open_trick_with::TaskDontOpenTrickWith {
            dont_open_trick_with::TaskDontOpenTrickWith::new(Some($diff.into()), [$($color),*])
        }
    };
}

macro_rules! decl_win_trick_with {
    ($name:ident, $diff:expr, $value:expr) => {
        pub fn $name() -> win_trick_with::TaskWinTrickWith {
            win_trick_with::TaskWinTrickWith::new(Some($diff.into()), $value, None)
        }
    };
    ($name:ident, $diff:expr, $value:expr, $must_win:expr) => {
        pub fn $name() -> win_trick_with::TaskWinTrickWith {
            win_trick_with::TaskWinTrickWith::new(Some($diff.into()), $value, Some($must_win))
        }
    };
}

macro_rules! decl_win_nb_tricks_compared_captain {
    ($name:ident, $diff:expr, $ord:expr) => {
        pub fn $name() -> win_nb_tricks_compared_captain::TaskWinNbTricksComparedCaptain {
            win_nb_tricks_compared_captain::TaskWinNbTricksComparedCaptain::new(
                Some($diff.into()),
                $ord,
            )
        }
    };
}

macro_rules! decl_dont_win_first_tricks {
    ($name:ident, $diff:expr, $n:expr) => {
        pub fn $name() -> dont_win_tricks::TaskDontWinTricks {
            dont_win_tricks::TaskDontWinTricks::new_n_first_tricks(Some($diff.into()), $n)
        }
    };
}

macro_rules! decl_win_tricks {
    ($name:ident, $diff:expr, $idx: expr, $last:expr, $strict:expr) => {
        pub fn $name() -> win_tricks::TaskWinTricks {
            win_tricks::TaskWinTricks::new(Some($diff.into()), $idx, $last, $strict)
        }
    };
}
macro_rules! decl_win_nb_tricks {
    ($name:ident, $diff:expr, $n:expr) => {
        pub fn $name() -> win_nb_tricks::TaskWinNbTricks {
            win_nb_tricks::TaskWinNbTricks::new(Some($diff.into()), $n)
        }
    };
}

macro_rules! decl_win_cards_amount_color {
    ($name:ident, $diff:expr, $exact:expr, $($constr:expr),*) => {
        pub fn $name() -> win_cards_amount_color::TaskWinCardsAmountColor {
            win_cards_amount_color::TaskWinCardsAmountColor::new(Some($diff.into()), $exact, [$($constr),*])
        }
    }
}

macro_rules! decl_win_cards_amount_number {
    ($name:ident, $diff:expr, $exact:expr, $($constr:expr),*) => {
        pub fn $name() -> win_cards_amount_number::TaskWinCardsAmountNumber {
            win_cards_amount_number::TaskWinCardsAmountNumber::new(Some($diff.into()), $exact, [$($constr),*])
        }
    }
}

macro_rules! decl_win_consecutive_tricks {
    ($name:ident, $diff:expr, $amount:expr, $exact:expr) => {
        pub fn $name() -> win_consecutive_tricks::TaskWinConsecutiveTricks {
            win_consecutive_tricks::TaskWinConsecutiveTricks::new(
                Some($diff.into()),
                $amount,
                $exact,
            )
        }
    };
}

macro_rules! decl_win_specific_submarine {
    ($name:ident, $diff:expr, $v:expr) => {
        pub fn $name() -> win_specific_submarine::TaskWinSpecificSubmarine {
            win_specific_submarine::TaskWinSpecificSubmarine::new(Some($diff.into()), $v)
        }
    };
}

macro_rules! decl_win_more_cards_color {
    ($name:ident, $diff:expr, $c1:expr, $c2:expr, $eq:expr) => {
        pub fn $name() -> win_more_cards_color::TaskWinMoreCardsColor {
            win_more_cards_color::TaskWinMoreCardsColor::new(Some($diff.into()), $c1, $c2, $eq)
        }
    };
}

macro_rules! decl_win_more_tricks {
    ($name:ident, $diff:expr, $together:expr, $fewer:expr) => {
        pub fn $name() -> win_more_tricks::TaskWinMoreTricks {
            win_more_tricks::TaskWinMoreTricks::new(Some($diff.into()), $together, $fewer)
        }
    };
}

decl_win_cards!(win_pink_1, [1, 1, 1], pink(1));
decl_win_cards!(win_yellow_1, [1, 1, 1], yellow(1));
decl_win_cards!(win_blue_4, [1, 1, 1], blue(4));
decl_win_cards!(win_green_6, [1, 1, 1], green(6));
decl_win_cards!(win_all_3s, [3, 4, 5], pink(3), blue(3), green(3), yellow(3));
decl_win_cards!(win_all_9s, [4, 5, 6], pink(9), blue(9), green(9), yellow(9));
decl_win_cards!(win_blue_1_2_3, [2, 3, 3], blue(1), blue(2), blue(3));
decl_win_cards!(win_blue_6_yellow_7, [2, 2, 3], blue(6), yellow(7));
decl_win_cards!(win_pink_5_yellow_6, [2, 2, 3], pink(5), yellow(6));
decl_win_cards!(win_green_5_blue_8, [2, 2, 3], green(5), blue(8));
decl_win_cards!(win_blue_5_pink_8, [2, 2, 3], blue(5), pink(8));
decl_win_cards!(win_pink_9_yellow_8, [2, 2, 3], pink(9), yellow(8));
decl_win_cards!(win_pink_1_green_7, [2, 2, 2], pink(1), green(7));
decl_win_cards!(win_yellow_9_blue_7, [2, 3, 3], yellow(9), blue(7));
decl_win_cards!(
    win_green_3_yellow_4_5,
    [3, 4, 4],
    green(3),
    yellow(4),
    yellow(5)
);
decl_win_cards!(win_3_submarine, [1, 1, 1], submarine(3));

decl_dont_win_cards_colors!(dont_win_pink, [2, 2, 2], PINK);
use dont_win_cards::TaskDontWinCards;
pub fn dont_win_submarine() -> TaskDontWinCards {
    TaskDontWinCards::new(
        Some([1, 1, 1].into()),
        [submarine(1), submarine(2), submarine(3), submarine(4)],
    )
}
decl_dont_win_cards_colors!(dont_win_green, [2, 2, 2], GREEN);
decl_dont_win_cards_colors!(dont_win_yellow, [2, 2, 2], YELLOW);
decl_dont_win_cards_colors!(dont_win_pink_blue, [3, 3, 3], PINK, BLUE);
decl_dont_win_cards_colors!(dont_win_yellow_green, [3, 3, 3], YELLOW, GREEN);

decl_dont_win_cards_values!(dont_win_8_9, [3, 3, 2], 8, 9);
decl_dont_win_cards_values!(dont_win_9, [1, 1, 1], 9);
decl_dont_win_cards_values!(dont_win_5, [1, 2, 2], 5);
decl_dont_win_cards_values!(dont_win_1, [2, 2, 2], 1);
decl_dont_win_cards_values!(dont_win_1_2_3, [3, 3, 3], 1, 2, 3);

decl_dont_open_with!(
    dont_open_with_pink_yellow_blue,
    [4, 3, 3],
    PINK,
    YELLOW,
    BLUE
);
decl_dont_open_with!(dont_open_with_pink_green, [2, 1, 1], PINK, GREEN);

decl_win_trick_with!(win_trick_with_6, [2, 3, 3], 6);
decl_win_trick_with!(win_trick_with_5, [2, 3, 4], 5);
decl_win_trick_with!(win_trick_with_3, [3, 4, 5], 3);
decl_win_trick_with!(win_trick_containing_5_with_7, [1, 2, 2], 7, 5);
decl_win_trick_with!(win_trick_containing_8_with_4, [3, 4, 5], 4, 8);
decl_win_trick_with!(win_trick_containing_6_with_6, [2, 3, 4], 6, 6);
decl_win_trick_with!(win_trick_with_2, [3, 4, 5], 2);

decl_win_nb_tricks_compared_captain!(
    win_more_tricks_than_captain,
    [2, 2, 3],
    std::cmp::Ordering::Greater
);
decl_win_nb_tricks_compared_captain!(
    win_same_nb_tricks_that_captain,
    [4, 3, 3],
    std::cmp::Ordering::Equal
);
decl_win_nb_tricks_compared_captain!(
    win_less_tricks_than_captain,
    [2, 2, 2],
    std::cmp::Ordering::Less
);

use win_trick_with_pred::TaskWinTrickWithPred;
pub fn win_trick_with_all_cards_lower_than_7() -> TaskWinTrickWithPred {
    TaskWinTrickWithPred::new_lower(Some([2, 3, 3].into()), 7)
}
pub fn win_trick_with_all_cards_greater_than_5() -> TaskWinTrickWithPred {
    TaskWinTrickWithPred::new_greater(Some([2, 3, 4].into()), 5)
}
pub fn win_trick_with_only_even_numbers() -> TaskWinTrickWithPred {
    TaskWinTrickWithPred::new_even(Some([2, 5, 6].into()))
}
pub fn win_trick_with_only_odd_numbers() -> TaskWinTrickWithPred {
    TaskWinTrickWithPred::new_odd(Some([2, 4, 5].into()))
}
pub fn win_trick_total_value_higher_than(n_players: usize) -> TaskWinTrickWithPred {
    let value = match n_players {
        3 => 23,
        4 => 28,
        5 => 31,
        _ => panic!("invalid n_players"),
    };
    TaskWinTrickWithPred::new_greater(Some([3, 3, 4].into()), value)
}
pub fn win_trick_total_value_lower_than(n_players: usize) -> TaskWinTrickWithPred {
    let value = match n_players {
        3 => 23,
        4 => 28,
        5 => 31,
        _ => panic!("invalid n_players"),
    };
    TaskWinTrickWithPred::new_lower(Some([3, 3, 4].into()), value)
}
pub fn win_trick_with_total_value_22_or_23() -> TaskWinTrickWithPred {
    TaskWinTrickWithPred::new_total_in(Some([3, 3, 4].into()), [22, 23])
}
pub fn win_trick_with_same_amount_green_and_yellow() -> TaskWinTrickWithPred {
    TaskWinTrickWithPred::new_same_nb_of_colors(Some([2, 3, 3].into()), green, yellow)
}
pub fn win_trick_with_same_amount_pink_and_blue() -> TaskWinTrickWithPred {
    TaskWinTrickWithPred::new_same_nb_of_colors(Some([2, 3, 3].into()), pink, blue)
}
pub fn win_pink_7_with_submarine() -> TaskWinTrickWithPred {
    TaskWinTrickWithPred::new_win_card_with_submarine(Some([3, 3, 3].into()), pink(7))
}
pub fn win_green_9_with_submarine() -> TaskWinTrickWithPred {
    TaskWinTrickWithPred::new_win_card_with_submarine(Some([3, 3, 3].into()), green(9))
}
pub fn win_green_2_in_last_trick(n_players: usize) -> TaskWinTrickWithPred {
    assert!(3 <= n_players && n_players <= 5);
    TaskWinTrickWithPred::new_win_card_last_trick(Some([3, 4, 5].into()), green(2), n_players)
}

decl_dont_win_first_tricks!(dont_win_first_three_tricks, [2, 3, 3], 3);
decl_dont_win_first_tricks!(dont_win_first_four_tricks, [1, 2, 3], 4);
decl_dont_win_first_tricks!(dont_win_first_five_tricks, [2, 3, 3], 5);
pub fn dont_win_any_tricks() -> dont_win_tricks::TaskDontWinTricks {
    dont_win_tricks::TaskDontWinTricks::new_any(Some([4, 3, 3].into()))
}

decl_win_tricks!(win_first_trick, [1, 1, 1], [0], false, false);
decl_win_tricks!(win_first_trick_only, [4, 3, 3], [0], false, true);
decl_win_tricks!(win_last_trick, [2, 3, 3], [], true, false);
decl_win_tricks!(win_last_trick_only, [4, 4, 4], [], true, true);
decl_win_tricks!(win_first_and_last_trick, [3, 4, 4], [0], true, false);
decl_win_tricks!(win_first_two_tricks, [1, 1, 2], [0, 1], false, false);
decl_win_tricks!(win_first_three_tricks, [2, 3, 4], [0, 1, 2], false, false);

decl_win_nb_tricks!(win_exactly_one_trick, [3, 2, 2], 1);
decl_win_nb_tricks!(win_exactly_two_tricks, [2, 2, 2], 2);
decl_win_nb_tricks!(win_exactly_four_tricks, [2, 3, 5], 4);
// TODO WIN X TRICKS

decl_win_cards_amount_color!(
    win_exactly_1_pink_1_green,
    [4, 4, 4],
    true,
    (PINK, 1),
    (GREEN, 1)
);
decl_win_cards_amount_color!(win_at_least_7_yellows, [3, 3, 3], false, (YELLOW, 7));
decl_win_cards_amount_color!(win_at_least_5_pinks, [2, 3, 3], false, (PINK, 5));
decl_win_cards_amount_color!(win_exactly_2_greens, [3, 4, 4], true, (GREEN, 2));
decl_win_cards_amount_color!(win_exactly_2_blues, [3, 4, 4], true, (BLUE, 2));
decl_win_cards_amount_color!(win_exactly_1_pink, [3, 3, 4], true, (PINK, 1));
decl_win_cards_amount_color!(
    win_at_least_one_each_color,
    [2, 3, 4],
    false,
    (PINK, 1),
    (GREEN, 1),
    (BLUE, 1),
    (YELLOW, 1)
);
decl_win_cards_amount_color!(win_exactly_1_submarine, [3, 3, 3], true, (SUBMARINE, 1));
decl_win_cards_amount_color!(win_exactly_2_submarines, [3, 3, 4], true, (SUBMARINE, 2));
decl_win_cards_amount_color!(win_exactly_3_submarines, [3, 4, 4], true, (SUBMARINE, 3));

decl_win_cards_amount_number!(win_at_least_three_5s, [3, 4, 5], false, (5, 3));
decl_win_cards_amount_number!(win_at_least_three_9s, [3, 4, 5], false, (9, 3));
decl_win_cards_amount_number!(win_at_least_two_7s, [2, 2, 2], false, (7, 2));
decl_win_cards_amount_number!(win_exactly_three_6s, [3, 4, 4], true, (6, 3));
decl_win_cards_amount_number!(win_exactly_two_9s, [2, 3, 3], true, (9, 2));

decl_win_consecutive_tricks!(win_two_consecutive_tricks, [1, 1, 1], 2, false);
decl_win_consecutive_tricks!(win_three_consecutive_tricks, [2, 3, 4], 3, false);
decl_win_consecutive_tricks!(win_exactly_three_consecutive_tricks, [3, 3, 4], 3, true);
decl_win_consecutive_tricks!(win_exactly_two_consecutive_tricks, [3, 3, 3], 2, true);

use dont_win_consecutive_tricks::TaskDontWinConsecutiveTricks;
pub fn dont_win_consecutive_tricks() -> TaskDontWinConsecutiveTricks {
    TaskDontWinConsecutiveTricks::new(Some([3, 2, 2].into()))
}

decl_win_specific_submarine!(win_submarine_only_1, [3, 3, 3], 1);
decl_win_specific_submarine!(win_submarine_only_2, [3, 3, 3], 2);

use win_all_cards_color::TaskWinAllCardsColor;
pub fn win_all_cards_color() -> TaskWinAllCardsColor {
    TaskWinAllCardsColor::new(Some([3, 4, 5].into()))
}

decl_win_more_cards_color!(
    win_same_amount_pink_and_yellow,
    [4, 4, 4],
    pink,
    yellow,
    true
);
decl_win_more_cards_color!(win_more_yellow_than_blue, [1, 1, 1], yellow, blue, false);
decl_win_more_cards_color!(win_more_pink_than_green, [1, 1, 1], pink, green, false);

decl_win_more_tricks!(win_more_tricks_than_everyone_else, [2, 3, 3], false, false);
decl_win_more_tricks!(
    win_more_tricks_than_everyone_else_together,
    [3, 4, 5],
    true,
    false
);
decl_win_more_tricks!(win_fewer_tricks_than_everyone_else, [2, 2, 3], false, true);
