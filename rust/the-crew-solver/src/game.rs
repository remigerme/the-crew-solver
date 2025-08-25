use crate::{player::Player, task::TaskStatus, trick::Trick};

pub struct State {
    players: Vec<Player>,
    first_player: usize,
    current_trick: Trick,
}

pub enum GameError {
    MissingCaptain,
}

impl State {
    pub fn n_players(&self) -> usize {
        self.players.len()
    }

    pub fn current_player_idx(&self) -> usize {
        (self.first_player + self.current_trick.len()) % self.n_players()
    }

    pub fn retrieve_captain(&self) -> Result<usize, GameError> {
        for (i, p) in self.players.iter().enumerate() {
            if p.is_captain() {
                return Ok(i);
            }
        }
        Err(GameError::MissingCaptain)
    }

    pub fn game_status(&self) -> TaskStatus {
        let mut done = true;
        for (i, p) in self.players.iter().enumerate() {
            match p.task_status(i, self) {
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
