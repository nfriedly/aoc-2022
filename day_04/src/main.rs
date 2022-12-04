use std::ops::RangeInclusive;

fn parse_range(input: &str) -> RangeInclusive<usize> {
    let (left, right) = input.split_once("-").expect("failed to split at dash");
    RangeInclusive::new(left.parse().unwrap(), right.parse().unwrap())
}

fn parse_line(line: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    // let my_int = from_str::<int>(my_str);
    // std::ops::Range { start: 3, end: 5 });
    let (left, right) = line.split_once(",").expect("failed to split at comma");
    (parse_range(left), parse_range(right))
}

fn main() {
    let input = include_str!("input.txt");
    let ranges = input
        .lines()
        .map(|l| l.trim())
        .map(|l| parse_line(l))
        .filter(|(left, right)| {
            (left.contains(right.start()) && left.contains(right.end()))
                || (right.contains(left.start()) && right.contains(left.end()))
        })
        .count();
    println!("contained pars: {} ", ranges);
}
