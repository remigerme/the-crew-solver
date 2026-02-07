use the_crew_solver::state::State;

use clap::Parser;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
struct Args {
    n_samples: usize,
    n_players: usize,
    diff_tasks_min: usize,
    diff_tasks_max: usize,
    #[arg(short, long)]
    out: Option<String>,
}

fn assign_random_tasks(s: &mut State, diff_tasks: usize) {}

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
    for diff_task in diff_tasks_min..diff_tasks_max + 1 {
        for _ in 0..n_samples {
            let mut s = State::new_random(n_players);
            assign_random_tasks(&mut s, diff_task);
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
