use std::collections::HashSet;

use crate::direction::Direction;

#[derive(Debug, Default, Clone)]
pub struct Snake {
    segments: Vec<(i32, i32)>,
    pub visited: HashSet<(i32, i32)>,
}

impl Snake {
    pub fn new(len: usize) -> Self {
        Self {
            segments: vec![(0, 0); len],
            visited: HashSet::new(),
        }
    }

    pub fn do_move(&mut self, dir: &Direction) {
        let direction_movement_value = dir.movement_tuple();
        self.segments[0].1 += direction_movement_value.0;
        self.segments[0].0 += direction_movement_value.1;
        // dbg!(self.segments[0]);

        for i in 1..self.segments.len() {
            let rowdiff = self.segments[i - 1].0 - self.segments[i].0;
            let colldiff = self.segments[i - 1].1 - self.segments[i].1;

            if rowdiff == 0 && colldiff.abs() > 1 {
                self.segments[i].1 += colldiff.signum();
            } else if colldiff == 0 && rowdiff.abs() > 1 {
                self.segments[i].0 += rowdiff.signum();
            } else if rowdiff.abs() > 1 || colldiff.abs() > 1 {
                self.segments[i].0 += rowdiff.signum();
                self.segments[i].1 += colldiff.signum();
            }
        }
        // dbg!(self.tail);
        self.visited.insert(*self.segments.last().unwrap());
    }
}
