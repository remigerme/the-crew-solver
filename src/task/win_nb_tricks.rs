use crate::task::{Task, TaskStatus};

#[derive(Debug)]
pub struct TaskWinNbTricks {
    n: usize,
}

impl TaskWinNbTricks {
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}

impl Task for TaskWinNbTricks {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let n_tricks = state.get_player(ip).get_tricks().len();

        if state.game_is_over() {
            if n_tricks == self.n {
                TaskStatus::Done
            } else {
                TaskStatus::Failed
            }
        } else {
            // If we won too many tricks, or there are not enough tricks anymore, task is failed.
            let left_tricks = state.n_tricks_left();
            if n_tricks > self.n || n_tricks + left_tricks < self.n {
                TaskStatus::Failed
            } else {
                TaskStatus::Unknown
            }
        }
    }
}
