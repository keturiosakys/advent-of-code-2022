#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<i32>>,
}

#[derive(Debug, Clone, Copy)]
enum Sight<'a> {
    Up(&'a [i32]),
    Left(&'a [i32]),
    Right(&'a [i32]),
    Down(&'a [i32]),
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

        return Self { trees };
    }

    fn best_scenic_view(&self, x: usize, y: usize, tree: i32) -> i32 {
        let vertical: Vec<i32> = self.trees.iter().map(|line| line[x]).collect();
        let horizontal = &self.trees[y];

        let up = Sight::Up(&vertical[..y]);
        let left = Sight::Left(&horizontal[..x]);
        let right = Sight::Right(&horizontal[x + 1..]);
        let down = Sight::Down(&vertical[y + 1..]);

        let scenic_score = visible_distance(left, &tree)
            * visible_distance(right, &tree)
            * visible_distance(up, &tree)
            * visible_distance(down, &tree);

        return scenic_score;
    }

    fn is_tree_visible(&self, x: usize, y: usize, tree: i32) -> bool {
        let vertical: Vec<i32> = self.trees.iter().map(|line| line[x]).collect();
        let horizontal = &self.trees[y];

        let up = &vertical[..y];
        let left = &horizontal[..x];
        let right = &horizontal[x + 1..];
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

/// Function that given a line of sight calculates the distance you're able to see.
/// Reverses if Up or Left slices are passed
fn visible_distance(sight: Sight, tree: &i32) -> i32 {
    match sight {
        Sight::Up(line) | Sight::Left(line) => {
            if let Some(value) = line.iter().rev().position(|t| t >= tree) {
                return value as i32 + 1;
            } else {
                return line.len() as i32;
            }
        }
        Sight::Right(line) | Sight::Down(line) => {
            if let Some(value) = line.iter().position(|t| t >= tree) {
                return value as i32 + 1;
            } else {
                return line.len() as i32;
            }
        }
    }
}

fn main() {
    let input_data =
        std::fs::read_to_string("src/inputs/day8.txt").expect("Failed to read the file!");
    let visible_trees = find_visible_trees(&input_data);
    let scenic_score = highest_scenic_score(&input_data);

    println!("Visible trees: {}", visible_trees);
    println!("Highest scenic score: {}", scenic_score);
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

fn highest_scenic_score(input_data: &str) -> i32 {
    let forest = Forest::new(input_data);

    return forest
        .trees
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, tree)| {
                    return forest.best_scenic_view(x, y, *tree);
                })
                .max()
        })
        .max()
        .unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &'static str = include_str!("../test_inputs/day8.test.txt");

    #[test]
    fn check_visible_trees() {
        assert_eq!(find_visible_trees(TEST_INPUT), 21);
    }

    #[test]
    fn check_scenic_score() {
        assert_eq!(highest_scenic_score(TEST_INPUT), 8);
    }
}
