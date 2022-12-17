use day11::{
    day11::Day11,
    input::{input, input_test},
    monkey::Monkey,
};

fn main() {
    let value = input();
    let mut day11 = Day11::default();
    value
        .iter()
        .enumerate()
        .for_each(|(monkey_number, monkey_input)| {
            day11
                .monkeys
                .insert(monkey_number, Monkey::from_words(monkey_input));
        });

    let day11_1 = day11.part1();
    println!("day11 part 1: {}", day11_1)
}
