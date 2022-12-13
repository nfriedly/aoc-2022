#[derive(Debug, Clone, Copy)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

use std::{cmp::Ordering, collections::HashSet};

use Dir::*;

#[derive(Debug)]
struct Move {
    dir: Dir,
    dist: u8,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Loc {
    x: i32,
    y: i32,
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

fn step_h(h: Loc, dir: Dir) -> Loc {
    match dir {
        Up => Loc { x: h.x, y: h.y + 1 },
        Down => Loc { x: h.x, y: h.y - 1 },
        Right => Loc { x: h.x + 1, y: h.y },
        Left => Loc { x: h.x - 1, y: h.y },
    }
}

fn step_t(t: Loc, h: Loc) -> Loc {
    let dx = h.x - t.x;
    let dy = h.y - t.y;
    let dxa = dx.abs();
    let dya = dy.abs();
    if dxa <= 1 && dya <= 1 {
        return t;
    }
    let x = match dx.cmp(&0) {
        Ordering::Greater => t.x + 1,
        Ordering::Equal => t.x,
        Ordering::Less => t.x - 1,
    };
    let y = match dy.cmp(&0) {
        Ordering::Greater => t.y + 1,
        Ordering::Equal => t.y,
        Ordering::Less => t.y - 1,
    };
    Loc { x, y }
}

fn follow(moves: Vec<Move>) -> usize {
    let mut h = Loc { x: 0, y: 0 };
    let mut t = Loc { x: 0, y: 0 };
    let mut tail_locations = HashSet::from([t]);
    for m in moves {
        for _ in 0..(m.dist) {
            h = step_h(h, m.dir);
            t = step_t(t, h);
            tail_locations.insert(t);
        }
    }
    tail_locations.len()
}

fn main() {
    let input = include_str!("input.txt");
    let moves = parse(input);
    let num_t_loc = follow(moves);
    println!("num tail locations: {}", num_t_loc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_t() {
        let h = Loc { x: 4, y: 2 };
        let t = Loc { x: 3, y: 0 };
        assert_eq!(step_t(t, h), Loc { x: 4, y: 1 });
    }

    #[test]
    fn test_step_follow() {
        let input = include_str!("input-sample.txt");
        let moves = parse(input);
        let num_t_loc = follow(moves);
        assert_eq!(num_t_loc, 13);
    }
}
