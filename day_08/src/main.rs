type Forrest = Vec<Vec<u8>>;
type VisibilityMap = Vec<Vec<bool>>;

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

use Side::*;

fn parse(input: &str) -> Forrest {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).expect("failed to parse char") as u8)
                .collect()
        })
        .collect()
}

fn calculate_visibility_pass(forrest: &Forrest, side: Side) -> VisibilityMap {
    let forrest_height = forrest.len();
    let forrest_width = forrest.get(0).expect("there must be at leastone row").len();

    // pre-fill the vec
    let mut result: VisibilityMap = Vec::with_capacity(forrest_height);
    for _ in 0..forrest_height {
        result.push(vec![false; forrest_width]);
    }

    let outer = match side {
        Left | Right => 0..forrest_height, // rows
        Top | Bottom => 0..forrest_width,  // cols
    };
    for i in outer {
        let mut max_height: i8 = -1;
        let inner: Box<dyn Iterator<Item = usize>> = match side {
            Left => Box::new(0..forrest_width),
            Right => Box::new((0..forrest_width).rev()),
            Top => Box::new(0..forrest_height),
            Bottom => Box::new((0..forrest_height).rev()),
        };

        for j in inner {
            let (x, y) = match side {
                Left | Right => (j, i),
                Top | Bottom => (i, j),
            };
            let height = forrest[y][x] as i8;
            // print!(
            //     "checking {} ({}, {}) against max_height {}: ",
            //     height, x, y, max_height
            // );
            let visible = height > max_height;
            if visible {
                max_height = height;
                // println!("visible");
            } else {
                // println!("hidden");
            };
            result[y][x] = visible;
        }
    }
    result
}

fn get_visibility(forrest: Forrest) -> VisibilityMap {
    let vis_left = calculate_visibility_pass(&forrest, Left);
    // println!("vis left {:?}", vis_left);
    let vis_right = calculate_visibility_pass(&forrest, Right);
    // println!("vis right {:?}", vis_right);
    let vis_top = calculate_visibility_pass(&forrest, Top);
    // println!("vis top {:?}", vis_top);
    let vis_bottom = calculate_visibility_pass(&forrest, Bottom);
    //println!("vis bottom {:?}", vis_bottom);

    combine_visibility(
        combine_visibility(vis_top, vis_bottom),
        combine_visibility(vis_left, vis_right),
    )
}

fn count_visible(map: VisibilityMap) -> usize {
    let mut count = 0;
    for row in map.into_iter() {
        for vis in row.into_iter() {
            if vis {
                count = count + 1;
            }
        }
    }
    count
}

// there's probably some cool matrix/vector/avx way to make this really fast and simple..
fn combine_visibility(a_map: VisibilityMap, b_map: VisibilityMap) -> VisibilityMap {
    a_map
        .into_iter()
        .zip(b_map.into_iter())
        .map(|(a_row, b_row)| {
            a_row
                .into_iter()
                .zip(b_row.into_iter())
                .map(|(a, b)| a || b)
                .collect()
        })
        .collect()
}

fn pretty_print_map(map: &VisibilityMap) {
    for row in map.into_iter() {
        for vis in row.into_iter() {
            if *vis {
                print!("1")
            } else {
                print!("0")
            }
        }
        println!("");
    }
}

fn main() {
    let input = include_str!("input.txt");
    let forrest = parse(input);
    //println!("parsed: {:?}", parse(input));
    let vis = get_visibility(forrest);
    pretty_print_map(&vis);
    println!("num visible: {}", count_visible(vis));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "
        123
        456
        789";
        assert_eq!(
            parse(input),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        );
    }

    #[test]
    fn test_vis_pass_top() {
        let input = "
        103
        406
        789";
        let forrest = parse(input);
        assert_eq!(
            calculate_visibility_pass(&forrest, Top),
            vec![
                vec![true, true, true],
                vec![true, false, true],
                vec![true, true, true]
            ]
        );
    }

    #[test]
    fn test_vis_pass_bottom() {
        let input = "
        123
        456
        789";
        let forrest = parse(input);
        assert_eq!(
            calculate_visibility_pass(&forrest, Bottom),
            vec![
                vec![false, false, false],
                vec![false, false, false],
                vec![true, true, true]
            ]
        )
    }
    #[test]
    fn test_vis_pass_left() {
        let input = "
        123
        456
        789";
        let forrest = parse(input);
        assert_eq!(
            calculate_visibility_pass(&forrest, Left),
            vec![
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true]
            ]
        );
    }
    #[test]
    fn test_vis_pass_right() {
        let input = "
        123
        456
        789";
        let forrest = parse(input);
        assert_eq!(
            calculate_visibility_pass(&forrest, Right),
            vec![
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, true]
            ]
        );
    }

    #[test]
    fn test_combine_visibility() {
        let a = vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![false, false, false],
        ];
        let b = vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, false, false],
        ];
        assert_eq!(
            combine_visibility(a, b),
            vec![
                vec![true, true, true],
                vec![true, false, false],
                vec![true, false, false]
            ]
        );
    }

    #[test]
    fn test_sample_input_count() {
        let input = include_str!("input-sample.txt");
        let forrest = parse(input);
        let vis = get_visibility(forrest);
        assert_eq!(count_visible(vis), 21);
    }
}
