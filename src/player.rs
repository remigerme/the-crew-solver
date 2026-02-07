use std::ops::Deref;

use crate::{
    card::Card,
    hand::Hand,
    state::{GameError, State},
    task::{BaseTask, Task, TaskStatus},
    trick::Trick,
};

pub fn check_valid_n_players(n_players: usize) -> Result<(), String> {
    if n_players < 3 || n_players > 5 {
        Err(format!(
            "Invalid number of players: expected between 3 and 5 (inclusive), found {}",
            n_players
        ))
    } else {
        Ok(())
    }
}

pub fn n_tricks_total(n_players: usize) -> usize {
    check_valid_n_players(n_players).unwrap();
    if n_players == 3 {
        13
    } else if n_players == 4 {
        10
    } else if n_players == 5 {
        8
    } else {
        panic!("Should not happen - number of players should have been checked.")
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    hand: Hand,
    tricks: Vec<Trick>,
    tasks: Vec<Task>,
}

impl Player {
    pub fn new(hand: Hand) -> Self {
        Player {
            hand,
            tricks: Vec::new(),
            tasks: Vec::new(),
        }
    }

    pub fn get_hand(&self) -> &Hand {
        &self.hand
    }

    pub fn remove_card_from_hand(&mut self, card: &Card) -> Result<(), GameError> {
        let before = self.hand.len();
        self.hand.retain(|c| c != card);
        if self.hand.len() == before {
            Err(GameError::CardNotFound(*card, self.hand.deref().clone()))
        } else {
            Ok(())
        }
    }

    pub fn get_tricks(&self) -> &[Trick] {
        &self.tricks
    }

    pub fn add_trick(&mut self, trick: Trick) -> Result<(), GameError> {
        if let Some(t0) = self.tricks.get(0) {
            if t0.len() != trick.len() {
                return Err(GameError::InvalidTrickSize(t0.len(), trick.len()));
            }
        }
        if let Some(lt) = self.tricks.last() {
            if lt.idx() >= trick.idx() {
                return Err(GameError::NonIncreasingTrickIdx);
            }
        }
        self.tricks.push(trick);
        Ok(())
    }

    pub fn add_task<T>(&mut self, task: T)
    where
        T: Into<Task>,
    {
        self.tasks.push(task.into());
    }

    pub fn is_captain(&self) -> bool {
        let is_submarine_4 = |&c| c == Card::Submarine(4);
        self.hand.iter().any(is_submarine_4)
            || self.tricks.iter().any(|t| t.iter().any(is_submarine_4))
    }

    pub fn tasks_status(&self, ip: usize, state: &State) -> TaskStatus {
        let mut done = true;
        for task in &self.tasks {
            match task.eval(state, ip) {
                TaskStatus::Failed => return TaskStatus::Failed,
                TaskStatus::Unknown => done = false,
                TaskStatus::Done => (),
            }
        }

        if done {
            TaskStatus::Done
        } else {
            TaskStatus::Unknown
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_captain() {
        let captain = Player::new(vec![Card::Green(1), Card::Submarine(4)].into());
        assert!(captain.is_captain());

        let not_captain = Player::new(vec![Card::Yellow(4), Card::Green(1)].into());
        assert!(!not_captain.is_captain());
    }

    #[test]
    fn test_remove_card_from_hand() {
        let mut p = Player::new(vec![Card::Yellow(6), Card::Pink(2)].into());

        assert!(p.remove_card_from_hand(&Card::Yellow(6)).is_ok());
        assert!(p.remove_card_from_hand(&Card::Yellow(6)).is_err());
        assert!(p.remove_card_from_hand(&Card::Pink(4)).is_err());
        assert_eq!(*p.get_hand(), vec![Card::Pink(2)].into());
    }
}
