pub mod app;
pub mod lineup;
pub mod simulation;

use std::convert::TryFrom;

use enum_iterator::IntoEnumIterator;
use multiset::HashMultiSet;
use num_enum::TryFromPrimitive;
use rand;
use rand::distributions::Distribution;
use rand::distributions::weighted::alias_method::WeightedIndex;

#[derive(Clone, Debug)]
pub struct Player{
    pub id: String,
    pub name: String,
    pub gender: Gender,
    pub stats: Stats,
    dist: WeightedIndex<usize>,
}

impl Player {
    pub fn new(id: &str, name: &str, gender: Gender, stats: Stats) -> Player {
        let c = &stats.plate_appearances;
        let dist = WeightedIndex::new(Outcome::into_enum_iter().map(|o| c.count_of(&o)).collect()).unwrap();

        return Player{
            id: String::from(id),
            name: String::from(name),
            gender,
            stats,
            dist,
        }
    }

    pub fn hit<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Outcome {
        return Outcome::try_from(self.dist.sample(rng)).unwrap();
    }
}

#[derive(Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, IntoEnumIterator, TryFromPrimitive)]
#[repr(usize)]
pub enum Outcome {
    Out,
    Walk,
    Single,
    Double,
    Triple,
    Homerun,
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub plate_appearances: HashMultiSet<Outcome>,
}

impl Stats {
    pub fn new(plate_appearances: &[Outcome]) -> Stats {
        return Stats {
            plate_appearances: plate_appearances.iter().cloned().collect(),
        }
    }
}
