use std::{collections::HashSet, vec};

fn main() {
    let mut count = 0;

    let data = include_str!("../input.txt")
        .lines()
        .map(|f| {
            f.split(',')
                .map(|s| {
                    s.split('-')
                        .map(|f| f.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .map(two_sized_vector_to_tuple)
        .for_each(|((a, b), (c, d))| {
            if (a <= c && b >= d) || (c <= a && d >= b) {
                println!("found");
                count += 1;
            } else {
                println!("not found");
            }
        });

    dbg!(data);
}

pub fn two_sized_vector_to_tuple(data: Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut value = Vec::new();
    data.iter()
        .for_each(|f| value.push((f.first(), f.get(1).unwrap())));
    value
}
