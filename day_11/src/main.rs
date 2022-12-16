use std::{collections::VecDeque, time::{Instant, Duration}};
use num_bigint::{ToBigUint, BigUint};

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

use Operation::*;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<BigUint>,
    operation: Operation,
    test_denominator: usize,
    id_if_divisible: usize,
    id_if_not_divisible: usize,
    num_inspections: usize,
}
fn make_items(input: Vec<usize>) -> VecDeque<BigUint> {
    input.iter().map(|item| item.to_biguint().unwrap() ).collect::<VecDeque<BigUint>>()
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Operation,
        test_denominator: usize,
        id_if_divisible: usize,
        id_if_not_divisible: usize,
    ) -> Self {
        Monkey {
            items: make_items(items),
            operation,
            test_denominator,
            id_if_divisible,
            id_if_not_divisible,
            num_inspections: 0,
        }
    }
}

fn get_monkeys() -> VecDeque<Monkey> {
    VecDeque::from([
        Monkey::new(vec![91, 54, 70, 61, 64, 64, 60, 85], Multiply(13), 2, 5, 2),
        Monkey::new(vec![82], Add(7), 13, 4, 3),
        Monkey::new(vec![84, 93, 70], Add(2), 5, 5, 1),
        Monkey::new(vec![78, 56, 85, 93], Multiply(2), 3, 6, 7),
        Monkey::new(vec![64, 57, 81, 95, 52, 71, 58], Square, 11, 7, 3),
        Monkey::new(vec![58, 71, 96, 58, 68, 90], Add(6), 17, 4, 1),
        Monkey::new(vec![56, 99, 89, 97, 81], Add(1), 7, 0, 2),
        Monkey::new(vec![68, 72], Add(8), 19, 6, 0),
    ])
}
fn round(monkeys: &mut VecDeque<Monkey>, divide: bool) {
    for i in 0..monkeys.len() {
        let monkey = monkeys.get_mut(i).unwrap();
        let mut item_targets: VecDeque<(BigUint, usize)> =
            VecDeque::with_capacity(monkey.items.len());
        monkey.num_inspections = monkey.num_inspections + monkey.items.len() as usize;
        while monkey.items.len() > 0 {
            let mut item = monkey.items.pop_front().unwrap();
            item = match monkey.operation {
                Add(v) => item + v,
                Multiply(v) => item * v,
                Square => item.pow(2),
            };
            if divide {
            item = item / 3 as u8;
            }
            let target = if item.clone() % monkey.test_denominator == 0.to_biguint().unwrap() {
                monkey.id_if_divisible
            } else {
                monkey.id_if_not_divisible
            };
            item_targets.push_back((item, target));
        }
        drop(monkey);
        while let Some((item, target)) = item_targets.pop_front() {
            monkeys
                .get_mut(target.to_owned() as usize)
                .unwrap()
                .items
                .push_back(item);
        }
    }
}

fn monkey_business(monkeys: &VecDeque<Monkey>) -> usize {
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.num_inspections)
        .collect::<Vec<usize>>();
    inspections.sort();
    let len = inspections.len();
    inspections[len - 1] as usize * inspections[len - 2] as usize
}

fn main() {
    let mut monkeys = get_monkeys();
    let now = Instant::now();
    let num_rounds = 200; // 10000;
    for i in 1..=num_rounds {
        if i%100 == 0 {
            println!("{}/10000 ({}%) in {}s", i, (i*100)/num_rounds, now.elapsed().as_secs() )
        }
        round(&mut monkeys, false);
    }
    println!("Monkey business: {:?}", monkey_business(&monkeys));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_monkeys() -> VecDeque<Monkey> {
        VecDeque::from([
            Monkey::new(vec![79, 98], Multiply(19), 23, 2, 3),
            Monkey::new(vec![54, 65, 75, 74], Add(6), 19, 2, 0),
            Monkey::new(vec![79, 60, 97], Square, 13, 1, 3),
            Monkey::new(vec![74], Add(3), 17, 0, 1),
        ])
    }

    #[test]
    fn test_round() {
        let mut monkeys = get_sample_monkeys();
        round(&mut monkeys, true);

        assert_eq!(monkeys[0].items, make_items(vec![20, 23, 27, 26]));
        assert_eq!(monkeys[1].items, make_items(vec![2080, 25, 167, 207, 401, 1046]));
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
    }

    #[test]
    fn test_several_rounds() {
        let mut monkeys = get_sample_monkeys();
        for _ in 0..20 {
            round(&mut monkeys, true);
        }

        assert_eq!(monkeys[0].items, make_items(vec![10, 12, 14, 26, 34]));
        assert_eq!(monkeys[1].items, make_items(vec![245, 93, 53, 199, 115]));
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
    }

    #[test]
    fn test_num_inspections() {
        let mut monkeys = get_sample_monkeys();
        for _ in 0..20 {
            round(&mut monkeys, true);
        }

        assert_eq!(monkeys[0].num_inspections, 101);
        assert_eq!(monkeys[1].num_inspections, 95);
        assert_eq!(monkeys[2].num_inspections, 7);
        assert_eq!(monkeys[3].num_inspections, 105);
    }

    #[test]
    fn test_monkey_business() {
        let mut monkeys = get_sample_monkeys();
        for _ in 0..20 {
            round(&mut monkeys, true);
        }
        assert_eq!(monkey_business(&monkeys), 10605);
    }

    #[test]
    fn test_10k_no_div() {
        let mut monkeys = get_sample_monkeys();
        for _ in 0..10000 {
            round(&mut monkeys, false);
        }
        assert_eq!(monkey_business(&monkeys), 2713310158);
    }
}
