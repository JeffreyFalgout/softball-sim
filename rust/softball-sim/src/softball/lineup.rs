use super::*;

use permutohedron;
use permutohedron::Heap;
use streaming_iterator::StreamingIterator;

pub trait Generator<'a>: StreamingIterator<Item = [&'a Player]> {
    fn len(&self) -> usize;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.len();
        return (l, Some(l));
    }
}

pub struct PermutationGenerator<'a> {
    pub heap: Heap<'a, [&'a Player], &'a Player>,
    has_next: bool,
}

impl<'a> PermutationGenerator<'a> {
    pub fn new(players: &'a mut [&'a Player]) -> PermutationGenerator<'a> {
        return PermutationGenerator{
            heap: Heap::new(players),
            has_next: false,
        };
    }
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
}

impl<'a> Generator<'a> for PermutationGenerator<'a> {
    fn len(&self) -> usize {
        return permutohedron::factorial(self.heap.get().len());
    }
}
