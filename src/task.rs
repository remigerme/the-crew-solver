use std::fmt::Debug;

use crate::{
    card::{blue, green, pink, submarine, yellow},
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
pub mod dont_win_tricks;
pub mod win_cards;
pub mod win_cards_amount_color;
pub mod win_nb_tricks;
pub mod win_nb_tricks_compared_captain;
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
