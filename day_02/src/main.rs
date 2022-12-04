#[derive(Debug, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loose,
}

use RPS::*;
use Outcome::*;

fn parse_line(line: &str) -> (RPS, Outcome) {
    let mut iter = line.split_whitespace();
    let left = match iter.next() {
        Some("A") => Rock,
        Some("B") => Paper,
        Some("C") => Scissors,
        _ => panic!("failed to parse first letter"),
    };
    let right = match iter.next() {
        Some("X") => Loose,
        Some("Y") => Draw,
        Some("Z") => Win,
        _ => panic!("failed to parse second letter"),
    };
    (left, right)
}

fn parse(input: &str) -> Vec<(RPS, Outcome)> {
    let mut parsed: Vec<(RPS, Outcome)> = Vec::new();
    for line in input.lines() {
        parsed.push(parse_line(line));
    }
    parsed
}

fn score_round(round: (RPS, Outcome)) -> usize {
    let my_hand = match round {
        (anything, Draw) => anything,
        (Rock, Loose) => Scissors,
        (Rock, Win) => Paper,
        (Paper, Loose) =>  Rock,
        (Paper, Win) => Scissors,
        (Scissors, Loose) => Paper,
        (Scissors, Win) => Rock,
    };
    let wld_points = match round {
        (_, Loose) => 0,
        (_, Draw) => 3,
        (_, Win) => 6
    };
    let shape_points = match my_hand {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };
    let points = wld_points + shape_points;

    println!("response for {:?} is {:?} for {} points", round, my_hand, points);
    points
}

fn score(rounds:  Vec<(RPS, Outcome)>) -> usize {
    let mut points = 0;
    for round in rounds {
        points += score_round(round);
    }
    points
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", score(parse(input)));
}
