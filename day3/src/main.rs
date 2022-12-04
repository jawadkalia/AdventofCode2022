use array_tool::vec::Intersect;
use std::collections::HashMap;

fn main() {
    let value = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|s| s.chars().collect::<Vec<char>>())
                .map(|s| {
                    let tuples_vecs = s.split_at(s.len() / 2);
                    tuples_vecs
                        .0
                        .to_owned()
                        .intersect(tuples_vecs.1.to_owned())
                        .iter()
                        .map(get_priority)
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .next();
    println!("Day 3 a value is {}", value.unwrap());
}

fn get_priority(character: &char) -> usize {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|f| (f.1, f.0 + 1))
        .collect::<HashMap<char, usize>>()
        .get_key_value(character)
        .unwrap()
        .1
        .to_owned()
}
