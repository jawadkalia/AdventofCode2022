use std::collections::VecDeque;

use crate::operation::Operation;

#[derive(Debug, Default, Clone)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    pub operation: Operation,
    pub worry_test: u64,
    pub counts: u32,
    pub destination: (u32, u32),
}

impl Monkey {
    pub fn from_words(words: &Vec<String>) -> Self {
        let mut monkey = Monkey::default();
        words
            .iter()
            .map(|sentences| sentences.split(' ').collect::<Vec<_>>())
            .for_each(|words| match *words.first().unwrap() {
                "Monkey" => (),
                "Starting" => words[2..].iter().for_each(|f| {
                    let item = f.replace(',', "").parse::<u64>().unwrap();
                    // println!("pushing item {item}");
                    monkey.items.push_back(item);
                    // dbg!(monkey.clone().items);
                }),
                "Operation:" => match words[4] {
                    "+" => match words[5] == "old" {
                        true => monkey.operation = Operation::AddSelf,
                        false => monkey.operation = Operation::Add(words[5].parse().unwrap()),
                    },
                    "*" => match words[5] == "old" {
                        true => monkey.operation = Operation::MultiplySelf,
                        false => monkey.operation = Operation::Multiply(words[5].parse().unwrap()),
                    },
                    _ => unreachable!(),
                },
                "Test:" => {
                    monkey.worry_test = words[3].parse().unwrap();
                }
                "If" => match words[1] {
                    "true:" => monkey.destination.0 = words[5].parse().unwrap(),
                    "false:" => monkey.destination.1 = words[5].parse().unwrap(),
                    _ => unreachable!(),
                },

                _ => (),
            });

        monkey
    }
}
