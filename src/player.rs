use std::rc::Rc;

use crate::{
    card::Card,
    hand::Hand,
    state::{GameError, State},
    task::{Task, TaskStatus},
    trick::Trick,
};

#[derive(Debug, Clone)]
pub struct Player {
    hand: Hand,
    tricks: Vec<Trick>,
    tasks: Vec<Rc<dyn Task>>,
}

impl Player {
    pub fn new(hand: Hand) -> Self {
        Player {
            hand,
            tricks: Vec::new(),
            tasks: Vec::new(),
        }
    }

    pub fn add_trick(&mut self, trick: Trick) -> Result<(), GameError> {
        if let Some(t0) = self.tricks.get(0) {
            if t0.len() != trick.len() {
                return Err(GameError::InvalidTrickSize(t0.len(), trick.len()));
            }
        }
        self.tricks.push(trick);
        Ok(())
    }

    pub fn add_task<T: Task + 'static>(&mut self, task: T) {
        self.tasks.push(Rc::new(task));
    }

    pub fn get_tricks(&self) -> &[Trick] {
        &self.tricks
    }

    pub fn is_captain(&self) -> bool {
        let is_trump_4 = |&c| c == Card::Trump(4);
        self.hand.iter().any(is_trump_4) || self.tricks.iter().any(|t| t.iter().any(is_trump_4))
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
