use super::*;

use std::iter;

use permutohedron;
use permutohedron::Heap;
use streaming_iterator::StreamingIterator;

pub trait Lineup<'a> {
    type O: BattingOrder<'a>;

    fn players(&self) -> &'a [&'a Player];

    fn order(&self) -> Self::O;
}

pub trait BattingOrder<'a> {
    fn next(&mut self) -> &'a Player;
}

impl<'a> Lineup<'a> for &'a [&'a Player] {
    type O = CycleBattingOrder<'a>;

    fn players(&self) -> &'a [&'a Player] {
        return self;
    }

    fn order(&self) -> Self::O {
        return CycleBattingOrder {
            iter: self.iter().cycle(),
        }
    }
}

pub struct CycleBattingOrder<'a> {
    iter: iter::Cycle<std::slice::Iter<'a, &'a Player>>,
}

impl<'a> BattingOrder<'a> for CycleBattingOrder<'a> {
    fn next(&mut self) -> &'a Player {
        return self.iter.next().unwrap();
    }
}

pub trait Generator<'a>: StreamingIterator where Self::Item: 'a, &'a Self::Item: Lineup<'a>{
    fn new(players: &'a mut [&'a Player]) -> Self;

    fn len(players: &[&Player]) -> usize;
}

pub struct PermutationGenerator<'a> {
    heap: Heap<'a, [&'a Player], &'a Player>,
    has_next: bool,
}

impl<'a> StreamingIterator for PermutationGenerator<'a> {
    type Item = [&'a Player];

    fn advance(&mut self) {
        match self.heap.next_permutation() {
            Some(_) => self.has_next = true,
            None => self.has_next = false,
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        return match self.has_next {
            true => Some(self.heap.get()),
            false => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = <Self as Generator>::len(self.heap.get());
        return (len, Some(len));
    }
}

impl<'a> Generator<'a> for PermutationGenerator<'a> {
    fn new(players: &'a mut [&'a Player]) -> Self {
        return PermutationGenerator{
            heap: Heap::new(players),
            has_next: false,
        };
    }

    fn len(players: &[&Player]) -> usize {
        return permutohedron::factorial(players.len());
    }
}
