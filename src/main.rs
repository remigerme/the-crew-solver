use crate::{
    card::Card,
    player::Player,
    state::{GameError, State},
    task::win_cards::TaskWinCards,
};

mod card;
mod hand;
mod player;
mod solver;
mod state;
mod task;
mod trick;

fn main() {
    let mut p1 = Player::new(vec![Card::Trump(4), Card::Blue(2)].into());
    let p2 = Player::new(vec![Card::Blue(1), Card::Red(6)].into());

    p1.add_task(TaskWinCards::new([Card::Blue(1), Card::Red(6)]));

    let mut s = State::new(vec![p1, p2]);
    match s.play() {
        Ok(solution) => println!("Found a solution: {:?}", solution),
        Err(GameError::NoSolutionFound) => println!("Unfortunately this game is not feasible"),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}
