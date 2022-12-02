use crate::{desired_outcome::DesiredOutcome, plays::Plays, round_result::RoundResult};

#[derive(Debug)]
pub struct Round {
    self_hand: Plays,
    opponnent_hand: Plays,
    desired_outcome: DesiredOutcome,
}

impl Round {
    pub fn from_vec_of_two(a: &Vec<&str>) -> Self {
        Self {
            opponnent_hand: Plays::from_str(a.first().unwrap()),
            self_hand: Plays::from_str(a.get(1).unwrap()),
            desired_outcome: DesiredOutcome::from_str(a.get(1).unwrap()),
        }
    }

    pub fn score(&self) -> usize {
        let p = (&self.opponnent_hand, &self.self_hand);
        match p {
            (Plays::Rock, Plays::Rock) => RoundResult::Draw.score(p),
            (Plays::Rock, Plays::Paper) => RoundResult::Win.score(p),
            (Plays::Rock, Plays::Scissors) => RoundResult::Lose.score(p),
            (Plays::Paper, Plays::Rock) => RoundResult::Lose.score(p),
            (Plays::Paper, Plays::Paper) => RoundResult::Draw.score(p),
            (Plays::Paper, Plays::Scissors) => RoundResult::Win.score(p),
            (Plays::Scissors, Plays::Rock) => RoundResult::Win.score(p),
            (Plays::Scissors, Plays::Paper) => RoundResult::Lose.score(p),
            (Plays::Scissors, Plays::Scissors) => RoundResult::Draw.score(p),
        }
    }

    pub fn score_with_desired_outcome(&self) -> usize {
        let p = (&self.opponnent_hand, &self.desired_outcome);
        match p {
            (Plays::Rock, DesiredOutcome::Win) => {
                RoundResult::Win.score((&Plays::Rock, &Plays::Paper))
            }
            (Plays::Rock, DesiredOutcome::Lose) => {
                RoundResult::Lose.score((&Plays::Rock, &Plays::Scissors))
            }
            (Plays::Rock, DesiredOutcome::Draw) => {
                RoundResult::Draw.score((&Plays::Rock, &Plays::Rock))
            }
            (Plays::Paper, DesiredOutcome::Win) => {
                RoundResult::Win.score((&Plays::Paper, &Plays::Scissors))
            }
            (Plays::Paper, DesiredOutcome::Lose) => {
                RoundResult::Lose.score((&Plays::Paper, &Plays::Rock))
            }
            (Plays::Paper, DesiredOutcome::Draw) => {
                RoundResult::Draw.score((&Plays::Paper, &Plays::Paper))
            }
            (Plays::Scissors, DesiredOutcome::Win) => {
                RoundResult::Win.score((&Plays::Scissors, &Plays::Rock))
            }
            (Plays::Scissors, DesiredOutcome::Lose) => {
                RoundResult::Lose.score((&Plays::Scissors, &Plays::Paper))
            }
            (Plays::Scissors, DesiredOutcome::Draw) => {
                RoundResult::Draw.score((&Plays::Scissors, &Plays::Scissors))
            }
        }
    }
}
