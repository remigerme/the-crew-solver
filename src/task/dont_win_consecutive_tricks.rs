use crate::task::{BaseTask, TaskDifficulty, TaskStatus};

#[derive(Debug, Clone)]
pub struct TaskDontWinConsecutiveTricks {
    difficulty: Option<TaskDifficulty>,
}

impl TaskDontWinConsecutiveTricks {
    pub fn new(difficulty: Option<TaskDifficulty>) -> Self {
        Self { difficulty }
    }
}

impl BaseTask for TaskDontWinConsecutiveTricks {
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

    impl_difficulty!();
}
