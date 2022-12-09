use std::collections::HashMap;

fn get_dir_sizes_cumlative(input: &str) -> HashMap<String, usize> {
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let mut path: Vec<&str> = Vec::new();
    for line in input.lines() {
        //println!("parsing '{}'", line);
        if line.starts_with("$ cd") {
            let dirname = line.split_at(5).1;
            if dirname == "/" {
                path.clear();
                path.push(dirname);
            } else if dirname == ".." {
                path.pop();
            } else {
                path.push(dirname);
            }
            //println!("path: {}", path.join("/"));
        } else {
            if let Some((left, _)) = line.split_once(" ") {
                if let Ok(file_size) = left.parse::<usize>() {
                    //println!("size {}", size);
                    for i in 0..path.len() {
                        let dir = path[0..=i].join("/");
                        //println!("part {}: {:?}", i, dir);
                        let dir_size = match dirs.get(&dir) {
                            Some(dir_size) => dir_size.to_owned(),
                            None => 0,
                        };
                        dirs.insert(dir, dir_size + file_size);
                    }
                }
                // else it's a ls command or a directory in ls results
            }
        }
    }
    dirs
}

fn sum_dirs_under_100k(dirs: HashMap<String, usize>) -> usize {
    let max: usize = 100000;
    dirs.values().filter(|&&size| size < max).sum()
}

fn main() {
    let input = include_str!("input.txt");

    let dir_sizes = get_dir_sizes_cumlative(input);
    println!(
        "total size of directories under 100k: {:?}",
        sum_dirs_under_100k(dir_sizes)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("input-sample.txt");

        let dir_sizes = get_dir_sizes_cumlative(input);
        assert_eq!(
            dir_sizes,
            HashMap::from([
                ("/".to_string(), 48381165),
                ("//a".to_string(), 94853),
                ("//d".to_string(), 24933642),
                ("//a/e".to_string(), 584)
            ])
        );
    }

    #[test]
    fn test_sizes() {
        let dirs: HashMap<String, usize> = HashMap::from([
            ("/".to_string(), 48381165),
            ("//a".to_string(), 94853),
            ("//d".to_string(), 24933642),
            ("//a/e".to_string(), 584),
        ]);
        assert_eq!(sum_dirs_under_100k(dirs), 95437);
    }
}
