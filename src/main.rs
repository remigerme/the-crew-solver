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
    let mut p1 = Player::new(vec![Card::Submarine(4), Card::Blue(2)].into());
    let p2 = Player::new(vec![Card::Blue(1), Card::Pink(6)].into());

    p1.add_task(TaskWinCards::new(None, [Card::Blue(1), Card::Pink(6)]));

    let mut s = State::new(vec![p1, p2]);
    match s.play() {
        Ok(solution) => println!("Found a solution: {:?}", solution),
        Err(GameError::NoSolutionFound) => println!("Unfortunately this game is not feasible"),
        Err(e) => eprintln!("Error encountered: {}", e),
    }

    // Intermediate demo
    let mut state = State::new_random(4);
    state.get_mut_player(0).add_task(TaskWinCards::new(
        None,
        [
            Card::Blue(1),
            Card::Pink(6),
            Card::Green(9),
            Card::Yellow(4),
        ],
    ));
    state
        .get_mut_player(1)
        .add_task(TaskDontWinCards::new_from_colors(
            None,
            [Card::Blue as fn(usize) -> Card, Card::Green],
        ));
    state.get_mut_player(2).add_task(TaskDontOpenTrickWith::new(
        None,
        [Card::Blue as fn(usize) -> Card, Card::Pink],
    ));
    state
        .get_mut_player(3)
        .add_task(TaskDontWinCards::new_from_values(None, [5, 8]));
    state.get_mut_player(3).add_task(TaskDontWinCards::new(
        None,
        [Card::Yellow(1), Card::Yellow(2)],
    ));
    match state.play() {
        Ok(solution) => println!("Found a solution: {:?}", solution),
        Err(GameError::NoSolutionFound) => println!("Unfortunately this game is not feasible"),
        Err(e) => eprintln!("Error encountered: {}", e),
    }

    // First real-world usecase
    // let mut r = Player::new(
    //     [
    //         Card::Submarine(1),
    //         Card::Blue(8),
    //         Card::Blue(1),
    //         Card::Pink(1),
    //         Card::Yellow(8),
    //     ]
    //     .into(),
    // );
    // r.add_task(TaskDontOpenTrickWith::new(
    //     None,
    //     [Card::Green as fn(usize) -> Card, Card::Pink],
    // ));
    // let mut m = Player::new(
    //     [
    //         Card::Submarine(4),
    //         Card::Yellow(6),
    //         Card::Blue(6),
    //         Card::Pink(2),
    //         Card::Pink(5),
    //     ]
    //     .into(),
    // );
    // m.add_task(TaskWinTrickWithPred::new_odd(None));
    // m.add_task(TaskWinCardsAmountColor::new(
    //     None,
    //     false,
    //     [(Card::Pink as fn(usize) -> Card, 5)],
    // ));
    // m.add_trick(
    //     (
    //         3,
    //         0,
    //         [
    //             Card::Green(9),
    //             Card::Green(1),
    //             Card::Green(8),
    //             Card::Green(6),
    //         ],
    //     )
    //         .into(),
    // )
    // .unwrap();
    // m.add_trick(
    //     (
    //         4,
    //         0,
    //         [
    //             Card::Pink(7),
    //             Card::Pink(8),
    //             Card::Pink(9),
    //             Card::Submarine(2),
    //         ],
    //     )
    //         .into(),
    // )
    // .unwrap();
    // let mut f = Player::new(
    //     [
    //         Card::Submarine(3),
    //         Card::Green(7),
    //         Card::Green(3),
    //         Card::Pink(4),
    //         Card::Pink(6),
    //     ]
    //     .into(),
    // );
    // f.add_task(TaskWinCards::new(
    //     None,
    //     [
    //         Card::Blue(3),
    //         Card::Pink(3),
    //         Card::Green(3),
    //         Card::Yellow(3),
    //     ],
    // ));
    // f.add_trick(
    //     (
    //         0,
    //         0,
    //         [
    //             Card::Yellow(1),
    //             Card::Yellow(4),
    //             Card::Yellow(7),
    //             Card::Pink(3),
    //         ],
    //     )
    //         .into(),
    // )
    // .unwrap();
    // f.add_trick(
    //     (
    //         1,
    //         0,
    //         [
    //             Card::Yellow(9),
    //             Card::Yellow(2),
    //             Card::Yellow(5),
    //             Card::Yellow(3),
    //         ],
    //     )
    //         .into(),
    // )
    // .unwrap();
    // f.add_trick(
    //     (
    //         2,
    //         0,
    //         [Card::Blue(4), Card::Blue(2), Card::Blue(3), Card::Blue(7)],
    //     )
    //         .into(),
    // )
    // .unwrap();
    // let mut l = Player::new(
    //     [
    //         Card::Blue(9),
    //         Card::Blue(5),
    //         Card::Green(5),
    //         Card::Green(4),
    //         Card::Green(2),
    //     ]
    //     .into(),
    // );
    // l.add_task(TaskDontWinCards::new_from_colors(
    //     None,
    //     [Card::Submarine as fn(usize) -> Card],
    // ));
    // let mut state = State::new([m, l, r, f]);
    // state.current_trick = (5, 0, []).into(); // You need to make it public to run this example
    // match state.play() {
    //     Ok(solution) => println!("Found a solution: {:?}", solution),
    //     Err(GameError::NoSolutionFound) => println!("Unfortunately this game is not feasible"),
    //     Err(e) => eprintln!("Error encountered: {}", e),
    // }
}
