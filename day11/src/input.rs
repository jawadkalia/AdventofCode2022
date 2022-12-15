pub fn input() -> Vec<String> {
    include_str!("../input.txt")
        .trim()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
}

pub fn input_test() -> Vec<&str> {
    include_str!("../test.txt")
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .split(|f| *f == "").
        .collect::<Vec<&str>>()
    // .map(|l| l.trim().split(' ').collect::<Vec<&str>>())
    // .collect::<Vec<Vec<&str>>>()
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
