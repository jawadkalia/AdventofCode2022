fn main() {
    let data = include_str!("../input.txt").split("/n").map(|s| {
        s.lines()
           .map(|line| line.split(" ").collect::<Vec<&str>>())
           .map(|element| Round::from_vec_of_two(&element).score()).collect::<Vec<usize>>()
            
    }).next();

    // dbg!(&data);
    let final_score = data.unwrap().iter().sum::<usize>();
    dbg!(final_score);
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    fn score(&self, p: (&Plays, &Plays)) -> usize {
       
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
       p.1.hand_score() - match p.0 {
        Plays::Rock => 1,
        _ => 0
    }
    }
    fn draw_calculation(p: (&Plays, &Plays)) -> usize {
        p.0.hand_score() + p.1.hand_score() 
    }
}

#[derive(Debug)]
enum Plays {
    Rock,
    Paper,
    Scissors,
    Other,
}

impl Plays {
    fn from_str(s: &str) -> Self {
        match s {
            "X" => Plays::Rock,
            "Y" => Plays::Paper,
            "Z" => Plays::Scissors,
            "A" => Plays::Rock,
            "B" => Plays::Paper,
            "C" => Plays::Scissors,
            _ => Plays::Other,
        }
    }

    fn hand_score(&self) -> usize {
        match &self {
            Plays::Rock => 1,
            Plays::Paper => 2,
            Plays::Scissors => 3,
            Plays::Other => 0,
        }
    }
}

#[derive(Debug)]
struct Round {
    self_hand: Plays,
    opponnent_hand: Plays,
}

const WINNING_SCORE: usize = 6;

impl Round {
    fn from_vec_of_two(a: &Vec<&str>) -> Self {
        Self {
            self_hand: Plays::from_str(a.first().unwrap()),
            opponnent_hand: Plays::from_str(a.get(1).unwrap()),
        }
    }

    fn score(&self) -> usize {
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



