use std::collections::{BinaryHeap, HashMap};

use crate::{monkey::Monkey, round::round};

#[derive(Debug, Default, Clone)]
pub struct Day11 {
    pub monkeys: HashMap<usize, Monkey>,
}

impl Day11 {
    pub fn part1(&mut self) -> u32 {
        let mut part1 = self.clone();

        for _ in 0..20 {
            round(&mut part1);
        }
        let monkey_buisness = part1
            .monkeys
            .values()
            .map(|f| f.counts)
            .collect::<BinaryHeap<_>>();
        monkey_buisness.iter().take(2).product::<u32>()
    }
}
