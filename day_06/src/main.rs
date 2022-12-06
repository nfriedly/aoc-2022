use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let marker = find_marker(input);
    println!("first marker: {:?}", marker);
}

fn find_marker(input: &str) -> Option<usize> {
    const WINDOW_SIZE: usize = 4;
    let mut window: VecDeque<char> = VecDeque::with_capacity(WINDOW_SIZE);
    println!("initial window len: {}", window.len());
    let char_indices = input.char_indices();
    for (index, char) in char_indices {
        if window.len() >= WINDOW_SIZE {
            window.pop_front();
        }
        window.push_back(char);
        if window.len() == WINDOW_SIZE {
            let uniq = window.clone().into_iter().collect::<HashSet<_>>();
            if uniq.len() == WINDOW_SIZE {
                return Some(index + 1); // +1 because counting starts at 1, but indexing starts at 0
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mjq() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
    }
    #[test]
    fn test_bvw() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
    }
    #[test]
    fn test_npp() {
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
    }
    #[test]
    fn test_nzn() {
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
    }
    #[test]
    fn test_zcf() {
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }
}
