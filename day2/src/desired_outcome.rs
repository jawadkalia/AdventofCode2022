#[derive(Debug)]
pub enum DesiredOutcome {
    Win,
    Lose,
    Draw,
}

impl DesiredOutcome {
    pub fn from_str(s: &str) -> Self {
        match s {
            "X" => DesiredOutcome::Lose,
            "Y" => DesiredOutcome::Draw,
            "Z" => DesiredOutcome::Win,
            _ => unreachable!()
        }
    } 
}