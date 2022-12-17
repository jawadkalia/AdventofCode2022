#[derive(Debug, Default, Clone)]
pub enum Operation {
    #[default]
    Noop,
    Add(u64),
    Multiply(u64),
    AddSelf,
    MultiplySelf,
}

impl Operation {
    pub fn calc(&self, val: u64) -> u64 {
        match self {
            Operation::Noop => val,
            Operation::Add(n) => val + n,
            Operation::Multiply(n) => val * n,
            Operation::AddSelf => val + val,
            Operation::MultiplySelf => val * val,
        }
    }
}
