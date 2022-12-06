use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    println!(
        "start of packet marker: {:?}",
        find_start_of_packet_marker(input)
    );
    println!(
        "start of message marker: {:?}",
        find_start_of_message_marker(input)
    );
}

fn find_start_of_packet_marker(input: &str) -> Option<usize> {
    find_marker(input, 4)
}

fn find_start_of_message_marker(input: &str) -> Option<usize> {
    find_marker(input, 14)
}

// finds the first substring of [size] unique characters and returns the position of the final character in that substring
fn find_marker(input: &str, size: usize) -> Option<usize> {
    let mut window: VecDeque<char> = VecDeque::with_capacity(size);
    let char_indices = input.char_indices();
    for (index, char) in char_indices {
        if window.len() >= size {
            window.pop_front();
        }
        window.push_back(char);
        if window.len() == size {
            let uniq = window.clone().into_iter().collect::<HashSet<_>>();
            if uniq.len() == size {
                return Some(index + 1); // +1 because counting starts at 1, but indexing starts at 0
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // start of packet tests
    #[test]
    fn test_mjq() {
        assert_eq!(
            find_start_of_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(7)
        );
    }
    #[test]
    fn test_bvw() {
        assert_eq!(
            find_start_of_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(5)
        );
    }
    #[test]
    fn test_npp() {
        assert_eq!(
            find_start_of_packet_marker("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(6)
        );
    }
    #[test]
    fn test_nzn() {
        assert_eq!(
            find_start_of_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(10)
        );
    }
    #[test]
    fn test_zcf() {
        assert_eq!(
            find_start_of_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(11)
        );
    }

    // start of message tests
    #[test]
    fn test_msg_mjq() {
        assert_eq!(
            find_start_of_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(19)
        );
    }
    #[test]
    fn test_msg_bvw() {
        assert_eq!(
            find_start_of_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(23)
        );
    }
    #[test]
    fn test_msg_npp() {
        assert_eq!(
            find_start_of_message_marker("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(23)
        );
    }
    #[test]
    fn test_msg_nzn() {
        assert_eq!(
            find_start_of_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(29)
        );
    }
    #[test]
    fn test_msg_zcf() {
        assert_eq!(
            find_start_of_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(26)
        );
    }
}
