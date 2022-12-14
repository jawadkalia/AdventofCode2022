#[derive(Debug)]
pub enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    pub fn from_string(s: Vec<&str>) -> Self {
        match *s.first().unwrap() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(s.last().unwrap().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}
