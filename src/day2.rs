use crate::lines_from_file;
use std::fmt;

#[derive(Copy, Clone)]
enum Pick {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

#[derive(Copy, Clone)]
struct Game {
    opp: Pick,
    mine: Pick,
}

impl fmt::Display for Pick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Pick::Rock => write!(f, "Rock"),
            Pick::Paper => write!(f, "Paper"),
            Pick::Scissors => write!(f, "Scissors"),
        }
    }
}

fn score_round(g: Game) -> i32 {
    use crate::day2::Outcome::*;
    use crate::day2::Pick::*;
    let score = match (g.opp, g.mine) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Win,
        (Rock, Scissors) => Loss,
        (Paper, Rock) => Loss,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Loss,
        (Scissors, Scissors) => Draw,
    };
    g.mine as i32 + score as i32
}

fn derive_pick(opp: Pick, outcome: Outcome) -> Pick {
    use crate::day2::Outcome::*;
    use crate::day2::Pick::*;
    let pick = match (opp, outcome) {
        (Rock, Draw) => Rock,
        (Rock, Win) => Paper,
        (Rock, Loss) => Scissors,
        (Paper, Loss) => Rock,
        (Paper, Draw) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Win) => Rock,
        (Scissors, Loss) => Paper,
        (Scissors, Draw) => Scissors,
    };
    pick
}

fn part_one() {
    let lines = lines_from_file("./inputs/day-2");
    let mut total_score = 0;
    for line in lines {
        let play: Vec<&str> = line.split(" ").collect();
        let game = Game {
            opp: match play[0] {
                "A" => Pick::Rock,
                "B" => Pick::Paper,
                "C" => Pick::Scissors,
                &_ => todo!(),
            },
            mine: match play[1] {
                "X" => Pick::Rock,
                "Y" => Pick::Paper,
                "Z" => Pick::Scissors,
                &_ => todo!(),
            },
        };
        let score = score_round(game);
        total_score += score;
    }
    println!("{}", total_score);
}

fn part_two(lines: Vec<String>) {
    let mut total_score: i32 = 0;
    for line in lines {
        let play_outcome: Vec<&str> = line.split(" ").collect();
        let opp = match play_outcome[0] {
            "A" => Pick::Rock,
            "B" => Pick::Paper,
            "C" => Pick::Scissors,
            &_ => todo!(),
        };
        let outcome = match play_outcome[1] {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            &_ => todo!(),
        };
        let game = Game {
            opp,
            mine: derive_pick(opp, outcome),
        };
        let score = score_round(game);
        total_score += score;
    }
    println!("{}", total_score);
}

pub fn run() {
    let lines = lines_from_file("./inputs/day-2");
    part_one();
    part_two(lines);
}
