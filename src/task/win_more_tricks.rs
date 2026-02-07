use crate::task::{BaseTask, TaskDifficulty, TaskStatus};

#[derive(Debug, Clone)]
pub struct TaskWinMoreTricks {
    difficulty: Option<TaskDifficulty>,
    everyone_else_together: bool,
    fewer: bool,
}

impl TaskWinMoreTricks {
    pub fn new(
        difficulty: Option<TaskDifficulty>,
        everyone_else_together: bool,
        fewer: bool,
    ) -> Self {
        // The following configuration does not exist in the game.
        // We rely on this for the implementation of [`eval`].
        assert!(!(everyone_else_together && fewer));
        Self {
            difficulty,
            everyone_else_together,
            fewer,
        }
    }
}

impl BaseTask for TaskWinMoreTricks {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let n_tricks_left = state.n_tricks_left();
        let n_tricks_won = state.get_player(ip).get_tricks().len();

        let mut vec_n_tricks_others = Vec::new();
        for i in 0..state.n_players() {
            if i == ip {
                continue;
            }
            vec_n_tricks_others.push(state.get_player(i).get_tricks().len());
        }
        let n_tricks_others: usize = vec_n_tricks_others.iter().sum();

        if self.everyone_else_together {
            assert!(!self.fewer); // This configuration does not exist in the game.
            if n_tricks_won > n_tricks_others + n_tricks_left {
                return TaskStatus::Done;
            }

            if n_tricks_won + n_tricks_left <= n_tricks_others {
                return TaskStatus::Failed;
            }

            TaskStatus::Unknown
        } else {
            let mut done = true;
            for n in vec_n_tricks_others {
                let done_i = if self.fewer {
                    n_tricks_won + n_tricks_left < n
                } else {
                    n_tricks_won > n + n_tricks_left
                };
                done &= done_i;

                let failed = if self.fewer {
                    n_tricks_won >= n + n_tricks_left
                } else {
                    n_tricks_won + n_tricks_left <= n
                };
                if failed {
                    return TaskStatus::Failed;
                }
            }

            if done {
                return TaskStatus::Done;
            }

            TaskStatus::Unknown
        }
    }

    impl_difficulty!();
}
