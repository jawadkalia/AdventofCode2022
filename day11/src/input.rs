pub fn input() -> Vec<Vec<String>> {
    parse_text(include_str!("../input.txt"))
}

pub fn input_test() -> Vec<Vec<String>> {
    parse_text(include_str!("../test.txt"))
}

fn parse_text(s: &str) -> Vec<Vec<String>> {
    s.trim()
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
        .chunks(7)
        .map(|f| f.to_vec())
        .collect::<Vec<Vec<String>>>()
}
