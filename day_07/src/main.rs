use std::{collections::HashMap};

type Directory = HashMap<String, Entity>;

enum Entity {
    File(usize),
    Dir(Directory),
}
use Entity::*;

fn get_directory(tree: Directory, path: Vec<&str>) -> Directory {
    tree
}

fn main() {
    let input = include_str!("input.txt");
    
    let mut cur_dir: Directory = HashMap::new();
    let mut tree = Dir(cur_dir);
    let mut cur_path: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dirname = line.split_at(5).1;
            if dirname == "/" {
                cur_path = Vec::new();
            } else if dirname == ".." {
                cur_path.pop();
            } else {
                cur_path.push(dirname);
            }
            println!("path: /{}", cur_path.join("/"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //     let input = "$ cd /
    // $ ls
    // dir a
    // 14848514 b.txt
    // 8504156 c.dat
    // dir d
    // $ cd a
    // $ ls
    // dir e
    // 29116 f
    // 2557 g
    // 62596 h.lst
    // $ cd e
    // $ ls
    // 584 i
    // $ cd ..
    // $ cd ..
    // $ cd d
    // $ ls
    // 4060174 j
    // 8033020 d.log
    // 5626152 d.ext
    // 7214296 k"

    // start of packet tests
    #[test]
    fn test_path_start() {
        assert_eq!(
            Path::from("/")
                .push("a")
                .push("b")
                .push("..")
                .push("c")
                .as_path(),
            Path::from("/a/c")
        );
    }
}
