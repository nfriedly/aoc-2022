#[derive(Debug,Default,PartialEq)]
struct Point {
    x: u8,
    y: u8,
}
#[derive(Debug,PartialEq)]
struct Maze {
    start: Point,
    end: Point,
    map: Vec<Vec<u8>>,
}

fn get_height(letter: char) -> u8 {
    match letter {
        'S' => 1, //=a
        'E' => 26, //=z
        'a'..='z' => ((letter as u32) - 96) as u8, // 1-26
        _ => panic!("unexpected letter")
    }
}

fn parse(input: &str) -> Maze {
    let mut start = Point::default();
    let mut end = Point::default();
    let map = input.lines()
        .map(|l| l.trim())
        .enumerate()
        .map(|(y, l)| l.chars()
            .enumerate()
            .map(|(x, c)| {
                if c == 'S' {
                    start = Point { x: x as u8, y: y as u8 };
                } else if c == 'E' {
                    end = Point { x: x as u8, y: y as u8 };
                }
                get_height(c)
            }).collect::<Vec<u8>>()
        )
        .collect::<Vec<Vec<u8>>>();
    Maze {
        start,
        end,
        map,
    }
}

fn main() {
    let input = include_str!("input-sample.txt");
    let maze = parse(input);
    println!("{:?}", maze);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height() {
        assert_eq!(get_height('S'), 1);
        assert_eq!(get_height('a'), 1);
        assert_eq!(get_height('b'), 2);
        assert_eq!(get_height('z'), 26);
    }

    #[test]
    fn test_parse() {
        let input = "bS
        Ec";
        assert_eq!(parse(input), Maze {
            start: Point { x: 1, y: 0 },
            end: Point { x: 0, y: 1 },
            map: vec![vec![2,1], vec![26, 3]],
        });
    }
} 