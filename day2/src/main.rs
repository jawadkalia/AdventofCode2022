use day2::round::Round;

fn main() {
    let data = parse_input_to_arrays();
    let day2data = data.clone();

    // dbg!(&data);

    let day2_a_value = data
        .unwrap()
        .iter()
        .map(|element| Round::from_vec_of_two(&element).score())
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>();

    let day2_b_value = day2data
        .unwrap()
        .iter()
        .map(|element| Round::from_vec_of_two(&element).score_with_desired_outcome())
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>();

    println!("day2_a answer is {}", day2_a_value);
    println!("day2_b answer is {}", day2_b_value);

}

fn parse_input_to_arrays() -> Option<Vec<Vec<&'static str>>> {
    include_str!("../input.txt")
        .split("/n")
        .map(|s| {
            s.lines()
                .map(|line| line.split(" ").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        })
        .next()
}
