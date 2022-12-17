use std::collections::HashMap;

use day11::{
    input::{input, input_test},
    monkey::Monkey,
};

fn main() {
    let value = input_test();
    // dbg!(value.clone());
    let mut day11 = Day11::default();
    value
        .iter()
        .enumerate()
        .for_each(|(monkey_number, monkey_input)| {
            day11
                .monkeys
                .insert(monkey_number, Monkey::from_words(monkey_input));
        });
    dbg!(day11);
}

#[derive(Debug, Default)]
struct Day11 {
    monkeys: HashMap<usize, Monkey>,
}
