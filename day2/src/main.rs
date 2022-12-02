use day2::round::Round;

fn main() {
    let data = include_str!("../input.txt").split("/n").map(|s| {
        s.lines()
           .map(|line| line.split(" ").collect::<Vec<&str>>())
           .map(|element| Round::from_vec_of_two(&element).score()).collect::<Vec<usize>>()
            
    }).next();

    let final_score = data.unwrap().iter().sum::<usize>();

    println!("day2_a answer is {}", final_score);
}

