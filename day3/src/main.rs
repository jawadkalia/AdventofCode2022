use std::collections::{HashMap, HashSet};

fn main() {
    let day3a = include_str!("../input.txt")
        .split("\n\n")
        .map(double_intersection)
        .next();

    println!("Day 3 a value is {}", day3a.unwrap());

    let day3b = triple_intersection();

    println!("Day 3 b value is {}", day3b);
}

fn double_intersection(s: &str) -> usize {
    lines_into_vector_or_vector_of_chars(s)
        .iter()
        .map(|s| {
            let (left, right) = s.split_at(s.len() / 2);
            let left: HashSet<&char> = HashSet::from_iter(left);
            let right = HashSet::from_iter(right);
            let summer = left
                .intersection(&right)
                .map(|f| get_priority(f))
                .sum::<usize>();
            summer
        })
        .sum::<usize>()
}

fn triple_intersection() -> usize {
    let chunks_created = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| {
            s.lines()
                .collect::<Vec<&str>>()
                .chunks(3)
                .map(|f| (f.first().unwrap(), f.get(1).unwrap(), f.get(2).unwrap()))
                .flat_map(|(a, b, c)| {
                    let a_hash: HashSet<char> = HashSet::from_iter(a.chars());
                    let b_hash: HashSet<char> = HashSet::from_iter(b.chars());
                    let c_hash: HashSet<char> = HashSet::from_iter(c.chars());

                    let intersection_1 = a_hash
                        .intersection(&b_hash)
                        .map(|f| f.to_owned())
                        .collect::<HashSet<char>>();
                    intersection_1
                        .intersection(&c_hash)
                        .collect::<HashSet<&char>>()
                        .iter()
                        .map(|f| get_priority(f.to_owned()))
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<usize>>()
                .iter()
                .sum::<usize>()
        })
        .sum::<usize>();

    chunks_created
}

fn lines_into_vector_or_vector_of_chars(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
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
