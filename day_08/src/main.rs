type Forrest = Vec<Vec<u8>>;

fn parse(input: &str) -> Forrest {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).expect("failed to parse char") as u8)
                .collect()
        })
        .collect()
}

fn calculate_visibility_pass(forrest: &Forrest, rev_x: bool, rev_y: bool) -> Forrest {
    let iter_y: Box<dyn Iterator<Item = &Vec<u8>>> = {
        if rev_y {
            Box::new(forrest.iter().rev())
        } else {
            Box::new(forrest.iter())
        }
    };
    iter_y
        .map(|row| {
            let mut max_height: u8 = 0;
            let iter_x: Box<dyn Iterator<Item = &u8>> = {
                if rev_x {
                    Box::new(row.iter().rev())
                } else {
                    Box::new(row.iter())
                }
            };
            println!("beginning row {:?}", row);
            iter_x
                .map(|height| {
                    print!("checking {} against max_height {}: ", height, max_height);
                    if max_height == 0 || height > &max_height {
                        max_height = *height;
                        println!("visible");
                        return 1;
                    }
                    println!("hidden");
                    0
                })
                .collect()
        })
        .collect()
}

fn count_visible(forrest: Forrest) -> usize {
    let mut count: usize = 0;
    let vis_left = calculate_visibility_pass(&forrest, false, false);

    //let vis_right = calculate_visibility_pass(&forrest, true, false);

    println!("vis left {:?}", vis_left);

    count
}

fn main() {
    let input = include_str!("input-sample.txt");
    let forrest = parse(input);
    println!("parsed: {:?}", parse(input));
    count_visible(forrest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "123\n456\n789";
        assert_eq!(
            parse(input),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        );
    }
}
