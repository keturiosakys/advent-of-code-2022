#[derive(Debug)]
struct Forest {
    width: usize,
    height: usize,
    trees: Vec<Vec<i32>>,
}

impl Forest {
    fn new(input: &str) -> Self {
        let trees: Vec<Vec<i32>> = input
            .lines()
            .filter_map(|line| {
                return line
                    .chars()
                    .filter_map(|ch| {
                        return Some(ch.to_string().parse::<i32>().ok());
                    })
                    .collect();
            })
            .collect();
        let width = trees.iter().count();
        let mut trees_iter = trees.iter();
        let height = trees_iter.next().unwrap().iter().count();

        return Self {
            trees,
            width,
            height,
        };
    }

    fn is_tree_visible(&self, x: usize, y: usize, tree: i32) -> bool {
        let vertical: Vec<i32> = self.trees.iter().map(|line| line[x]).collect();
        let horizontal = &self.trees[y];

        let left = &horizontal[..x]; // this slice is not inclusive of x value
        let right = &horizontal[x + 1..]; // but this one is!!! WHY!!!!!
        let up = &vertical[..y];
        let down = &vertical[y + 1..];

        if left.is_empty() || up.is_empty() || right.is_empty() || down.is_empty() {
            return true;
        } else {
            match (
                left.iter().max() >= Some(&tree),
                right.iter().max() >= Some(&tree),
                up.iter().max() >= Some(&tree),
                down.iter().max() >= Some(&tree),
            ) {
                (true, true, true, true) => return false,
                _ => return true,
            }
        }
    }
}

fn main() {
    let input_data =
        std::fs::read_to_string("src/inputs/day8.txt").expect("Failed to read the file!");
    let visible_trees = find_visible_trees(&input_data);

    println!("Visible trees: {}", visible_trees);
}

fn find_visible_trees(input_data: &str) -> i32 {
    let forest = Forest::new(input_data);

    return forest
        .trees
        .iter()
        .enumerate()
        .map(|(y, line)| {
            return line
                .iter()
                .enumerate()
                .filter_map(|(x, tree)| {
                    if forest.is_tree_visible(x, y, *tree) {
                        return Some(tree);
                    } else {
                        return None;
                    }
                })
                .count() as i32;
        })
        .sum();
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &'static str = include_str!("../test_inputs/day8.test.txt");

    #[test]
    fn check_visible_trees() {
        assert_eq!(find_visible_trees(TEST_INPUT), 21);
    }
}
