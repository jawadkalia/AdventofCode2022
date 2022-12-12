use std::collections::{HashMap, HashSet};

use day8::{
    directions::{DOWN, LEFT, RIGHT, UP},
    input,
};

fn main() {
    let lines = input::input();
    let mut value = Day8 {
        height: lines.len() as i32,
        width: lines[0].len() as i32,
        ..Default::default()
    };

    lines
        .clone()
        .iter()
        .for_each(|line| value.grid.push(line.chars().collect::<Vec<char>>()));

    println!("Day 8 part a answer {}", value.parta());
    println!("Day 8 part b answer {}", value.partb());
}

#[derive(Debug, Default)]
pub struct Day8 {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Day8 {
    fn parta(&mut self) -> usize {
        let mut total = HashSet::new();

        for (start, step, search) in [
            ((0, 0), RIGHT, DOWN),
            ((0, 0), DOWN, RIGHT),
            ((self.height - 1, self.width - 1), UP, LEFT),
            ((self.height - 1, self.width - 1), LEFT, UP),
        ] {
            let mut walk = start;

            while walk.0 >= 0 && walk.0 < self.height && walk.1 >= 0 && walk.1 < self.width {
                let (mut row, mut col) = walk;
                let mut tallest = self.grid[row as usize][col as usize];

                total.insert((row, col));

                // dbg!(total.clone());

                while tallest < '9' {
                    row += search.0;
                    col += search.1;
                    // dbg!(search.clone());

                    if row < 0 || row >= self.height || col < 0 || col >= self.width {
                        break;
                    }

                    let tree = self.grid[row as usize][col as usize];
                    if tree > tallest {
                        total.insert((row, col));
                        tallest = tree;
                        // dbg!(tallest.clone());
                    }
                }

                walk.0 += step.0;
                walk.1 += step.1;

                // dbg!(walk.clone());
            }
        }
        total.len()
    }

    fn partb(&mut self) -> i32 {
        let mut max_score = 0;

        for row in 1..self.height - 1 {
            for col in 1..self.width - 1 {
                let mut score = 1;
                for step in [UP, DOWN, LEFT, RIGHT] {
                    let mut walk = (row, col);
                    let height_tree_now = self.grid[row as usize][col as usize];

                    walk.0 += step.0;
                    walk.1 += step.1;

                    // dbg!(walk.clone());

                    let mut count = 0;
                    while walk.0 >= 0 && walk.0 < self.height && walk.1 >= 0 && walk.1 < self.width
                    {
                        count += 1;

                        if self.grid[walk.0 as usize][walk.1 as usize] >= height_tree_now {
                            break;
                        }

                        // dbg!(count.clone());

                        walk.0 += step.0;
                        walk.1 += step.1;
                    }

                    score *= count;
                }
                max_score = max_score.max(score);
                // dbg!(max_score.clone());
            }
        }

        max_score
    }
}
