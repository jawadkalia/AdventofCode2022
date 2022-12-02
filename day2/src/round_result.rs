use crate::{plays::Plays, WINNING_SCORE, LOSING_SCORE, DRAW_SCORE};

#[derive(Debug)]
pub enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    pub fn score(&self, p: (&Plays, &Plays)) -> usize {
       
        match &self {
            RoundResult::Win => Self::win_calculation(p),
            RoundResult::Lose => Self::lose_calculation(p),
            RoundResult::Draw => Self::draw_calculation(p),
        }
    }

    fn win_calculation(p: (&Plays, &Plays)) -> usize {
        p.1.hand_score() + WINNING_SCORE
    }

    fn lose_calculation(p: (&Plays, &Plays)) -> usize {
        p.1.hand_score() + LOSING_SCORE
    }
    
    fn draw_calculation(p: (&Plays, &Plays)) -> usize {
        p.1.hand_score() + DRAW_SCORE
    }
}