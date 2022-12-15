#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Noop,
    Addx(isize),
}

use Instruction::*;

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            if line.eq("noop") {
                Noop
            } else if line.starts_with("addx ") {
                let (_, num) = line.split_at(5);
                Addx(num.parse().expect("unparseable addx value"))
            } else {
                panic!("unable to parse line {}", line)
            }
        })
        .collect()
}

struct CPU {
    instructions: Vec<Instruction>,
    reg_x: isize,
    cycle: usize,
    instruction: usize,
    processing: bool,
}

impl CPU {
    fn init(instructions: Vec<Instruction>) -> Self {
        CPU {
            instructions,
            reg_x: 1,
            cycle: 1,
            instruction: 0,
            processing: false,
        }
    }

    fn tick(self: &mut Self) -> Option<()> {
        let instruction = self.instructions.get(self.instruction)?;
        match instruction {
            Noop => {
                self.instruction = self.instruction + 1;
                self.processing = false;
            }
            Addx(value) => {
                if self.processing {
                    self.processing = false;
                    self.reg_x = self.reg_x + value;
                    self.instruction = self.instruction + 1;
                } else {
                    self.processing = true;
                }
            }
        }
        self.cycle = self.cycle + 1;
        Some(())
    }

    fn get_cumulative_signal_stregnths(self: &mut Self) -> isize {
        let mut cumlutative_strength = 0;
        while self.tick() == Some(()) {
            if self.cycle >= 20 && (self.cycle - 20) % 40 == 0 {
                let strength = self.reg_x * self.cycle as isize;
                println!(
                    "strength at cycle {} is {} (reg_x: {})",
                    self.cycle, strength, self.reg_x
                );
                cumlutative_strength = cumlutative_strength + strength;
            }
        }
        cumlutative_strength
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut cpu = CPU::init(parse(input));
    println!(
        "cumulative signal stregnth: {}",
        cpu.get_cumulative_signal_stregnths()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "noop
        addx 3
        addx -5";
        assert_eq!(parse(input), vec![Noop, Addx(3), Addx(-5)]);
    }

    #[test]
    fn test_simple() {
        let mut cpu = CPU::init(vec![Noop, Addx(3), Addx(-5)]);
        assert_eq!(cpu.reg_x, 1); // initial state
        cpu.tick(); // finish noop
        assert_eq!(cpu.reg_x, 1);
        cpu.tick(); // begin addx 3
        assert_eq!(cpu.reg_x, 1);
        cpu.tick(); // finish addx 3
        assert_eq!(cpu.reg_x, 4);
        cpu.tick(); // begin addx -5
        assert_eq!(cpu.reg_x, 4);
        cpu.tick(); // finish addx -5
        assert_eq!(cpu.reg_x, -1);
    }

    #[test]
    fn test_cm_str() {
        let input = include_str!("input-sample.txt");
        let mut cpu = CPU::init(parse(input));
        assert_eq!(cpu.get_cumulative_signal_stregnths(), 13140);
    }
}
