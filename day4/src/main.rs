use std::{collections::HashSet, vec};

fn main() {
    let mut count = 0;

    include_str!("../input.txt")
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
                count += 1;
            } else {
            }
        });

    // dbg!(data);
    dbg!(count);
}

pub fn two_sized_vector_to_tuple(data: Vec<Vec<usize>>) -> ((usize, usize), (usize, usize)) {
    let mut value = ((0, 0), (0, 0));
    value.0 .0 = data[0][0];
    value.0 .1 = data[0][1];
    value.1 .0 = data[1][0];
    value.1 .1 = data[1][1];
    // dbg!(&value);
    value
}
