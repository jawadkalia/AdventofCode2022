fn main() {
    let mut day4a_count = 0;
    let mut day4b_count = 0;

    parse_input().iter().for_each(|((a, b), (c, d))| {
        if (a <= c && b >= d) || (c <= a && d >= b) {
            day4a_count += 1;
        } else {
        }
    });

    parse_input().iter().for_each(|((a, b), (c, d))| {
        if (a >= c && a <= d) || (b >= c && b <= d) || (a <= c && b >= c) || (a <= d && b >= d) {
            day4b_count += 1;
        }
    });

    println!("Day 4 a count answer is {}", day4a_count);
    println!("Day 4 b count answer is {}", day4b_count);
}

fn parse_input() -> Vec<((usize, usize), (usize, usize))> {
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
        .collect()
}

pub fn two_sized_vector_to_tuple(data: Vec<Vec<usize>>) -> ((usize, usize), (usize, usize)) {
    let mut value = ((0, 0), (0, 0));
    value.0 .0 = data[0][0];
    value.0 .1 = data[0][1];
    value.1 .0 = data[1][0];
    value.1 .1 = data[1][1];
    value
}
