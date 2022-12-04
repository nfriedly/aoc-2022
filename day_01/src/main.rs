

struct CaloriesList<> {
    elves: Vec<Vec<usize>>,
}

impl CaloriesList {
     fn parse(input: &str) -> CaloriesList {
        let mut elves: Vec<Vec<usize>> = Vec::new();
        let mut elf: Vec<usize> = Vec::new();
        for line in input.lines() {
            if line.is_empty() {
                elves.push(elf);
                elf = Vec::new();
            } else {
                let num: usize = line.trim().parse().expect("expecting a number or an empty line");
                elf.push(num);
            }
        }
        elves.push(elf);
        CaloriesList {
            elves,
        }
    }

    fn find_biggest_group_total(&self) -> usize {
        let mut biggest: usize = 0;
        for elf in self.elves.iter() {
            let mut cur_size: usize = 0;
            for item in elf.iter() {
                cur_size += item;
            }
            if cur_size > biggest {
                biggest = cur_size;
            }
        }
        biggest
    }
}

fn main() {
    let input = include_str!("input.txt");
    let calories_list: CaloriesList = CaloriesList::parse(&input);
    let biggest: usize = calories_list.find_biggest_group_total();
    println!("{}", biggest);
}
