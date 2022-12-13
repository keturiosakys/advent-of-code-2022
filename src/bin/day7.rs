// The whole tree data type is devised from this blog post:
// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

#[derive(Debug)]
struct Directory {
    parent: Option<usize>,
    size: Option<i32>,
    name: String,
}

impl Directory {
    fn new(dir: &str) -> Self {
        return Self {
            parent: None,
            size: None,
            name: dir.to_string(),
        };
    }
}

#[derive(Debug, Default)]
struct FileTree {
    nodes: Vec<Directory>,
}

impl FileTree {
    fn node(&mut self, pwd: usize, dir: &str) -> usize {
        if let Some(idx) = self.get_node_in_level(pwd, dir) {
            return idx;
        } else {
            return self.create_node(dir);
        };
    }

    fn get_node_in_level(&self, pwd: usize, dir: &str) -> Option<usize> {
        for (idx, node) in self.nodes.iter().enumerate() {
            if node.name == dir && node.parent == Some(pwd) {
                return Some(idx);
            } else {
                return None;
            }
        }
        return None;
    }

    fn get_node(&self, dir: &str) -> Option<usize> {
        for (idx, node) in self.nodes.iter().enumerate() {
            if node.name == dir {
                return Some(idx);
            } else {
                return None;
            }
        }
        return None;
    }

    fn create_node(&mut self, dir: &str) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Directory::new(dir));
        return idx;
    }

    fn add_size(&mut self, pwd: usize, size: i32) -> () {
        self.nodes[pwd].size = Some(size + self.nodes[pwd].size.unwrap_or(0));

        if let Some(value) = self.nodes[pwd].parent {
            self.add_size(value, size);
        }
    }
}

fn parse_input(input_data: &str) -> FileTree {
    let mut file_tree = FileTree::default();
    let mut pwd = 0;
    for mut line in input_data.lines() {
        line = line.strip_prefix("$ ").unwrap_or(line);
        let mut line_iter = line.split_whitespace();

        match (line_iter.next(), line_iter.next()) {
            (Some("cd"), Some("/")) => {
                pwd = file_tree.create_node("/");
            }

            (Some("cd"), Some("..")) => {
                pwd = file_tree.nodes[pwd]
                    .parent
                    .expect("Attempting to cd beyond root");
            }

            (Some("cd"), Some(dir)) => {
                let parent = pwd;
                pwd = file_tree.node(pwd, dir);
                file_tree.nodes[pwd].parent = Some(parent);
            }

            (Some("dir"), Some(_)) => continue,

            (Some(number), Some(_)) => {
                let size: i32 = number
                    .parse()
                    .expect("Attempting to parse a non-numeric value");
                file_tree.add_size(pwd, size);
            }

            _ => continue,
        }
    }

    return file_tree;
}

fn main() {
    let input_data =
        std::fs::read_to_string("src/inputs/day7.txt").expect("Failed to read the file!");

    let small_dir_size = calc_small(&input_data);
    let smallest_necessary = calc_smallest_necessary(&input_data);
    println!("{}", small_dir_size);
    println!("{}", smallest_necessary);
}

fn calc_smallest_necessary(input_data: &str) -> i32 {
    static TOTAL: i32 = 70000000;
    static UPDATE_SIZE: i32 = 30000000;
    let dirs = parse_input(input_data);
    let occupied = &dirs.nodes[dirs.get_node("/").unwrap()].size.unwrap();

    return dirs
        .nodes
        .iter()
        .filter_map(|node| {
            let remaining = occupied - node.size.unwrap();
            if remaining + UPDATE_SIZE <= TOTAL {
                return node.size;
            } else {
                return None;
            }
        })
        .min()
        .unwrap();
}

fn calc_small(input_data: &str) -> i32 {
    static LIMIT: i32 = 100000;
    let dirs = parse_input(input_data);

    return dirs
        .nodes
        .iter()
        .filter_map(|node| {
            if let Some(value) = node.size {
                if value < LIMIT {
                    return Some(node.size.unwrap());
                } else {
                    return None;
                }
            } else {
                return None;
            }
        })
        .sum::<i32>();
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &'static str = include_str!("../test_inputs/day7.test.txt");

    #[test]
    fn check_file_size() {
        assert_eq!(calc_small(TEST_INPUT), 95437);
    }

    #[test]
    fn check_smallest_deletable_dir() {
        assert_eq!(calc_smallest_necessary(TEST_INPUT), 24933642)
    }
}
