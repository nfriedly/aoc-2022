use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: u8,
    y: u8,
}
#[derive(Debug, PartialEq)]
struct Maze {
    start: Point,
    end: Point,
    map: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PathNode {
    f_score: usize, // distance of shortest nown path to this point + "as the crow flies" distance to goal
    point: Point,
}

// todo: custom ordering so that smaller fScores come first and PathNodes are considered equal if fScore is equal

type Path = VecDeque<Point>;

fn parse_height(letter: char) -> u8 {
    match letter {
        'S' => 1,                                  //=a
        'E' => 26,                                 //=z
        'a'..='z' => ((letter as u32) - 96) as u8, // 1-26
        _ => panic!("unexpected letter"),
    }
}

fn parse(input: &str) -> Maze {
    let mut start = Point::default();
    let mut end = Point::default();
    let map = input
        .lines()
        .map(|l| l.trim())
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Point {
                            x: x as u8,
                            y: y as u8,
                        };
                    } else if c == 'E' {
                        end = Point {
                            x: x as u8,
                            y: y as u8,
                        };
                    }
                    parse_height(c)
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    Maze { start, end, map }
}

// find_path and reconstruct_path are based on wikipedia's a* pseudocode
fn reconstruct_path(came_from: &HashMap<Point, Point>, end: Point) -> Path {
    let mut total_path = Path::from([end]);
    let mut current = end;
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push_front(current);
    }
    total_path
}

fn height(point: &Point, maze: &Maze) -> u8 {
    maze.map[point.y as usize][point.x as usize]
}

fn distance_heuristic(current: &Point, maze: &Maze) -> usize {
    let goal = maze.end;
    let d_flat = (goal.x.abs_diff(current.x) + goal.y.abs_diff(current.y)) as usize;
    let d_height = 26 - height(&current, &maze) as usize;
    d_flat.max(d_height)
}

fn reachable_neighbors(current: &Point, map: &Vec<Vec<u8>>) -> Vec<Point> {
    let x_max = map[0].len() as isize;
    let y_max = map.len() as isize;
    let cur_height = map[current.y as usize][current.x as usize];
    // start with the deltas
    vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        // first ensure the new points will be on the map
        .filter(|(dx, dy)| {
            let x = current.x as isize + dx;
            let y = current.y as isize + dy;
            x >= 0 && y >= 0 && x < x_max && y < y_max
        })
        // then convert to actual points
        .map(|(dx, dy)| Point {
            x: (current.x as isize + dx) as u8,
            y: (current.y as isize + dy) as u8,
        })
        // finally, filter out any that are too high or too low
        .filter(|n| {
            let n_height = map[n.y as usize][n.x as usize];
            cur_height.abs_diff(n_height) <= 1
        })
        .collect::<Vec<Point>>()
}

// A* finds a path from start to goal.
// h is the heuristic function. h(n) estimates the cost to reach goal from node n.
fn find_path(maze: &Maze) -> Option<Path> {
    let start = maze.start;
    let goal = maze.end;
    let map = &maze.map;
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut open_set = BinaryHeap::from([Reverse(PathNode {
        f_score: distance_heuristic(&start, &maze),
        point: start,
    })]);

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from = HashMap::new();

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    // this is currently merged into openSet
    // let mut fScore = HashMap::new();
    // fScore.insert(start, distance_heuristic(start, goal));

    while !open_set.is_empty() {
        // This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
        let current = open_set.pop()?.0.point; // the node in openSet having the lowest fScore[] value
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }

        // println!("");
        // draw(reconstruct_path(&came_from, current), &maze);

        for neighbor in reachable_neighbors(&current, &map) {
            // d(current,neighbor) is the weight of the edge from current to neighbor, always 1 here
            // tentative_gScore is the distance from start to the neighbor through current
            let tentative_g_score = g_score[&current] + 1;
            if &tentative_g_score < g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                // This path to neighbor is better than any previous one. Record it!
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                // todo: figure out a better way to know if a given Point is in the openSet
                let mut in_open_set = false;
                for candicate in &open_set {
                    if candicate.0.point == neighbor {
                        in_open_set = true;
                        break;
                    }
                }
                if !in_open_set {
                    open_set.push(Reverse(PathNode {
                        f_score: tentative_g_score + distance_heuristic(&neighbor, &maze),
                        point: neighbor,
                    }))
                }
            }
        }
    }
    // Open set is empty but goal was never reached
    None
}

fn draw(path: Path, maze: &Maze) {
    let mut map = maze
        .map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| {
                    if x == maze.end.x as usize && y == maze.end.y as usize {
                        'E'
                    } else {
                        '.'
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    use std::cmp::Ordering::*;
    path.iter().reduce(|src, dst| {
        let graphic = match (dst.x.cmp(&src.x), src.y.cmp(&dst.y)) {
            (Equal, Greater) => '^',
            (Equal, Less) => 'V',
            (Greater, Equal) => '>',
            (Less, Equal) => '<',
            _ => panic!("invalid jump from {:?} to {:?}", src, dst),
        };
        // println!(
        //     "from ({},{}) to ({},{}): {}",
        //     src.x, src.y, dst.x, dst.y, graphic
        // );
        map[src.y as usize][src.x as usize] = graphic;
        dst
    });
    for row in map {
        for char in row {
            print!("{}", char);
        }
        println!("")
    }
}

fn main() {
    let input = include_str!("input.txt");
    let maze = parse(input);
    let path = find_path(&maze).unwrap();
    println!("steps in shortest path: {}", path.len() - 1);
    draw(path, &maze);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height() {
        assert_eq!(parse_height('S'), 1);
        assert_eq!(parse_height('a'), 1);
        assert_eq!(parse_height('b'), 2);
        assert_eq!(parse_height('z'), 26);
    }

    #[test]
    fn test_parse() {
        let input = "bS
        Ec";
        assert_eq!(
            parse(input),
            Maze {
                start: Point { x: 1, y: 0 },
                end: Point { x: 0, y: 1 },
                map: vec![vec![2, 1], vec![26, 3]],
            }
        );
    }

    #[test]
    fn test_shortest_path() {
        let input = include_str!("input-sample.txt");
        let maze = parse(input);
        assert_eq!(find_path(maze).unwrap().len() - 1, 31);
    }
}
