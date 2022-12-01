use std::collections::BinaryHeap;

fn main() {
    // Parse the input into an array of summed integers
    let parsed_value = parse_into_vector_of_sums();

    // Calculate day1_a answer
    let day1_a = &parsed_value.iter().max().unwrap().to_owned();
    println!("Day 1a answer is {}", day1_a);
    
    // Calculate day2_a answer
    let parsed_binary_heap = BinaryHeap::from(parsed_value);
    println!("Day 1b answer is {:#?}", parsed_binary_heap.iter().take(3).sum::<u32>());
    
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
