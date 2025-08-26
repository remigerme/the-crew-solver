use crate::{player::Player, task::TaskStatus, trick::Trick};

#[derive(Debug, Clone)]
pub struct State {
    players: Vec<Player>,
    first_player: usize,
    current_trick: Trick,
}

#[derive(Debug)]
pub enum GameError {
    MissingCaptain,
    InvalidTrickSize(usize, usize),
}

impl State {
    pub fn retrieve_captain(players: &Vec<Player>) -> Result<usize, GameError> {
        for (i, p) in players.iter().enumerate() {
            if p.is_captain() {
                return Ok(i);
            }
        }
        Err(GameError::MissingCaptain)
    }

    pub fn new(players: Vec<Player>) -> Self {
        let captain = State::retrieve_captain(&players).unwrap();
        State {
            players,
            first_player: captain,
            current_trick: (0, vec![]).into(),
        }
    }

    pub fn n_players(&self) -> usize {
        self.players.len()
    }

    pub fn get_player(&self, i: usize) -> &Player {
        &self.players[i]
    }

    pub fn current_player_idx(&self) -> usize {
        (self.first_player + self.current_trick.len()) % self.n_players()
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player_idx()]
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
