#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Play {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

fn evaluate_game((opp, me): (Play, Play)) -> Outcome {
    return match (opp, me) {
        (Play::Scissors, Play::Rock) => Outcome::Win,
        (Play::Rock, Play::Paper) => Outcome::Win,
        (Play::Paper, Play::Scissors) => Outcome::Win,
        (Play::Rock, Play::Rock) => Outcome::Draw,
        (Play::Paper, Play::Paper) => Outcome::Draw,
        (Play::Scissors, Play::Scissors) => Outcome::Draw,
        _ => Outcome::Lose,
    };
}

fn find_play((opp, outcome): (Play, Outcome)) -> Play {
    return match (opp, outcome) {
        (Play::Rock, Outcome::Win) => Play::Paper,
        (Play::Paper, Outcome::Win) => Play::Scissors,
        (Play::Scissors, Outcome::Win) => Play::Rock,
        (Play::Rock, Outcome::Lose) => Play::Scissors,
        (Play::Paper, Outcome::Lose) => Play::Rock,
        (Play::Scissors, Outcome::Lose) => Play::Paper,
        (play, Outcome::Draw) => play,
    };
}

fn main() {
    let file_data = std::fs::read_to_string("src/inputs/day2.txt").expect("Failed to read the file!");

    let first_score: i32 = file_data
        .split("\n")
        .filter(|game| !game.is_empty())
        .map(|game| {
            let (opp, me) = game.split_once(" ").expect("Failed to read input.txt");

            let outcome = evaluate_game((Play::from(opp), Play::from(me)));

            return outcome.score() + Play::from(me).score();
        })
        .sum();

    let second_score: i32 = file_data
        .split("\n")
        .filter(|game| !game.is_empty())
        .map(|game| {
            let (opp, outcome) = game.split_once(" ").expect("Failed to read input.txt");

            let me = find_play((Play::from(opp), Outcome::from(outcome)));

            return Outcome::from(outcome).score() + me.score();
        })
        .sum();

    dbg!(first_score);
    dbg!(second_score);
}
