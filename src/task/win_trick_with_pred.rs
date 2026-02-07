use std::{collections::HashSet, fmt::Debug, rc::Rc};

use crate::{
    card::Card,
    player,
    task::{BaseTask, TaskDifficulty, TaskStatus},
    trick::Trick,
};

#[derive(Clone)]
pub struct TaskWinTrickWithPred {
    difficulty: Option<TaskDifficulty>,
    name: String,
    pred: Rc<dyn Fn(&Trick) -> bool>,
}

/// We cannot derive Debug so we must implement it manually.
impl Debug for TaskWinTrickWithPred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TaskWinTrickWithPred<name=\"{}\">", self.name)
    }
}

impl TaskWinTrickWithPred {
    pub fn new_even(difficulty: Option<TaskDifficulty>) -> Self {
        Self {
            difficulty,
            name: "even".to_string(),
            pred: Rc::new(|t| t.iter().all(|c| c.val() % 2 == 0)),
        }
    }

    pub fn new_odd(difficulty: Option<TaskDifficulty>) -> Self {
        Self {
            difficulty,
            name: "odd".to_string(),
            pred: Rc::new(|t| t.iter().all(|c| c.val() % 2 == 1)),
        }
    }

    pub fn new_greater(difficulty: Option<TaskDifficulty>, value: usize) -> Self {
        Self {
            difficulty,
            name: format!("greater than {}", value),
            pred: Rc::new(move |t| t.iter().all(|c| c.val() > value)),
        }
    }

    pub fn new_lower(difficulty: Option<TaskDifficulty>, value: usize) -> Self {
        Self {
            difficulty,
            name: format!("lower than {} (without submarines)", value),
            pred: Rc::new(move |t| t.iter().all(|c| c.val() < value && !c.is_submarine())),
        }
    }

    pub fn new_total_greater(difficulty: Option<TaskDifficulty>, value: usize) -> Self {
        Self {
            difficulty,
            name: format!("total value greather than {} (without submarines)", value),
            pred: Rc::new(move |t| {
                t.iter().all(|c| !c.is_submarine())
                    && t.iter().map(|c| c.val()).sum::<usize>() > value
            }),
        }
    }

    pub fn new_total_lower(difficulty: Option<TaskDifficulty>, value: usize) -> Self {
        Self {
            difficulty,
            name: format!("total value lower than {} (without submarines)", value),
            pred: Rc::new(move |t| {
                t.iter().all(|c| !c.is_submarine())
                    && t.iter().map(|c| c.val()).sum::<usize>() < value
            }),
        }
    }

    pub fn new_total_in<I>(difficulty: Option<TaskDifficulty>, values: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let set: HashSet<usize> = values.into_iter().collect();
        Self {
            difficulty,
            name: format!("total value in {:?}", set),
            pred: Rc::new(move |t| set.contains(&t.iter().map(|c| c.val()).sum::<usize>())),
        }
    }

    pub fn new_same_nb_of_colors(
        difficulty: Option<TaskDifficulty>,
        color_1: fn(usize) -> Card,
        color_2: fn(usize) -> Card,
    ) -> Self {
        let d1 = color_1(1);
        let d2 = color_2(1);
        assert!(!d1.same_color(&d2), "colors must be different");
        Self {
            difficulty,
            name: format!("same number of cards of two colors: {:?} and {:?}", d1, d2),
            pred: Rc::new(move |t| {
                let n1 = t.iter().filter(|&c| c.same_color(&d1)).count();
                let n2 = t.iter().filter(|&c| c.same_color(&d2)).count();
                n1 > 0 && n1 == n2
            }),
        }
    }

    pub fn new_win_card_with_submarine(difficulty: Option<TaskDifficulty>, card: Card) -> Self {
        assert!(card.is_valid());
        Self {
            difficulty,
            name: format!("win card {:?} with a submarine", card),
            pred: Rc::new(move |t| {
                t.iter().any(|c| *c == card) && t.iter().any(|c| c.is_submarine())
            }),
        }
    }

    pub fn new_win_card_last_trick(
        difficulty: Option<TaskDifficulty>,
        card: Card,
        n_players: usize,
    ) -> Self {
        assert!(card.is_valid());
        player::check_valid_n_players(n_players).unwrap();
        let last_trick = player::n_tricks_total(n_players) - 1;
        Self {
            difficulty,
            name: format!("win card {:?} in the last trick ({})", card, last_trick),
            pred: Rc::new(move |t| t.idx() == last_trick && t.iter().any(|c| *c == card)),
        }
    }
}

impl BaseTask for TaskWinTrickWithPred {
    fn eval(&self, state: &crate::state::State, ip: usize) -> super::TaskStatus {
        // If the solver ever takes too long, we can considering splitting this struct
        // and specializing some tasks to allow for early fail
        // (currently not supported because of the genericity):
        // - even/odd fail as soon as one player does not have an even/odd card anymore
        // - same_nb_colors fail as soon as one color has been fully played
        // - win_card_with_trump fail as soon as card already won or no trump left in hand
        // - win_card_last_trick fail as soon as card already won
        if state
            .get_player(ip)
            .get_tricks()
            .iter()
            .any(self.pred.as_ref())
        {
            return TaskStatus::Done;
        }
        if state.game_is_over() {
            return TaskStatus::Failed;
        }
        TaskStatus::Unknown
    }

    impl_get_difficulty!();
}
