#[derive(Debug, Default, Clone)]
pub enum Operation {
    #[default]
    Noop,
    Add(u32),
    Multiply(u32),
    AddSelf,
    MultiplySelf,
}
