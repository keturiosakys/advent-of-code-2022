#[derive(Debug)]
struct Priority {
    character: Vec<char>,
    rank: Vec<i32>,
}

impl Priority {
    fn new() -> Self {
        let number_iter: Vec<i32> = (1..=52).collect();

        let mut character_iter: Vec<char> = (0..26).map(|c| (c + b'a') as char).collect();

        let mut uppercase_iter: Vec<char> = character_iter
            .iter()
            .map(|c| c.to_ascii_uppercase())
            .collect();

        character_iter.append(&mut uppercase_iter);

        return Priority {
            character: character_iter,
            rank: number_iter,
        };
    }
}

fn main() {
    let file_data = std::fs::read_to_string("./inputs/day3.txt").expect("Failed to read the file!");
    let parsed: Vec<&str> = file_data.split("\n").collect();

    let overlap = get_overlapping_score(&parsed);
    let badges = get_badges_score(&parsed);

    println!("Item overlap priority sum is {:?}", &overlap);
    println!("Badges priority sum is {:?}", &badges);
}

fn get_badges_score(rucksacks: &Vec<&str>) -> i32 {
    let badges: Vec<_> = rucksacks
        .chunks(3)
        .filter_map(|group| {
            if let [first, second, third] = group {
                let badge: Option<char> = first
                    .chars()
                    .find_map(|item| {
                        match (second.contains(item), third.contains(item)) {
                            (true, true) => return Some(item),
                            (true, false) => return None,
                            (false, true) => return None,
                            (false, false) => return None,
                        };
                    })
                    .take();

                return badge;
            } else {
                return None
            };
        })
        .collect();

    let score = score(badges);
    return score;
}

fn get_overlapping_score(rucksacks: &Vec<&str>) -> i32 {
    let overlap = rucksacks
        .iter()
        .filter_map(|rucksack| {
            let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

            let overlapping_item: Option<char> = first_compartment
                .chars()
                .find_map(|item| {
                    let position =
                        second_compartment.find(|item_second: char| return item == item_second);
                    match position {
                        Some(position) => {
                            let matching = second_compartment.chars().nth(position).unwrap();
                            return Some(matching);
                        }
                        None => return None,
                    }
                })
                .take();

            return overlapping_item;
        })
        .collect::<Vec<char>>();

    let score = score(overlap);

    return score;
}

fn score(items: Vec<char>) -> i32 {
    let priority = Priority::new();
    return items
        .iter()
        .map(|item| {
            let index = priority
                .character
                .iter()
                .position(|&c| c == *item)
                .expect("couldn't find matching char");

            let rank = priority
                .rank
                .get(index)
                .expect("Couldn't find matching rank");
            return rank;
        })
        .sum();
}
