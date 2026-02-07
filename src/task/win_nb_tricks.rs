use crate::task::{BaseTask, TaskDifficulty, TaskStatus};

#[derive(Debug, Clone)]
pub struct TaskWinNbTricks {
    difficulty: Option<TaskDifficulty>,
    n: usize,
}

impl TaskWinNbTricks {
    pub fn new(difficulty: Option<TaskDifficulty>, n: usize) -> Self {
        Self { difficulty, n }
    }
}

impl BaseTask for TaskWinNbTricks {
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

    impl_difficulty!();
}
