pub fn input() -> Vec<Vec<String>> {
    let value = include_str!("../input.txt")
        .trim()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .chunks(7)
        .map(|f| f.to_vec())
        .collect::<Vec<Vec<String>>>();
    value
}

pub fn input_test() -> Vec<Vec<String>> {
    let value = include_str!("../test.txt")
        .trim()
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
        .chunks(7)
        .map(|f| f.to_vec())
        .collect::<Vec<Vec<String>>>();
    value
}

// pub fn input2() -> Vec<String> {
//     include_str!("../input2.txt")
//         .lines()
//         .map(|l| l.to_string())
//         .collect::<Vec<String>>()
// }

// pub fn input_test2() -> Vec<String> {
//     include_str!("../test2.txt")
//         .lines()
//         .map(|l| l.to_string())
//         .collect::<Vec<String>>()
// }
