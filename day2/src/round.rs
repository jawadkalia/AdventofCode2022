use crate::{plays::Plays, round_result::RoundResult};


#[derive(Debug)]
pub struct Round {
    self_hand: Plays,
    opponnent_hand: Plays,
}



impl Round {
    pub fn from_vec_of_two(a: &Vec<&str>) -> Self {
        Self {
            opponnent_hand: Plays::from_str(a.first().unwrap()),
            self_hand: Plays::from_str(a.get(1).unwrap()),
        }
    }

    pub fn score(&self) -> usize {
        let p = (&self.self_hand, &self.opponnent_hand);
        match p {
            (Plays::Rock, Plays::Rock) => RoundResult::Draw.score(p),
            (Plays::Rock, Plays::Paper) => RoundResult::Win.score(p),
            (Plays::Rock, Plays::Scissors) => RoundResult::Lose.score(p),
            (Plays::Rock, Plays::Other) => 0,
            (Plays::Paper, Plays::Rock) => RoundResult::Lose.score(p),
            (Plays::Paper, Plays::Paper) => RoundResult::Draw.score(p),
            (Plays::Paper, Plays::Scissors) => RoundResult::Win.score(p),
            (Plays::Paper, Plays::Other) => 0,
            (Plays::Scissors, Plays::Rock) => RoundResult::Win.score(p),
            (Plays::Scissors, Plays::Paper) => RoundResult::Lose.score(p),
            (Plays::Scissors, Plays::Scissors) => RoundResult::Draw.score(p),
            (Plays::Scissors, Plays::Other) => 0,
            (Plays::Other, Plays::Rock) => 0,
            (Plays::Other, Plays::Paper) => 0,
            (Plays::Other, Plays::Scissors) => 0,
            (Plays::Other, Plays::Other) => 0,
        }
    }

}