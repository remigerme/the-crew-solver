use std::fmt::Debug;

use crate::state::State;

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
pub mod win_nb_tricks_compared_captain;
pub mod win_trick_with;
pub mod win_trick_with_pred;
