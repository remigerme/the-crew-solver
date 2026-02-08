use clap::Parser;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::Write;
use the_crew_solver::state::State;
use the_crew_solver::task::*;

#[derive(Parser, Debug)]
struct Args {
    n_samples: usize,
    n_players: usize,
    diff_tasks_min: usize,
    diff_tasks_max: usize,
    #[arg(short, long)]
    out: Option<String>,
}

fn build_tasks(n_players: usize) -> Vec<Task> {
    vec![
        // TaskWinCards
        win_pink_1().into(),
        win_yellow_1().into(),
        win_blue_4().into(),
        win_green_6().into(),
        win_all_3s().into(),
        win_all_9s().into(),
        win_blue_1_2_3().into(),
        win_blue_6_yellow_7().into(),
        win_pink_5_yellow_6().into(),
        win_green_5_blue_8().into(),
        win_blue_5_pink_8().into(),
        win_pink_9_yellow_8().into(),
        win_pink_1_green_7().into(),
        win_yellow_9_blue_7().into(),
        win_green_3_yellow_4_5().into(),
        win_3_submarine().into(),
        // TaskDontWinCards
        dont_win_pink().into(),
        dont_win_submarine().into(),
        dont_win_green().into(),
        dont_win_yellow().into(),
        dont_win_pink_blue().into(),
        dont_win_yellow_green().into(),
        dont_win_8_9().into(),
        dont_win_9().into(),
        dont_win_5().into(),
        dont_win_1().into(),
        dont_win_1_2_3().into(),
        // TaskDontOpenTrickWith
        dont_open_with_pink_yellow_blue().into(),
        dont_open_with_pink_green().into(),
        // TaskWinTrickWith
        win_trick_with_6().into(),
        win_trick_with_5().into(),
        win_trick_with_3().into(),
        win_trick_containing_5_with_7().into(),
        win_trick_containing_8_with_4().into(),
        win_trick_containing_6_with_6().into(),
        win_trick_with_2().into(),
        // TaskWinNbTricksComparedCaptain
        win_more_tricks_than_captain().into(),
        win_same_nb_tricks_that_captain().into(),
        win_less_tricks_than_captain().into(),
        // TaskWinTrickWithPred
        win_trick_with_all_cards_lower_than_7().into(),
        win_trick_with_all_cards_greater_than_5().into(),
        win_trick_with_only_even_numbers().into(),
        win_trick_with_only_odd_numbers().into(),
        win_trick_total_value_higher_than(n_players).into(),
        win_trick_total_value_lower_than(n_players).into(),
        win_trick_with_total_value_22_or_23().into(),
        win_trick_with_same_amount_green_and_yellow().into(),
        win_trick_with_same_amount_pink_and_blue().into(),
        win_pink_7_with_submarine().into(),
        win_green_9_with_submarine().into(),
        win_green_2_in_last_trick(n_players).into(),
        // TaskDontWinTricks
        dont_win_first_three_tricks().into(),
        dont_win_first_four_tricks().into(),
        dont_win_first_five_tricks().into(),
        dont_win_any_tricks().into(),
        // TaskWinTricks
        win_first_trick().into(),
        win_first_trick_only().into(),
        win_last_trick().into(),
        win_last_trick_only().into(),
        win_first_and_last_trick().into(),
        win_first_two_tricks().into(),
        win_first_three_tricks().into(),
        // TaskWinNbTricks
        win_exactly_one_trick().into(),
        win_exactly_two_tricks().into(),
        win_exactly_four_tricks().into(),
        // TaskWinCardsAmountColor
        win_exactly_1_pink_1_green().into(),
        win_at_least_7_yellows().into(),
        win_at_least_5_pinks().into(),
        win_exactly_2_greens().into(),
        win_exactly_2_blues().into(),
        win_exactly_1_pink().into(),
        win_at_least_one_each_color().into(),
        win_exactly_1_submarine().into(),
        win_exactly_2_submarines().into(),
        win_exactly_3_submarines().into(),
        // TaskWinCardsAmountNumber
        win_at_least_three_5s().into(),
        win_at_least_three_9s().into(),
        win_at_least_two_7s().into(),
        win_exactly_three_6s().into(),
        win_exactly_two_9s().into(),
        // TaskWinConsecutiveTricks
        win_two_consecutive_tricks().into(),
        win_three_consecutive_tricks().into(),
        win_exactly_three_consecutive_tricks().into(),
        win_exactly_two_consecutive_tricks().into(),
        // TaskDontWinConsecutiveTricks
        dont_win_consecutive_tricks().into(),
        // TaskWinSpecificSubmarine
        // win_submarine_only_1().into(),
        // win_submarine_only_2().into(),
        // TaskWinAllCardsColor
        win_all_cards_color().into(),
        // TaskWinMoreCardsColor
        win_same_amount_pink_and_yellow().into(),
        win_more_yellow_than_blue().into(),
        win_more_pink_than_green().into(),
        // TaskWinMoreTricks
        win_more_tricks_than_everyone_else().into(),
        win_more_tricks_than_everyone_else_together().into(),
        win_fewer_tricks_than_everyone_else().into(),
    ]
}

fn extract_first_valid(tasks: &Vec<Task>, diff_max: usize, n_players: usize) -> Option<Task> {
    for task in tasks {
        if task.get_difficulty(n_players).unwrap() <= diff_max {
            return Some(task.clone());
        }
    }
    None
}

fn assign_random_tasks(s: &mut State, diff_tasks: usize, tasks: &mut Vec<Task>) {
    let mut rng = rand::rng();
    let n_players = s.n_players();
    let mut ip = 0;
    let mut total_difficulty = 0;
    tasks.shuffle(&mut rng);

    while total_difficulty < diff_tasks
        && let Some(task) = extract_first_valid(&tasks, diff_tasks - total_difficulty, n_players)
    {
        total_difficulty += task.get_difficulty(n_players).unwrap();
        s.get_mut_player(ip).add_task(task);
        ip = (ip + 1) % n_players;
        tasks.shuffle(&mut rng);
    }
}

fn main() {
    let args = Args::parse();
    let n_samples = args.n_samples;
    let n_players = args.n_players;
    if n_players < 3 || 5 < n_players {
        panic!(
            "n_players should be between 3 and 5 (inclusive), got {}",
            n_players
        )
    }
    let diff_tasks_min = args.diff_tasks_min;
    let diff_tasks_max = args.diff_tasks_max;
    if diff_tasks_min > diff_tasks_max {
        panic!(
            "diff_tasks_min should be <= diff_tasks_max, got min={} and max={} instead",
            diff_tasks_min, diff_tasks_max
        )
    }

    let mut results = Vec::new();
    let mut tasks = build_tasks(n_players);
    for diff_task in diff_tasks_min..diff_tasks_max + 1 {
        for _ in 0..n_samples {
            let mut s = State::new_random(n_players);
            assign_random_tasks(&mut s, diff_task, &mut tasks);
            let start = std::time::Instant::now();
            let (n_done, n_failed, n_unknown) = s.play();
            results.push((n_done, n_failed, n_unknown, start.elapsed()));
        }
    }

    // Exporting to CSV
    let mut output: Box<dyn Write> = match &args.out {
        Some(path) => Box::new(File::create(path).expect("Failed to create output file")),
        None => Box::new(std::io::stdout()),
    };

    writeln!(output, "n_done,n_failed,n_unknown,duration").unwrap();
    for (n_done, n_failed, n_unknown, d) in results {
        writeln!(
            output,
            "{},{},{},{}",
            n_done,
            n_failed,
            n_unknown,
            d.as_secs_f32()
        )
        .unwrap();
    }
}
