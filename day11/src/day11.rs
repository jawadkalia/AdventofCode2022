use std::collections::{BinaryHeap, HashMap};

use crate::{
    monkey::Monkey,
    round::{round, round2},
};

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

    pub fn part2(&mut self) -> u64 {
        let mut part2 = self.clone();

        for _ in 0..10000 {
            round2(&mut part2);
        }
        let mut monkey_buisness = part2
            .monkeys
            .values()
            .map(|f| f.counts)
            .collect::<BinaryHeap<_>>();
        let value = u64::from(
            monkey_buisness.pop().unwrap() as u64 * monkey_buisness.pop().unwrap() as u64,
        );
        value
    }
}
