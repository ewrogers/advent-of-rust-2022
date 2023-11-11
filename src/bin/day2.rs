#![warn(clippy::pedantic)]
use crate::Outcome::{Draw, Loss, Win};
use crate::Shape::{Paper, Rock, Scissors};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn parse(string: &str) -> Option<Self> {
        match string {
            "A" | "X" => Some(Rock),
            "B" | "Y" => Some(Paper),
            "C" | "Z" => Some(Scissors),
            _ => None,
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    pub fn parse(string: &str) -> Option<Self> {
        match string {
            "X" => Some(Loss),
            "Y" => Some(Draw),
            "Z" => Some(Win),
            _ => None,
        }
    }
}

struct Round {
    their_shape: Shape,
    your_shape: Shape,
    desired_shape: Shape,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day2_input.txt")?;
    let reader = BufReader::new(file);

    let mut rounds: Vec<Round> = Vec::with_capacity(1000);

    for line in reader.lines() {
        let line = line.unwrap();

        // Attempt to split into two column values, skip if invalid line
        let Some((them, you)) = line.split_once(' ') else {
            println!("Invalid line: {}", &line);
            continue;
        };

        // Parse the shape that the opponent is playing
        let Some(their_shape) = Shape::parse(them) else {
            continue;
        };

        // Parse the shape that you are playing (part 1 only)
        let Some(your_shape) = Shape::parse(you) else {
            continue;
        };

        // Parse the desired outcome for you (part 2 only)
        let Some(desired_outcome) = Outcome::parse(you) else {
            continue;
        };

        // Determine the shape needed to play for the desired outcome (part 2 only)
        let desired_shape = match (&their_shape, &desired_outcome) {
            (Rock, Win) | (Scissors, Loss) => Paper,
            (Paper, Win) | (Rock, Loss) => Scissors,
            (Scissors, Win) | (Paper, Loss) => Rock,
            _ => their_shape.clone(),
        };

        let round = Round {
            their_shape,
            your_shape,
            desired_shape,
        };

        rounds.push(round);
    }

    let part_1_score: u32 = rounds
        .iter()
        .map(|round| calc_score(&round.their_shape, &round.your_shape))
        .sum();

    let part_2_score: u32 = rounds
        .iter()
        .map(|round| calc_score(&round.their_shape, &round.desired_shape))
        .sum();

    println!("[Part I] Total score after all rounds is {part_1_score}");
    println!("[Part II] Total score after all desired outcomes is {part_2_score}");
    Ok(())
}

// Calculates the score of a round based on what shapes were played
fn calc_score(their_shape: &Shape, your_shape: &Shape) -> u32 {
    let match_score = match determine_outcome(their_shape, your_shape) {
        Win => 6,
        Draw => 3,
        Loss => 0,
    };

    let shape_score: u32 = match your_shape {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    match_score + shape_score
}

// Determines the outcome of a round based on what shapes were played
fn determine_outcome(their_shape: &Shape, your_shape: &Shape) -> Outcome {
    match (their_shape, your_shape) {
        (them, you) if them == you => Draw,
        (them, you) if a_beats_b(you, them) => Win,
        _ => Loss,
    }
}

// Determines if shape A beats shape B
fn a_beats_b(a: &Shape, b: &Shape) -> bool {
    // See: https://doc.rust-lang.org/std/macro.matches.html
    matches!((a, b), (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper))
}
