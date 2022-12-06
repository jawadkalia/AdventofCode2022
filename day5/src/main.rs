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

    // ship.instructions.iter().for_each(|f| {
    //     for _ in 0..f.amount {
    //         let ship_crate = ship.ship_crates[f.from].pop_back().unwrap();
    //         ship.ship_crates[f.to].push_back(ship_crate)
    //     }
    // });

    // let mut answer = "".to_string();
    // for stack in &ship.ship_crates {
    //     answer.push(*stack.back().unwrap());
    // }

    let mut answer2 = "".to_string();
    ship.instructions.iter().for_each(|f|{
        let split_point = ship.ship_crates[f.from].len() - f.amount;
        let mut removed = ship.ship_crates[f.from].split_off(split_point);
        ship.ship_crates[f.to].append(&mut removed);
    });
    for stack in &ship.ship_crates {
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

#[derive(Debug, Default)]
pub struct Ship {
    ship_crates: Vec<VecDeque<char>>,
    instructions: Vec<Move>,
}

impl Ship {
    pub fn new() -> Ship {
        
        Ship::default()
    }
}

#[derive(Debug, Default)]
pub struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

// if f.1.contains('[') {
//     dbg!(f.0 + 1/ 9);
//     dbg!(f.1);
//    f.1.chars().enumerate().map(|f| {dbg!(&f); f}).filter(|(a, b)| *b != ' ' && (a + 3) % 4 == 0).for_each(|f| {
//     // dbg!(f.1);
//     // while ship.instructions.len() < stack + 1 {
//     //     ship.ship_crates.push(VecDeque::new())

//     // }
//     // ship.ship_crates[stack].push_front(f.1);
//    }
