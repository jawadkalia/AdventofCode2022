use array_tool::vec::Intersect;
use std::collections::HashMap;

fn main() {
    let day3a = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| double_intersection(s))
        .next();

    println!("Day 3 a value is {}", day3a.unwrap());

    let day3b = include_str!("../input.txt")
        .split("\n\n")
        .map(triple_intersection)
        .next();

    println!("Day 3 b value is {}", day3b.unwrap());
}

fn double_intersection(s: &str) -> usize {
    lines_into_vector_or_vector_of_chars(s)
        .iter()
        .map(|s| {
            let tuples_vecs = s.split_at(s.len() / 2);
            tuples_vecs
                .0
                .to_owned()
                // TODO: use HASHSET here
                .intersect(tuples_vecs.1.to_owned())
                .iter()
                .map(get_priority)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn triple_intersection(s: &str) -> usize {
    lines_into_vector_or_vector_of_chars(s)
        .chunks(3)
        .map(|f| (f.get(0).unwrap(), f.get(1).unwrap(), f.get(2).unwrap()))
        .map(|f| {
            f.0.to_vec()
                .intersect(f.1.to_vec())
                .intersect(f.2.to_vec())
                .to_vec()
        })
        .map(|f| f.iter().map(get_priority).sum::<usize>())
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>()
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
