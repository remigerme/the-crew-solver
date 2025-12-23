use crate::{
    task::{Task, TaskStatus},
    trick::Trick,
};

#[derive(Debug)]
pub struct TaskWinConsecutiveTricks {
    amount: usize,
    exactly: bool,
}

impl TaskWinConsecutiveTricks {
    pub fn new(amount: usize, exactly: bool) -> Self {
        Self { amount, exactly }
    }

    pub fn new_two_consecutive_tricks() -> Self {
        Self::new(2, false)
    }

    pub fn new_exactly_two_consecutive_tricks() -> Self {
        Self::new(2, true)
    }

    pub fn new_three_consecutive_tricks() -> Self {
        Self::new(3, false)
    }

    pub fn new_exactly_three_consecutive_tricks() -> Self {
        Self::new(3, true)
    }
}

/// Returns (`gap`, `biggest_streak`, `current_streak`).
fn review_gameplay(tricks: &[Trick]) -> (bool, usize, usize) {
    let mut gap = false;
    let mut biggest_streak = 0;
    let mut current_streak = 0;

    let mut prev_trick_opt: Option<usize> = None;
    for trick in tricks {
        if let Some(prev_trick) = prev_trick_opt
            && prev_trick != trick.idx() - 1
        {
            current_streak = 0;
            gap = true;
        }

        current_streak += 1;
        biggest_streak = std::cmp::max(current_streak, biggest_streak);
        prev_trick_opt = Some(trick.idx());
    }

    (gap, biggest_streak, current_streak)
}

impl Task for TaskWinConsecutiveTricks {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let (gap, biggest_streak, current_streak) =
            review_gameplay(state.get_player(ip).get_tricks());

        if !self.exactly && biggest_streak >= self.amount {
            return TaskStatus::Done;
        }

        if self.exactly && state.game_is_over() && biggest_streak == self.amount && !gap {
            return TaskStatus::Done;
        }

        if self.exactly && (gap || biggest_streak > self.amount) {
            return TaskStatus::Failed;
        }

        let missing = self.amount - current_streak;
        if missing < state.n_tricks_left() {
            return TaskStatus::Failed;
        }

        TaskStatus::Unknown
    }
}
