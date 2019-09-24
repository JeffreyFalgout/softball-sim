use super::*;

use rand;

const MAX_RUNS_PER_INNING: u32 = 100;

pub trait Simulator {
    fn simulate_game<R: rand::Rng + ?Sized>(&self, rng: &mut R, lineup: &[&Player], num_innings: u32) -> u32;

    fn simulate_games<R: rand::Rng + ?Sized>(&self, rng: &mut R, lineup: &[&Player], num_innings: u32, num_games: u32) -> f64 {
        let mut runs = 0;
        for _ in 0..num_games {
            runs += self.simulate_game(rng, lineup, num_innings);
        }
        return runs as f64 / num_games as f64;
    }
}

pub struct MonteCarlo {
}

impl MonteCarlo {
    pub fn new() -> impl Simulator {
        return MonteCarlo{}
    }
}

impl Simulator for MonteCarlo {
    fn simulate_game<R: rand::Rng + ?Sized>(&self, rng: &mut R, lineup: &[&Player], num_innings: u32) -> u32 {
        let mut total_runs = 0;
        let mut batter = 0;

        for _ in 0..num_innings {
            let mut runs = 0;
            let mut bases = BaseState::new();
            let mut outs = 0;
            while outs < 3 && runs < MAX_RUNS_PER_INNING {
                let o = lineup[batter].hit(rng);
                match o {
                    Outcome::Out => outs += 1,
                    _ => runs += bases.advance_runners(o.num_bases()),
                }

                batter += 1;
                batter %= lineup.len()
            }

            total_runs += runs;
        }

        return total_runs;
    }
}

impl Outcome {
    fn num_bases(&self) -> u32{
        return match self {
            Outcome::Out => 0,
            Outcome::Walk | Outcome::Single => 1,
            Outcome::Double => 2,
            Outcome::Triple => 3,
            Outcome::Homerun => 4,
        }
    }
}

struct BaseState {
    state: u8,
}

lazy_static! {
    static ref STATE_TRANSITIONS: [(u8, u32); 32] = {
        let mut a = [(0,0); 32];
        for i in 0..32 {
            // 00 = single, 01 = double, 10 = triple, 11 = homerun.
            let hit = (0b11 & i) + 1;

            let mut first = (i >> 2 & 1) == 1;
            let mut second = (i >> 3 & 1) == 1;
            let mut third = (i >> 4 & 1) == 1;

            let mut num_runs = 0;
            for j in 0..hit {
                if third {
                    num_runs += 1;
                }

                third = second;
                second = first;
                first = j == 0;
            }

            let mut result: u8 = 0;
            result |= third as u8;
            result <<= 1;
            result |= second as u8;
            result <<= 1;
            result |= first as u8;

            a[i] = (result, num_runs);
        }
        return a;
    };
}

impl BaseState {
    fn new() -> BaseState {
        return BaseState{
            state: 0,
        };
    }

    fn advance_runners(&mut self, num_bases: u32) -> u32 {
        if num_bases == 0 {
            return 0;
        }

        let transition = (self.state << 2) | (num_bases as u8 - 1);
        let (next_state, num_runners) = STATE_TRANSITIONS[transition as usize];
        self.state = next_state;
        return num_runners;
    }
}
