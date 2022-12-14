pub fn input() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
}

pub fn input_test() -> Vec<String> {
    include_str!("../test.txt")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
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
