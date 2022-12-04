#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

use RPS::*;

fn parse_line(line: &str) -> (RPS, RPS) {
    let mut iter = line.split_whitespace();
    let left = match iter.next() {
        Some("A") => Rock,
        Some("B") => Paper,
        Some("C") => Scissors,
        _ => panic!("failed to parse first letter"),
    };
    let right = match iter.next() {
        Some("X") => Rock,
        Some("Y") => Paper,
        Some("Z") => Scissors,
        _ => panic!("failed to parse second letter"),
    };
    (left, right)
}

fn parse(input: &str) -> Vec<(RPS, RPS)> {
    let mut parsed: Vec<(RPS, RPS)> = Vec::new();
    for line in input.lines() {
        parsed.push(parse_line(line));
    }
    parsed
}

fn score_round(round: (RPS, RPS)) -> usize {
    let mut points = 0;
    points += match round {
        (_, Rock) => 1,
        (_, Paper) => 2,
        (_, Scissors) => 3,
    };
    points += match round {
        (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => 0,
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
    };
    println!("score for {:?} is {}", round, points);
    points
}

fn score(rounds:  Vec<(RPS, RPS)>) -> usize {
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
