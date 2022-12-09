#[derive(Debug)]
struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

impl Instruction {
    fn new(input: &str) -> Option<Self> {
        let vec: Vec<u8> = input
            .split(" ")
            .filter_map(|e| {
                if let Some(num) = e.parse::<u8>().ok() {
                    return Some(num);
                } else {
                    return None;
                }
            })
            .collect();

        if let [amount, from, to] = vec[..] {
            return Some(Instruction { amount, from, to });
        } else {
            return None;
        }
    }
}

fn parse_boxes(input: &str) -> Vec<Vec<char>> {
    let (boxes_input, _) = input.split_at(
        input
            .find("\n\n")
            .expect("Input data is corrupt"),
    );
    let mut boxes = vec![];

    let mut boxes_iter = boxes_input.lines().rev();
    let index_line = boxes_iter.next().expect("Input is corrupt");

    for stack_idx in index_line.chars() {
        if stack_idx.is_digit(10) {
            boxes.push(vec![])
        }
    }

    while let Some(line) = boxes_iter.next() {
        let mut line_iter = line.chars().skip(1).step_by(4);
        let mut idx = 0;
        while let Some(el) = line_iter.next() {
            if el.is_alphabetic() {
                boxes[idx].push(el);
            }
            idx += 1;
        }
    }

    return boxes;
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let (_, instructions_input) =
        input.split_at(input.find("\n\n").expect("Input data is corrupt"));
    return instructions_input
        .lines()
        .filter_map(|line| return Instruction::new(line))
        .collect();
}

fn main() {
    let file_data =
        std::fs::read_to_string("src/inputs/day5.txt").expect("Failed to read the file!");

    let top_crates_9000 = read_top_crates_one(&file_data);
    let top_crates_9001 = read_top_crates_two(&file_data);
    println!("Top crates 9000: {:?}", top_crates_9000);
    println!("Top crates 9001: {:?}", top_crates_9001);
}

fn read_top_crates_one(file_data: &str) -> String {
    let mut boxes = parse_boxes(file_data);
    let instructions = parse_instructions(file_data);

    for instr in instructions {
        let from_idx = instr.from - 1;
        let to_idx = instr.to - 1;
        let amount = instr.amount;

        for _ in 0..amount {
            let transport = boxes[from_idx as usize]
                .pop()
                .expect("No more boxes in stack!");
            boxes[to_idx as usize].push(transport);
        }
    }

    return boxes
        .iter()
        .filter_map(|stack| {
            if let Some(top_box) = stack.last() {
                return Some(top_box);
            } else {
                return None;
            };
        })
        .collect::<String>();
}

fn read_top_crates_two(file_data: &str) -> String {
    let mut boxes = parse_boxes(file_data);
    let instructions = parse_instructions(file_data);

    for instr in instructions {
        let from_idx = instr.from - 1;
        let to_idx = instr.to - 1;
        let amount = instr.amount;
        let mut transport = vec![];

        for _ in 0..amount {
            transport.push(boxes[from_idx as usize].pop().unwrap())
        }

        boxes[to_idx as usize].append(&mut transport.into_iter().rev().collect());
    }

    return boxes
        .iter()
        .filter_map(|stack| {
            if let Some(top_box) = stack.last() {
                return Some(top_box);
            } else {
                return None;
            };
        })
        .collect::<String>();
}

#[cfg(test)]

mod tests {

    use super::*;

    static TEST_INPUT: &str = include_str!("../test_inputs/day5.test.txt");

    #[test]
    fn top_crates_one() {
        assert_eq!(read_top_crates_one(TEST_INPUT), "CMZ");
    }

    #[test]
    fn top_crates_two() {
        assert_eq!(read_top_crates_two(TEST_INPUT), "MCD");
    }
}
