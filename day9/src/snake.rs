use std::collections::HashSet;

use crate::direction::Direction;

#[derive(Debug, Default, Clone)]
pub struct Snake {
    head: (i32, i32),
    tail: (i32, i32),
    pub visited: HashSet<(i32, i32)>,
}

impl Snake {
    pub fn do_move(&mut self, dir: &Direction) {
        let direction_movement_value = dir.movement_tuple();
        self.head.0 += direction_movement_value.0;
        self.head.1 += direction_movement_value.1;
        // dbg!(self.head);

        let rowdiff = self.head.0 - self.tail.0;
        let colldiff = self.head.1 - self.tail.1;

        if rowdiff == 0 && colldiff.abs() > 1 {
            self.tail.1 += colldiff.signum();
        } else if colldiff == 0 && rowdiff.abs() > 1 {
            self.tail.0 += rowdiff.signum();
        } else if rowdiff.abs() > 1 || colldiff.abs() > 1 {
            self.tail.0 += rowdiff.signum();
            self.tail.1 += colldiff.signum();
        }
        // dbg!(self.tail);
        self.visited.insert(self.tail);
    }
}
