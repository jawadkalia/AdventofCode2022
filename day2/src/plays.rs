#[derive(Debug)]
pub enum Plays {
    Rock,
    Paper,
    Scissors,
}

impl Plays {
    pub fn from_str(s: &str) -> Self {
        match s {
            "X" => Plays::Rock,
            "Y" => Plays::Paper,
            "Z" => Plays::Scissors,
            "A" => Plays::Rock,
            "B" => Plays::Paper,
            "C" => Plays::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn hand_score(&self) -> usize {
        match &self {
            Plays::Rock => 1,
            Plays::Paper => 2,
            Plays::Scissors => 3,
        }
    }
}
