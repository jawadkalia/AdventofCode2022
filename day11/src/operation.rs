#[derive(Debug, Default, Clone)]
pub enum Operation {
    #[default]
    Noop,
    Add(u32),
    Multiply(u32),
    AddSelf,
    MultiplySelf,
}

impl Operation {
    pub fn calc(&self, val: u32) -> u32 {
        match self {
            Operation::Noop => val,
            Operation::Add(n) => val + n,
            Operation::Multiply(n) => val * n,
            Operation::AddSelf => val + val,
            Operation::MultiplySelf => val * val,
        }
    }
}
