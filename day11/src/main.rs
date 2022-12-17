use day11::{
    input::{input, input_test},
    monkey::Monkey,
};

fn main() {
    let value = input();

    println!("{:#?}", value);
}

struct Day10 {
    monkeys: Vec<Monkey>,
}
