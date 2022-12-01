fn main() {
    let value = parse_into_vector_of_sums()
        .iter()
        .max()
        .unwrap()
        .to_owned();
    println!("Day 1a answer is {:#?}", value);
}

fn parse_into_vector_of_sums() -> Vec<u32> {
    include_str!("../input.txt")
        .split("\n\n")
        
        .map(|n| {
            n.lines()
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>()
}
