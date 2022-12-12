fn main() {
    println!("Hello, world!");
}

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
