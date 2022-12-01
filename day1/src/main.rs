use std::collections::BinaryHeap;

fn main() {
    // Parse the input into an array of summed integers
    let parsed_value = parse_into_vector_of_sums();

    // Calculate day1_a answer
    let day1_a = &parsed_value.iter().max().unwrap().to_owned();
    
    // Calculate day2_a answer
    let mut vec_of_3 = Vec::new();

    let mut parsed_binary_heap = BinaryHeap::from(parsed_value);
    for _i in 0..3 {
        vec_of_3.push(parsed_binary_heap.pop().unwrap());
    }

    println!("Day 1a answer is {}", day1_a);
    println!("Day 1b answer is {}", vec_of_3.iter().sum::<u32>());
    // println!("Day 1b answer is {:#?}", vec_of_3);
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
