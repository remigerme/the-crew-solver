use rand::seq::SliceRandom;
use thiserror::Error;

use crate::{
    card::{self, COLOR_RANGE, COLORS, Card, NB_CARDS, SUBMARINE_RANGE},
    player::{self, Player},
    task::TaskStatus,
    trick::Trick,
};

#[derive(Debug, Clone)]
pub struct State {
    players: Vec<Player>,
    current_trick: Trick,
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Captain was not found - is there a 4 of submarine in the game?")]
    MissingCaptain,
    #[error("Invalid trick size: expected {0}, got {1}.")]
    InvalidTrickSize(usize, usize),
    #[error("Non increasing trick index: have you called `.incr` to increment the trick index?")]
    NonIncreasingTrickIdx,
    #[error("Unfortunately this game is not feasible.")]
    NoSolutionFound,
    #[error("Card {0:?} was not found in {1:?}")]
    CardNotFound(Card, Vec<Card>),
}

impl State {
    pub fn retrieve_captain<'a, I>(
        players: I,
        current_trick: Option<&Trick>,
    ) -> Result<usize, GameError>
    where
        I: IntoIterator<Item = &'a Player>,
    {
        let players: Vec<&'a Player> = players.into_iter().collect();
        let n_players = players.len();
        for (i, p) in players.into_iter().enumerate() {
            if p.is_captain(current_trick.map(|ct| (ct, n_players, i))) {
                return Ok(i);
            }
        }
        Err(GameError::MissingCaptain)
    }

    pub fn new<I>(players: I) -> Self
    where
        I: IntoIterator<Item = Player>,
    {
        let players = players.into_iter().collect();
        let captain = State::retrieve_captain(&players, None).unwrap();
        State {
            players,
            current_trick: (0, captain, vec![]).into(),
        }
    }

    pub fn new_random(n_players: usize) -> Self {
        player::check_valid_n_players(n_players).unwrap();

        let mut cards = Vec::new();
        for i in COLOR_RANGE {
            for variant in COLORS {
                cards.push(variant(i));
            }
        }
        for i in SUBMARINE_RANGE {
            cards.push(Card::Submarine(i));
        }

        let mut rng = rand::rng();
        cards.shuffle(&mut rng);

        let mut players = vec![];
        let cards_per_player = NB_CARDS / n_players;
        for ip in 0..n_players {
            let lb = cards_per_player * ip;
            let ub = if ip == n_players - 1 {
                NB_CARDS
            } else {
                cards_per_player * (ip + 1)
            };
            let p = Player::new(cards[lb..ub].to_vec().into());
            players.push(p);
        }

        State::new(players)
    }

    pub fn first_player(&self) -> usize {
        self.current_trick.get_first_player()
    }

    pub fn n_players(&self) -> usize {
        self.players.len()
    }

    pub fn get_players(&self) -> &[Player] {
        &self.players
    }

    pub fn get_player(&self, i: usize) -> &Player {
        &self.players[i]
    }

    pub fn get_mut_player(&mut self, i: usize) -> &mut Player {
        &mut self.players[i]
    }

    pub fn get_current_player_idx(&self) -> usize {
        (self.first_player() + self.current_trick.len()) % self.n_players()
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.get_current_player_idx()]
    }

    pub fn get_mut_current_player(&mut self) -> &mut Player {
        let ip = self.get_current_player_idx();
        &mut self.players[ip]
    }

    pub fn get_current_trick(&self) -> &Trick {
        &self.current_trick
    }

    pub fn n_tricks_left(&self) -> usize {
        let cards_left: usize = self.players.iter().map(|p| p.get_hand().len()).sum();
        let cards_left_before_trick = cards_left + self.current_trick.len();
        cards_left_before_trick / self.n_players()
    }

    pub fn game_status(&self) -> TaskStatus {
        let mut done = true;
        for (i, p) in self.players.iter().enumerate() {
            match p.tasks_status(i, self) {
                TaskStatus::Failed => return TaskStatus::Failed,
                TaskStatus::Unknown => done = false,
                TaskStatus::Done => (),
            }
        }
        if done {
            TaskStatus::Done
        } else if self.game_is_over() {
            TaskStatus::Failed
        } else {
            TaskStatus::Unknown
        }
    }

    pub fn game_is_over(&self) -> bool {
        let mut cards_played = 0;
        for p in &self.players {
            cards_played += p.get_tricks().iter().map(|t| t.len()).sum::<usize>();
        }
        let leftover_cards = card::NB_CARDS % self.n_players();
        card::NB_CARDS - leftover_cards == cards_played
    }

    fn add_to_current_trick(&mut self, card: &Card) -> Result<(), GameError> {
        self.current_trick.push(*card);
        if self.current_trick.len() == self.n_players() {
            let ip = self.current_trick.winner();
            let trick = self.current_trick.clone();
            self.get_mut_player(ip).add_trick(trick)?;
            self.current_trick.clear();
            self.current_trick.set_first_player(ip);
            self.current_trick.incr();
        }
        Ok(())
    }

    pub fn play_card(&mut self, card: &Card) -> Result<(), GameError> {
        self.get_mut_current_player().remove_card_from_hand(card)?;
        self.add_to_current_trick(card)
    }
}
