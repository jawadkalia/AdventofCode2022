use std::collections::HashMap;

fn main() {
    let value = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
                .iter()
                .map(|s| println!("{:#?}", s))
                .next()
        })
        .next();

    dbg!(value);
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
