use crate::{
    card::Card,
    game::State,
    hand::Hand,
    task::{Task, TaskStatus},
    trick::Trick,
};

pub struct Player {
    hand: Hand,
    tricks: Vec<Trick>,
    tasks: Vec<Box<dyn Task>>,
}

impl Player {
    pub fn is_captain(&self) -> bool {
        let is_trump_4 = |&c| c == Card::Trump(4);
        self.hand.iter().any(is_trump_4) || self.tricks.iter().any(|t| t.iter().any(is_trump_4))
    }

    pub fn task_status(&self, ip: usize, state: &State) -> TaskStatus {
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
