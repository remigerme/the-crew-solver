use std::cmp::Ordering;

use crate::{
    state::State,
    task::{BaseTask, TaskDifficulty, TaskStatus},
};

#[derive(Debug, Clone)]
pub struct TaskWinNbTricksComparedCaptain {
    difficulty: Option<TaskDifficulty>,
    comp: Ordering,
}

impl TaskWinNbTricksComparedCaptain {
    pub fn new(difficulty: Option<TaskDifficulty>, comp: Ordering) -> Self {
        TaskWinNbTricksComparedCaptain { difficulty, comp }
    }

    fn can_have_more_tricks(state: &State, i1: usize, i2: usize, strict: bool) -> bool {
        let n1 = state.get_player(i1).get_tricks().len();
        let n2 = state.get_player(i2).get_tricks().len();
        let n_tricks_left = state.n_tricks_left();

        n1 + n_tricks_left > n2 || (n1 + n_tricks_left >= n2 && !strict)
    }
}

impl BaseTask for TaskWinNbTricksComparedCaptain {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        let i_captain = State::retrieve_captain(state.get_players()).unwrap();
        assert_ne!(ip, i_captain, "This task cannot be given to the captain.");

        if state.game_is_over() {
            let n_tricks_captain = state.get_player(i_captain).get_tricks().len();
            let n_tricks_player = state.get_player(ip).get_tricks().len();
            if n_tricks_player.cmp(&n_tricks_captain) == self.comp {
                return TaskStatus::Done;
            }
            return TaskStatus::Failed;
        }

        // At this point we could be lazy and return TaskStatus::Unknown
        // but we can investigate some cases failing the task:
        // - comp is Ordering::Less and the captain won't be able to get more tricks than the player
        // - comp is Ordering::Greater and the player won't be able to get more tricks than the captain
        match self.comp {
            Ordering::Less => {
                if !TaskWinNbTricksComparedCaptain::can_have_more_tricks(state, i_captain, ip, true)
                {
                    return TaskStatus::Failed;
                }
            }
            Ordering::Greater => {
                if !TaskWinNbTricksComparedCaptain::can_have_more_tricks(state, ip, i_captain, true)
                {
                    return TaskStatus::Failed;
                }
            }
            Ordering::Equal => {
                if !TaskWinNbTricksComparedCaptain::can_have_more_tricks(
                    state, i_captain, ip, false,
                ) || !TaskWinNbTricksComparedCaptain::can_have_more_tricks(
                    state, ip, i_captain, false,
                ) {
                    return TaskStatus::Failed;
                }
            }
        };

        TaskStatus::Unknown
    }

    impl_get_difficulty!();
}
