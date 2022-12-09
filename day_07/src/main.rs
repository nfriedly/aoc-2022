use std::collections::HashMap;

type Directory = HashMap<String, Entity>;

#[derive(Debug)]
enum Entity {
    File(usize),
    Dir(Directory),
}
fn get_directory<'a>(tree: &'a Directory, path: &Vec<&'a str>) -> &'a Directory {
    tree
}

fn parse(input: &str) -> Option<Directory> {
    let mut root: Directory = HashMap::new();
    let mut dir = &mut root;
    let mut path: Vec<&str> = Vec::new();
    for line in input.lines() {
        println!("parsing '{}'", line);
        use Entity::*;
        let words: Vec<&str> = line.split_whitespace().collect();
        if line.starts_with("$ cd") {
            let dirname = line.split_at(5).1;
            if dirname == "/" {
                path = Vec::new();
                dir = &mut root;
            } else if dirname == ".." {
                path.pop();
                dir = match dir.get_mut("..")? {
                    Dir(dir) => dir,
                    _ => &mut root,
                }
            } else {
                path.push(dirname);
                dir = match dir.get_mut("..")? {
                    Dir(dir) => dir,
                    _ => panic!("dir {} (/{}) not found", dirname, path.join("/")),
                }
            }
            println!("path: /{}", path.join("/"));
        } else if line.starts_with("$ ls") {
            // do nothing
            // dir = get_directory(&root, &path);
        } else if line.starts_with("dir ") {
            let name = line.split_at(4).1;
            let new_dir: Directory = HashMap::from([("..".to_string(), Dir(dir))]);
            dir.insert(name.to_string(), Dir(new_dir));
        } else {
            if let Ok(size) = words.get(0)?.parse() {
                dir.insert(words.get(1)?.to_string(), File(size));
            } else {
                println!("error parsing size");
                return None;
            }
        }
    }
    Some(root)
}

fn main() {
    let input = include_str!("input.txt");

    let root = parse(input);
    println!("parsed: {:?}", root);
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
