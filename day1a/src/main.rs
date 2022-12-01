fn main() {
    let value = include_str!("../input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| {
            n.split("\n")
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .iter()
                .sum::<u32>()
        })
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap()
        .to_owned();
    println!("{:#?}", value);
}
