use day10::{
    input::{input, input_test},
    instruction::Instruction,
};

fn main() {
    let mut day10 = Day10::default();
    input().iter().for_each(|f| {
        let words = f.split(' ').collect::<Vec<&str>>();
        day10.statements.push(Instruction::from_string(words));
    });

    dbg!(day10.part1());
}

#[derive(Debug, Default)]
struct Day10 {
    statements: Vec<Instruction>,
    display_crt: Vec<String>,
}

impl Day10 {
    fn part1(&mut self) -> i32 {
        let mut signal_strength = 0;
        let mut x_register = 1;
        let mut cycle = 0;
        let mut check = 20;

        self.statements.iter().for_each(|ele| match ele {
            Instruction::AddX(value) => {
                cycle += 2;
                if cycle >= check {
                    signal_strength += check * x_register;
                    println!("At cycle {check}, signal = {}", cycle * x_register);
                    check += 40;
                }
                x_register += value;
            }
            Instruction::Noop => {
                cycle += 1;
                if cycle >= check {
                    signal_strength += cycle * x_register;
                    println!("At cycle {check}, signal = {}", cycle * x_register);
                    check += 40;
                }
            }
        });
        signal_strength
    }

    fn part2(&self) -> Vec<String> {
        self.display_crt.clone()
    }
}
