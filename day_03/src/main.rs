use std::{collections::{HashSet, HashMap}};


fn main() {
    // there's gotta be a less verbose way to do this...
    let priorities = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),          
    ]);
    let input = include_str!("input.txt");
    let sum: i32 = input.lines().map(|line| {
        let (left, right) = line.split_at(line.len()/2);
        let left_set: HashSet<char, std::collections::hash_map::RandomState> = HashSet::from_iter(left.chars());
        let right_set: HashSet<char, std::collections::hash_map::RandomState> = HashSet::from_iter(right.chars());
        let intersection: Vec<&char> = left_set.intersection(&right_set).collect();
        println!("intersection: {:?} ", intersection);
        assert!(intersection.len() == 1);
        intersection.get(0).unwrap().to_owned().to_owned()
    })
    .map(|c| priorities.get(&c).unwrap().to_owned())
    .sum();
    println!("sum: {}", sum);
}
