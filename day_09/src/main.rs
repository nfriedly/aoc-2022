#[derive(Debug)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

use Dir::*;

#[derive(Debug)]
struct Move {
    dir: Dir,
    dist: u8,
}

fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let (dir_str, dist_str) = line.split_once(" ").expect("no whitespace inline");
            let dir = match dir_str {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => panic!("unexpected direction input"),
            };
            let dist = dist_str.parse().expect("unparseable distance");
            Move { dir, dist }
        })
        .collect()
}

fn main() {
    let input = include_str!("input-sample.txt");
    let moves = parse(input);
    println!("moves: {:#? }", moves);
}
