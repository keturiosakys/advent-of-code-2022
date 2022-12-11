use std::{collections::HashSet, fmt::Debug, ops::AddAssign};

// The whole tree data type is devised from this blog post:
// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

#[derive(Debug, Default)]
struct Tree<T>
where
    T: PartialEq,
{
    nodes: Vec<Node<T>>,
}

#[derive(Debug, Clone)]
struct Node<T> {
    idx: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    dir: T,
    size: i32,
}

impl<T> Node<T> {
    fn new(idx: usize, dir: T) -> Self {
        return Self {
            idx,
            dir,
            parent: None,
            children: vec![],
            size: 0,
        };
    }
}

impl<T> Tree<T>
where
    T: PartialEq + Debug,
{
    fn node(&mut self, dir: T) -> usize {
        for node in &self.nodes {
            if node.dir == dir {
                return node.idx;
            }
        }

        let idx = self.nodes.len();

        self.nodes.push(Node::new(idx, dir));
        return idx;
    }

    fn find_size(&self, idx: usize) -> i32 {
        let node = &self.nodes[idx];
        let mut size = node.size;

        println!("in find_size {:?}", node);

        for child in node.children.iter() {
            size += self.find_size(*child);
        }

        return size;
    }
}

fn main() {
    let input_data =
        std::fs::read_to_string("src/inputs/day7.txt").expect("Failed to read the file!");

    let small_dir_size = calc_small(&input_data);
    println!("{}", small_dir_size);
}

fn calc_small(input_data: &str) -> i32 {
    static LIMIT: i32 = 100000;
    let dirs = parse_input(input_data);
    dbg!(&dirs);
    let mut sized_dirs = vec![];

    for node in dirs.nodes.clone() {
        println!("{:?}", node);
        let size = dirs.find_size(node.idx);
        if size < LIMIT {
            sized_dirs.push(size)
        }
    }

    return sized_dirs.iter().sum();
}

fn parse_input(input_data: &str) -> Tree<String> {
    let mut file_tree = Tree::default();
    let mut curr_idx = 0;

    for mut line in input_data.lines() {
        line = line.strip_prefix("$ ").unwrap_or(line);
        let mut line_iter = line.split_whitespace();

        match (line_iter.next(), line_iter.next()) {
            (Some("dir"), Some(dir)) => {
                let dir_idx = file_tree.node(dir.to_owned());
                file_tree.nodes[dir_idx].parent = Some(curr_idx);
                file_tree.nodes[curr_idx].children.push(dir_idx);
            }
            (Some("cd"), Some("..")) => {
                if let Some(parent_idx) = file_tree.nodes[curr_idx].parent {
                    curr_idx = parent_idx;
                } else {
                    panic!("Attempting to index beyond root / ")
                }
            }
            (Some("cd"), Some("/")) => {
                curr_idx = file_tree.node("/".to_owned());
            }
            (Some("cd"), Some(dir)) => {
                curr_idx = file_tree.node(dir.to_owned());
            }
            (Some(number), Some(_)) => {
                let size = number.parse::<i32>().expect("Not a number");
                file_tree.nodes[curr_idx].size.add_assign(size);
            }
            _ => continue, //ignore the rest
        }
    }

    return file_tree;
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &'static str = include_str!("../test_inputs/day7.test.txt");

    #[test]
    fn check_file_size() {
        assert_eq!(calc_small(TEST_INPUT), 95437);
    }
}
