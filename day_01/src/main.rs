

struct CaloriesList<> {
    elves: Vec<usize>,
}

impl CaloriesList {
     fn parse(input: &str) -> CaloriesList {
        let mut elves: Vec<usize> = Vec::new();
        let mut elf: usize = 0;
        for line in input.lines() {
            if line.is_empty() {
                elves.push(elf);
                elf = 0;
            } else {
                let num: usize = line.trim().parse().expect("expecting a number or an empty line");
                elf += num;
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
            if elf > &biggest {
                biggest = elf.clone();
            }
        }
        biggest
    }

    fn find_top_three(&self) -> usize {
        let mut sorted = self.elves.clone();
        sorted.sort();
        let third_place = sorted.len() - 3;
        let top_three = &sorted[third_place..];
        println!("top 3: {}, {}, {}", top_three[0], top_three[1], top_three[2]);

        let mut total: usize = 0;
        for elf in top_three.iter() {
            total += elf;
        }
        total
    }
}

fn main() {
    let input = include_str!("input.txt");
    let calories_list: CaloriesList = CaloriesList::parse(&input);
    let top_three: usize = calories_list.find_top_three();
    println!("{}", top_three);
}
