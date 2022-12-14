use day10::{
    input::{input, input_test},
    instruction::Instruction,
    machine::Machine,
};

fn main() {
    let mut day10 = Day10::default();
    input().iter().for_each(|f| {
        let words = f.split(' ').collect::<Vec<&str>>();
        day10.statements.push(Instruction::from_string(words));
    });

    dbg!(day10.part1());
    dbg!(day10.part2());
}

#[derive(Debug, Default)]
struct Day10 {
    statements: Vec<Instruction>,
    machine: Machine,
}

impl Day10 {
    fn part1(&mut self) -> i32 {
        let mut signal_strength = 0;
        self.machine.init();
        let mut check = 20;

        self.statements.iter().for_each(|ele| match ele {
            Instruction::AddX(value) => {
                let prev_xreg = self.machine.xreg;
                self.machine.addx(*value);
                if self.machine.current_cycle >= check {
                    signal_strength += check * prev_xreg;
                    check += 40;
                }
            }
            Instruction::Noop => {
                self.machine.noop();
                if self.machine.current_cycle >= check {
                    signal_strength += self.machine.current_cycle * self.machine.xreg;
                    println!(
                        "At cycle {check}, signal = {}",
                        self.machine.current_cycle * self.machine.xreg
                    );
                    check += 40;
                }
            }
        });
        self.machine.noop();
        signal_strength
    }

    fn part2(&self) -> Vec<String> {
        self.machine.crt.clone()
    }
}
