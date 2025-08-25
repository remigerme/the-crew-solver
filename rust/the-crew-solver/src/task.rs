use crate::game::State;

pub enum TaskStatus {
    Done,
    Unknown,
    Failed,
}

pub trait Task {
    fn eval(&self, state: &State, ip: usize) -> TaskStatus;
}
