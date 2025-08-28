use the_crew_solver::{
    card::Card,
    player::Player,
    state::{GameError, State},
    task::{
        dont_open_trick_with::TaskDontOpenTrickWith, dont_win_cards::TaskDontWinCards,
        win_cards::TaskWinCards,
    },
};

fn main() {
    // Minimal demo
    let mut p1 = Player::new(vec![Card::Trump(4), Card::Blue(2)].into());
    let p2 = Player::new(vec![Card::Blue(1), Card::Red(6)].into());

    p1.add_task(TaskWinCards::new([Card::Blue(1), Card::Red(6)]));

    let mut s = State::new(vec![p1, p2]);
    match s.play() {
        Ok(solution) => println!("Found a solution: {:?}", solution),
        Err(GameError::NoSolutionFound) => println!("Unfortunately this game is not feasible"),
        Err(e) => eprintln!("Error encountered: {}", e),
    }

    // Real world demo
    let mut state = State::new_random(4);
    state.get_mut_player(0).add_task(TaskWinCards::new([
        Card::Blue(1),
        Card::Red(6),
        Card::Green(9),
        Card::Yellow(4),
    ]));
    state
        .get_mut_player(1)
        .add_task(TaskDontWinCards::new_from_colors([
            Card::Blue as fn(usize) -> Card,
            Card::Green,
        ]));
    state
        .get_mut_player(2)
        .add_task(TaskDontOpenTrickWith::new([
            Card::Blue as fn(usize) -> Card,
            Card::Red,
        ]));
    state
        .get_mut_player(3)
        .add_task(TaskDontWinCards::new_from_values([5, 8]));
    state
        .get_mut_player(3)
        .add_task(TaskDontWinCards::new([Card::Yellow(1), Card::Yellow(2)]));
    match state.play() {
        Ok(solution) => println!("Found a solution: {:?}", solution),
        Err(GameError::NoSolutionFound) => println!("Unfortunately this game is not feasible"),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}
