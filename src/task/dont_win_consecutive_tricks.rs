use crate::task::{Task, TaskStatus};

#[derive(Debug)]
pub struct TaskWinNoConsecutiveTricks {}

impl TaskWinNoConsecutiveTricks {
    pub fn new() -> Self {
        Self {}
    }
}

impl Task for TaskWinNoConsecutiveTricks {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let mut prev_trick = None;
        for trick in state.get_player(ip).get_tricks() {
            if let Some(prev) = prev_trick
                && prev == trick.idx() - 1
            {
                return TaskStatus::Failed;
            }
            prev_trick = Some(trick.idx());
        }

        if state.game_is_over() {
            return TaskStatus::Done;
        }

        TaskStatus::Unknown
    }
}
