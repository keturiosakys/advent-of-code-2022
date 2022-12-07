use std::cmp::Ordering::{Equal, Greater, Less};

type Assignment = (u8, u8);

struct Pair {
    first_assignment: Assignment,
    second_assignment: Assignment,
}

impl Pair {
    fn new(line: &str) -> Option<Self> {
        if let Some((first, second)) = line.split_once(",") {
            return Some(Pair {
                first_assignment: Self::parse_assignment(first),
                second_assignment: Self::parse_assignment(second),
            });
        } else {
            return None;
        }
    }

    fn flatten(&self) -> Vec<(u8, u8)> {
        return vec![self.first_assignment, self.second_assignment];
    }

    fn parse_assignment(assign_data: &str) -> Assignment {
        let (start_data, end_data) = assign_data.split_once("-").expect("no - found");
        let start = start_data.parse::<u8>().expect("not a u8");
        let end = end_data.parse::<u8>().expect("not a u8");

        return (start, end);
    }
}

fn main() {
    let file_data =
        std::fs::read_to_string("src/inputs/day4.txt").expect("Failed to read the file!");

    let pair_overlapping_count = calc_full_overlap(&file_data);
    let all_overlapping_count = calc_all_overlap(&file_data);
    println!("{:?}", pair_overlapping_count);
    println!("{:?}", all_overlapping_count);
}

fn parse_input(file_data: &str) -> Vec<Pair> {
    return file_data
        .lines()
        .filter_map(|line| return Pair::new(line))
        .collect::<Vec<Pair>>();
}

fn calc_full_overlap(input: &str) -> i32 {
    let assignments = parse_input(input);
    return assignments
        .iter()
        .filter_map(|pair| {
            let (first_start, first_end) = pair.first_assignment;
            let (second_start, second_end) = pair.second_assignment;

            // if the first pair range is smaller than the second pair range
            if first_end - first_start <= second_end - second_start {
                if is_fully_contained(first_start, first_end, second_start, second_end) {
                    return Some(pair);
                } else {
                    return None;
                };
            } else {
                if is_fully_contained(second_start, second_end, first_start, first_end) {
                    return Some(pair);
                } else {
                    return None;
                };
            }
        })
        .count() as i32;
}

fn is_fully_contained(
    smaller_start: u8,
    smaller_end: u8,
    larger_start: u8,
    larger_end: u8,
) -> bool {
    match (smaller_start >= larger_start, smaller_end <= larger_end) {
        (true, true) => return true,
        _ => return false,
    }
}

fn calc_all_overlap(input: &str) -> i32 {
    let assignments = parse_input(input);

    let mut intervals: Vec<Assignment> = assignments
        .iter()
        .filter_map(|pair| {
            let (first_start, first_end) = pair.first_assignment;
            let (second_start, second_end) = pair.second_assignment;

            if first_start <= second_end && second_start <= first_end {
                match (first_start.cmp(&second_start), first_end.cmp(&second_end)) {
                    (Less, Less) => return Some((first_start, second_end)),
                    (Less, Equal) => return Some((first_start, first_end)),
                    (Less, Greater) => return Some((first_start, first_end)),
                    (Equal, Less) => return Some((first_start, second_end)),
                    (Equal, Equal) => return Some((first_start, first_end)),
                    (Equal, Greater) => return Some((first_start, first_end)),
                    (Greater, Less) => return Some((second_start, first_end)),
                    (Greater, Equal) => return Some((second_start, second_end)),
                    (Greater, Greater) => return Some((second_start, second_end)),
                }
            } else {
                return None;
            }
        })
        .collect();

    intervals.sort_by_key(|interval| (interval.0, interval.1));

    let mut overlapping: Vec<Assignment> = vec![];

    let mut intervals_iter = intervals.iter();
    let first_interval = intervals_iter.next().expect("Got an empty iterator"); // first iterator WILL have value
    overlapping.push(*first_interval);
    let (_, end_first) = first_interval;
    let mut end_previous = end_first;

    for interval in intervals_iter {
        let (start_current, end_current) = interval;
        if start_current <= end_previous {
            end_previous = end_current;
            overlapping.push(*interval);
            continue;
        } else {
            end_previous = end_current;
            continue;
        }
    }

    return overlapping.iter().count() as i32;
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("../test_inputs/day4.test.txt");

    #[test]
    fn check_full_overlap() {
        assert_eq!(calc_full_overlap(TEST_INPUT), 2)
    }

    #[test]
    fn check_all_overlap() {
        assert_eq!(calc_all_overlap(TEST_INPUT), 4)
    }
}
