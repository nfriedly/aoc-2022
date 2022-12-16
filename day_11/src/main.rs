use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

use Operation::*;

type Item = HashMap<usize, usize>;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test_denominator: usize,
    id_if_divisible: usize,
    id_if_not_divisible: usize,
    num_inspections: usize,
}
fn make_items(input: Vec<usize>, denominators: &Vec<usize>) -> VecDeque<Item> {
    input
        .iter()
        .map(|item| {
            let mut remainders: Item = HashMap::new();
            for d in denominators.iter() {
                remainders.insert(*d, item % d);
            }
            remainders
        })
        .collect::<VecDeque<Item>>()
}

impl Monkey {
    fn new(
        items: VecDeque<Item>,
        operation: Operation,
        test_denominator: usize,
        id_if_divisible: usize,
        id_if_not_divisible: usize,
    ) -> Self {
        Monkey {
            items,
            operation,
            test_denominator,
            id_if_divisible,
            id_if_not_divisible,
            num_inspections: 0,
        }
    }
}

fn get_monkeys() -> VecDeque<Monkey> {
    let denominators = vec![2, 13, 5, 3, 11, 17, 7, 19];
    VecDeque::from([
        Monkey::new(
            make_items(vec![91, 54, 70, 61, 64, 64, 60, 85], &denominators),
            Multiply(13),
            2,
            5,
            2,
        ),
        Monkey::new(make_items(vec![82], &denominators), Add(7), 13, 4, 3),
        Monkey::new(make_items(vec![84, 93, 70], &denominators), Add(2), 5, 5, 1),
        Monkey::new(
            make_items(vec![78, 56, 85, 93], &denominators),
            Multiply(2),
            3,
            6,
            7,
        ),
        Monkey::new(
            make_items(vec![64, 57, 81, 95, 52, 71, 58], &denominators),
            Square,
            11,
            7,
            3,
        ),
        Monkey::new(
            make_items(vec![58, 71, 96, 58, 68, 90], &denominators),
            Add(6),
            17,
            4,
            1,
        ),
        Monkey::new(
            make_items(vec![56, 99, 89, 97, 81], &denominators),
            Add(1),
            7,
            0,
            2,
        ),
        Monkey::new(make_items(vec![68, 72], &denominators), Add(8), 19, 6, 0),
    ])
}
fn round(monkeys: &mut VecDeque<Monkey>, divide: bool) {
    for i in 0..monkeys.len() {
        let monkey = monkeys.get_mut(i).unwrap();
        let mut item_targets: VecDeque<(Item, usize)> = VecDeque::with_capacity(monkey.items.len());
        monkey.num_inspections = monkey.num_inspections + monkey.items.len() as usize;
        while monkey.items.len() > 0 {
            let mut item = monkey.items.pop_front().unwrap();
            //println!("monkey {} inspects item {:?} and {:?}", i, item, monkey.operation);
            item = match monkey.operation {
                Add(v) => {
                    let mut new_item: Item = HashMap::new();
                    for denominator in item.keys() {
                        let remainder = item.get(denominator).unwrap();
                        new_item.insert(*denominator, (remainder + v) % denominator);
                    }
                    new_item
                }
                Multiply(v) => {
                    let mut new_item: Item = HashMap::new();
                    for denominator in item.keys() {
                        let remainder = item.get(denominator).unwrap();
                        let new_remainder: u128 =
                            *remainder as u128 * v as u128 % *denominator as u128;
                        new_item.insert(*denominator, new_remainder as usize);
                    }
                    new_item
                }
                Square => {
                    let mut new_item: Item = HashMap::new();
                    for denominator in item.keys() {
                        let remainder = item.get(denominator).unwrap();
                        let new_remainder: u128 =
                            *remainder as u128 * *remainder as u128 % *denominator as u128;
                        new_item.insert(*denominator, new_remainder as usize);
                    }
                    new_item
                }
            };
            if divide {
                let mut new_item: Item = HashMap::new();
                for denominator in item.keys() {
                    let remainder = item.get(denominator).unwrap();
                    // ugh, this isn't right
                    new_item.insert(*denominator, ((remainder) / 3) % denominator);
                }
                item = new_item
            }
            //print!("new item {:?} checking for divisibility by {}...", item, monkey.test_denominator);
            let target = if item.get(&monkey.test_denominator) == Some(&0) {
                //println!("evenly divisible, going to {}",monkey.id_if_divisible);
                monkey.id_if_divisible
            } else {
                //println!("not evenly divisible, going to {}",monkey.id_if_not_divisible);
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
    let total_time = Instant::now();
    let num_rounds = 10000; // 10,000
    let checkpoint = 100;
    let mut checkpoint_time = Instant::now();
    println!("running for {} rounds", num_rounds);
    for i in 1..=num_rounds {
        if i % checkpoint == 0 {
            let checkpoint_duration = checkpoint_time.elapsed().as_secs();
            checkpoint_time = Instant::now();
            let round_duration = checkpoint_duration / checkpoint;
            let eta = round_duration * (num_rounds - i);
            println!(
                "{} rounds ({}%) completed in {}s. ETA is {}s",
                i,
                (i * 100) / num_rounds,
                total_time.elapsed().as_secs(),
                eta
            )
        }
        round(&mut monkeys, false);
    }
    println!("Monkey business: {:?}", monkey_business(&monkeys));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_denominators() -> Vec<usize> {
        vec![23, 19, 13, 17]
    }

    fn get_sample_monkeys() -> VecDeque<Monkey> {
        let denominators = get_sample_denominators();
        VecDeque::from([
            Monkey::new(
                make_items(vec![79, 98], &denominators),
                Multiply(19),
                23,
                2,
                3,
            ),
            Monkey::new(
                make_items(vec![54, 65, 75, 74], &denominators),
                Add(6),
                19,
                2,
                0,
            ),
            Monkey::new(
                make_items(vec![79, 60, 97], &denominators),
                Square,
                13,
                1,
                3,
            ),
            Monkey::new(make_items(vec![74], &denominators), Add(3), 17, 0, 1),
        ])
    }

    #[test]
    fn test_round() {
        let mut monkeys = get_sample_monkeys();
        round(&mut monkeys, true);

        let denominators = get_sample_denominators();
        // 20, 23, 27, 26
        assert_eq!(
            monkeys[0].items,
            make_items(vec![20, 23, 27, 26], &denominators)
        );
        assert_eq!(
            monkeys[1].items,
            make_items(vec![2080, 25, 167, 207, 401, 1046], &denominators)
        );
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
    }

    // #[test]
    // fn test_several_rounds() {
    //     let mut monkeys = get_sample_monkeys();
    //     for _ in 0..20 {
    //         round(&mut monkeys, true);
    //     }

    //     assert_eq!(monkeys[0].items, make_items(vec![10, 12, 14, 26, 34]));
    //     assert_eq!(monkeys[1].items, make_items(vec![245, 93, 53, 199, 115]));
    //     assert_eq!(monkeys[2].items, vec![]);
    //     assert_eq!(monkeys[3].items, vec![]);
    // }

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
