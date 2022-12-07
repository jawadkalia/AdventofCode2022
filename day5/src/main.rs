use std::{char, collections::VecDeque};

fn main() {
    let word = parse_input();
    let mut ship = Ship::new();
    word.clone().iter().for_each(|f| {
        if f.first().unwrap() == "move" {
            let amount = f[1].parse::<usize>().unwrap();
            let from = f[3].parse::<usize>().unwrap() - 1;
            let to = f[5].parse::<usize>().unwrap() - 1;
            let _ = &ship.instructions.push(Move { from, amount, to });
        }
    });

    ship_input()
        .lines()
        .filter(|f| f.contains("["))
        .map(|f| {
            f.chars()
                .enumerate()
                .filter(|(i, c)| *c != ' ' && (i + 3) % 4 == 0)
                .map(|f| (((f.0 - 1) / 4) + 1, f.1))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<(usize, char)>>>()
        .iter()
        .for_each(|info| {
            info.iter().for_each(|(a, b)| {
                while ship.ship_crates.len() < a.to_owned() {
                    ship.ship_crates.push(VecDeque::new());
                }
                ship.ship_crates[*a - 1].push_front(*b);
            });
        });

    let mut ship2 = ship.clone();


    ship.instructions.iter().for_each(|f| {
        for _ in 0..f.amount {
            let ship_crate = ship.ship_crates[f.from].pop_back().unwrap();
            ship.ship_crates[f.to].push_back(ship_crate)
        }
    });

    let mut answer = "".to_string();
    for stack in &ship.ship_crates {
        answer.push(*stack.back().unwrap());
    }

    dbg!(answer);

    let mut answer2 = "".to_string();
    ship2.instructions.iter().for_each(|f|{
        let split_point = ship2.ship_crates[f.from].len() - f.amount;
        let mut removed = ship2.ship_crates[f.from].split_off(split_point);
        ship2.ship_crates[f.to].append(&mut removed);
    });
    for stack in &ship2.ship_crates {
        answer2.push(*stack.back().unwrap());
    }

    dbg!(answer2);

}

pub fn parse_input() -> Vec<Vec<String>> {
    include_str!("../input.txt")
        .lines()
        .to_owned()
        .map(|f| f.split(' ').map(|f| f.to_owned()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

pub fn ship_input() -> String {
    include_str!("../input.txt")
        .split("\n\n")
        .nth(0)
        .unwrap()
        .to_owned()
}

pub fn move_input() -> String {
    include_str!("../input.txt")
        .split("\n\n")
        .last()
        .unwrap()
        .to_owned()
}

#[derive(Debug, Default, Clone)]
pub struct Ship {
    ship_crates: Vec<VecDeque<char>>,
    instructions: Vec<Move>,
}

impl Ship {
    pub fn new() -> Ship {
        
        Ship::default()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

