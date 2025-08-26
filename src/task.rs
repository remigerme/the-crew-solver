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

/// Add your task module here.
pub mod win_cards;
