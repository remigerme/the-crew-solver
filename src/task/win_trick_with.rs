use crate::{
    card::{COLOR_RANGE, Card},
    task::{Task, TaskStatus},
    trick::Trick,
};

#[derive(Debug)]
pub struct TaskWinTrickWith {
    win_with: usize,
    must_win: Option<usize>,
}

impl TaskWinTrickWith {
    pub fn new(win_with: usize, must_win: Option<usize>) -> Self {
        TaskWinTrickWith { win_with, must_win }
    }
}

impl Task for TaskWinTrickWith {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        // Checking if a trick already satisfies the task
        let n_players = state.n_players();
        let idx_in_trick = |t: &Trick| (ip + n_players - t.get_first_player()) % n_players;
        let won_with = |t: &Trick| t[idx_in_trick(t)].val() == self.win_with;
        let contains_additional_card = |t: &Trick| match self.must_win {
            None => true,
            Some(c) => {
                t.contains(&Card::Blue(c))
                    || t.contains(&Card::Green(c))
                    || t.contains(&Card::Red(c))
                    || t.contains(&Card::Yellow(c))
            }
        };
        let goal_trick = |t| won_with(t) && contains_additional_card(t);
        let p = state.get_player(ip);
        if p.get_tricks().iter().any(goal_trick) {
            return TaskStatus::Done;
        }

        // If player does not have a card of the required value, task is failed
        let mut missing_in_hand = true;
        for c in p.get_hand().iter() {
            if c.val() == self.win_with && !c.is_trump() {
                missing_in_hand = false;
            }
        }
        if missing_in_hand {
            return TaskStatus::Failed;
        }

        // If no card that should be won remains in the game, task is failed
        if let Some(target_val) = self.must_win {
            let mut missing_other_card = true;
            for i in 0..n_players {
                if i == ip {
                    continue;
                }

                for c in state.get_player(i).get_hand().iter() {
                    if c.val() == target_val && !c.is_trump() {
                        missing_other_card = false;
                    }
                }
            }

            if missing_other_card {
                return TaskStatus::Failed;
            }
        }

        TaskStatus::Unknown
    }
}
