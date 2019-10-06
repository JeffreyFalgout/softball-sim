#[macro_use]
extern crate lazy_static;

mod softball;

use softball::lineup::Generator;
use softball::lineup::Lineup;

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
    let simulator = softball::simulation::MonteCarlo::new();

    println!("{:?}", run::<_, softball::lineup::PermutationGenerator>(&simulator, &players));

    return Ok(());
}

fn run<'a, S, G>(simulator: &S, players: &'a [&'a softball::Player]) -> (f64, Vec<softball::Player>)
    where S: softball::simulation::Simulator + Sync,
          G: 'a + softball::lineup::Generator<'a>,
          &'a <G as StreamingIterator>::Item: softball::lineup::Lineup<'a> {
    let num_threads = num_cpus::get();
    let num_lineups = G::len(players);
    let num_lineups_per_thread = num_lineups / num_threads;

    let pb = ProgressBar::new(num_lineups as u64);
    pb.set_style(ProgressStyle::default_bar()
                 .template("[{elapsed_precise}][{bar}]{pos}/{len} ({eta})")
                 .progress_chars("#>-"));


    return crossbeam::scope(|s| {
        let mut workers = Vec::with_capacity(num_threads);
        for i in 0..num_threads {
            let local_pb = pb.clone();
            workers.push(s.spawn(move |_| {
                let players: Vec<softball::Player> = players.iter().cloned().cloned().collect();
                let mut players: Vec<&softball::Player> = players.iter().collect();
                let mut lineups = softball::lineup::PermutationGenerator::new(&mut players).skip(i * num_lineups_per_thread);

                let mut rng = rngs::SmallRng::from_entropy();
                let mut best_runs = 0.0;
                let mut best_lineup: Vec<softball::Player> = Vec::new();

                for _ in 0..num_lineups_per_thread {
                    match lineups.next() {
                        Some(lineup) => {
                            let runs = simulator.simulate_games(&mut rng, &lineup, NUM_INNINGS, NUM_GAMES);
                            local_pb.inc(1);
                            if runs > best_runs {
                                best_runs = runs;
                                best_lineup = lineup.players().iter().cloned().cloned().collect();
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
