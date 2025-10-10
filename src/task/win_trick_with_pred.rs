use std::{collections::HashSet, fmt::Debug};

use crate::{
    card::Card,
    player,
    task::{Task, TaskStatus},
    trick::Trick,
};

pub struct TaskWinTrickWithPred {
    name: String,
    pred: Box<dyn Fn(&Trick) -> bool>,
}

/// We cannot derive Debug so we must implement it manually.
impl Debug for TaskWinTrickWithPred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TaskWinTrickWithPred<name=\"{}\">", self.name)
    }
}

impl TaskWinTrickWithPred {
    pub fn new_even() -> Self {
        Self {
            name: "even".to_string(),
            pred: Box::new(|t| t.iter().all(|c| c.val() % 2 == 0)),
        }
    }

    pub fn new_odd() -> Self {
        Self {
            name: "odd".to_string(),
            pred: Box::new(|t| t.iter().all(|c| c.val() % 2 == 1)),
        }
    }

    pub fn new_greater(value: usize) -> Self {
        Self {
            name: format!("greater than {}", value),
            pred: Box::new(move |t| t.iter().all(|c| c.val() > value)),
        }
    }

    pub fn new_lower(value: usize) -> Self {
        Self {
            name: format!("lower than {} (without submarines)", value),
            pred: Box::new(move |t| t.iter().all(|c| c.val() < value && !c.is_submarine())),
        }
    }

    pub fn new_total_greater(value: usize) -> Self {
        Self {
            name: format!("total value greather than {} (without submarines)", value),
            pred: Box::new(move |t| {
                t.iter().all(|c| !c.is_submarine())
                    && t.iter().map(|c| c.val()).sum::<usize>() > value
            }),
        }
    }

    pub fn new_total_lower(value: usize) -> Self {
        Self {
            name: format!("total value lower than {} (without submarines)", value),
            pred: Box::new(move |t| {
                t.iter().all(|c| !c.is_submarine())
                    && t.iter().map(|c| c.val()).sum::<usize>() < value
            }),
        }
    }

    pub fn new_total_in<I>(values: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let set: HashSet<usize> = values.into_iter().collect();
        Self {
            name: format!("total value in {:?}", set),
            pred: Box::new(move |t| set.contains(&t.iter().map(|c| c.val()).sum::<usize>())),
        }
    }

    pub fn new_same_nb_of_colors(color_1: fn(usize) -> Card, color_2: fn(usize) -> Card) -> Self {
        let d1 = color_1(1);
        let d2 = color_2(1);
        assert!(!d1.same_color(&d2), "colors must be different");
        Self {
            name: format!("same number of cards of two colors: {:?} and {:?}", d1, d2),
            pred: Box::new(move |t| {
                let n1 = t.iter().filter(|&c| c.same_color(&d1)).count();
                let n2 = t.iter().filter(|&c| c.same_color(&d2)).count();
                n1 > 0 && n1 == n2
            }),
        }
    }

    pub fn new_win_card_with_trump(card: Card) -> Self {
        assert!(card.is_valid());
        Self {
            name: format!("win card {:?} with a submarine", card),
            pred: Box::new(move |t| {
                t.iter().any(|c| *c == card) && t.iter().any(|c| c.is_submarine())
            }),
        }
    }

    pub fn new_win_card_last_trick(card: Card, n_players: usize) -> Self {
        assert!(card.is_valid());
        player::check_valid_n_players(n_players).unwrap();
        let last_trick = player::n_tricks_total(n_players) - 1;
        Self {
            name: format!("win card {:?} in the last trick ({})", card, last_trick),
            pred: Box::new(move |t| t.idx() == last_trick && t.iter().any(|c| *c == card)),
        }
    }
}

impl Task for TaskWinTrickWithPred {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        // If the solver ever takes too long, we can considering splitting this struct
        // and specializing some tasks to allow for early fail
        // (currently not supported because of the genericity):
        // - even/odd fail as soon as one player does not have an even/odd card anymore
        // - same_nb_colors fail as soon as one color has been fully played
        // - win_card_with_trump fail as soon as card already won or no trump left in hand
        // - win_card_last_trick fail as soon as card already won
        if state.get_player(ip).get_tricks().iter().any(&self.pred) {
            return TaskStatus::Done;
        }
        if state.game_is_over() {
            return TaskStatus::Failed;
        }
        TaskStatus::Unknown
    }
}
