#[macro_use]
extern crate lazy_static;

mod softball;
use softball::simulation::Simulator;
use softball::lineup::{self, Generator};

use std::clone::Clone;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::iter::FromIterator;
use std::io::{self, BufRead};
use std::path::Path;

use crossbeam;
use indicatif::{ProgressBar, ProgressStyle};
use num_cpus;
use rand::{SeedableRng, rngs};
use streaming_iterator::StreamingIterator;

static NUM_INNINGS: u32 = 7;
static NUM_GAMES: u32 = 10000;

fn main() -> Result<(), Box<Error>> {
    let all_players;
    let players_in_game: HashSet<String>;

    {
        let mut all_players_file = File::open(Path::new("softball.app.json"))?;
        let players_in_game_file = File::open(Path::new("players"))?;

        all_players = softball::app::load_export(&mut all_players_file)?;
        players_in_game = HashSet::from_iter(
            io::BufReader::new(players_in_game_file)
                .lines()
                .map(|l| l.unwrap()));
    }

    let players: Vec<_> = all_players.iter().filter(|p| players_in_game.contains(&p.name)).collect();

    let mut data = players.clone();
    let lineups = lineup::PermutationGenerator::new(&mut data);

    let simulator = softball::simulation::MonteCarlo::new();

    let pb = ProgressBar::new(lineups.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
                 .template("[{elapsed_precise}][{bar}]{pos}/{len} ({eta})")
                 .progress_chars("#>-"));

    println!("{:?}", run(&simulator, &lineups, pb));

    return Ok(());
}

fn run<'a, S>(simulator: &S, lineups: &softball::lineup::PermutationGenerator<'a>, pb: ProgressBar) -> (f64, Vec<softball::Player>)
    where S: softball::simulation::Simulator + Sync {
    let num_threads = num_cpus::get();
    let num_lineups = lineups.len();
    let num_lineups_per_thread = num_lineups / num_threads;

    return crossbeam::scope(|s| {
        let mut workers = Vec::with_capacity(num_threads);
        for i in 0..num_threads {
            let local_pb = pb.clone();
            workers.push(s.spawn(move |_| {
                let players: Vec<softball::Player> = lineups.heap.get().iter().cloned().cloned().collect();
                let mut players: Vec<&softball::Player> = players.iter().collect();
                let mut lineups = softball::lineup::PermutationGenerator::new(&mut players)
                    .skip(i * num_lineups_per_thread);

                let mut rng = rngs::SmallRng::from_entropy();
                let mut best_runs = 0.0;
                let mut best_lineup: Vec<softball::Player> = Vec::new();

                for _ in 0..num_lineups_per_thread {
                    match lineups.next() {
                        Some(lineup) => {
                            let runs = simulator.simulate_games(&mut rng, lineup, NUM_INNINGS, NUM_GAMES);
                            local_pb.inc(1);
                            if runs > best_runs {
                                best_runs = runs;
                                best_lineup = lineup.iter().cloned().cloned().collect();
                            }
                        },
                        None => break,
                    }
                }

                return (best_runs, best_lineup);
            }));
        }

        let mut best_runs = 0.0;
        let mut best_lineup: Vec<softball::Player> = Vec::new();
        for worker in workers {
            let (runs, lineup) = worker.join().unwrap();
            if runs > best_runs {
                best_runs = runs;
                best_lineup = lineup;
            }
        }

        return (best_runs, best_lineup);
    }).unwrap();
}
