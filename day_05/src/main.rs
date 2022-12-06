use std::collections::HashMap;

type Stack = Vec<char>;
type Stacks = HashMap<usize, Stack>;

// note: this assumes that stacks numbers can be infered from their position
fn parse_initial_state(input: &str) -> Stacks {
    let (mut stack_lines, id_line): (Vec<&str>, Vec<&str>) =
        input.lines().partition(|l| !l.starts_with(" 1"));
    let num_stacks: usize = id_line.get(0).unwrap().split_whitespace().count();
    let mut stacks: Stacks = HashMap::with_capacity(num_stacks);
    stack_lines.reverse();
    for line in stack_lines {
        let chars: Vec<char> = line.chars().collect();
        for stack_num in 1..=num_stacks {
            let col = (stack_num - 1) * 4 + 1;
            let char = chars.get(col).unwrap();
            if char.eq(&' ') {
                continue;
            }
            if !stacks.contains_key(&stack_num) {
                stacks.insert(stack_num, Vec::new());
            }
            let stack = stacks.get_mut(&stack_num).unwrap();
            stack.push(char.clone());
        }
    }
    stacks
}

#[derive(Debug)]
struct Move {
    num: usize,
    src: usize,
    dest: usize,
}

fn parse_instructions(input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        moves.push(Move {
            num: words.get(1).unwrap().parse().unwrap(),
            src: words.get(3).unwrap().parse().unwrap(),
            dest: words.get(5).unwrap().parse().unwrap(),
        })
    }
    moves
}

fn do_moves(mut stacks: Stacks, moves: Vec<Move>) -> Stacks {
    for instruction in moves {
        for _ in 0..instruction.num {
            let thing = stacks.get_mut(&instruction.src).unwrap().pop().unwrap();
            stacks.get_mut(&instruction.dest).unwrap().push(thing);
        }
        println!("\nmove:{:?}\n stacks now: {:?}\n", instruction, stacks);
    }
    stacks
}

fn get_tops(stacks: &Stacks) -> Vec<char> {
    let mut tops: Vec<char> = Vec::new();
    for key in 1..=stacks.len() {
        tops.push(stacks.get(&key).unwrap().last().unwrap().clone())
    }
    tops
}

fn main() {
    let input = include_str!("input.txt");
    let (initial_state_input, instructions_input) = input.split_once("\r\n\r\n").unwrap();
    let mut stacks = parse_initial_state(initial_state_input);
    println!("initial state:\n{}\n{:?}", initial_state_input, stacks);
    let instructions = parse_instructions(instructions_input);
    println!(
        "instructions_input:\n{}\n{:?}",
        instructions_input, instructions
    );
    stacks = do_moves(stacks, instructions);
    println!("after moves:\n{:?}", stacks);
    println!("top letters: {:?}", get_tops(&stacks));
}
