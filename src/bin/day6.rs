fn main() {
    let input_data =
        std::fs::read_to_string("src/inputs/day6.txt").expect("Failed to read the file!");

    let marker = find_marker(&input_data, 4);
    let marker_14 = find_marker(&input_data, 14);
    println!("Start of packet 4: {:?}", marker);
    println!("Start of packet 14: {:?}", marker_14);
}

fn find_marker(input_data: &str, start_of_packet: u8) -> i32 {
    let messages = input_data.char_indices();
    let mut unique = vec![];
    let mut position = 0;

    for msg in messages {
        let (idx, byte) = msg;
        if !unique.contains(&byte) {
            unique.push(byte);
            if unique.len() == start_of_packet as usize {
                position = idx + 1;
                break;
            }
        } else {
            unique.push(byte);
            unique.drain(..=unique.iter().position(|i| *i == byte).unwrap());
        }
    }
    return position as i32;
}


#[cfg(test)]

mod tests {

    use super::*;

    static TEST_INPUT: &str = include_str!("../test_inputs/day6.test.txt");

    #[test]
    fn routine() {
        assert_eq!(find_marker(TEST_INPUT, 4), 10);
    }

    #[test]
    fn routine_14() {
        assert_eq!(find_marker(TEST_INPUT, 14), 29);
    }
}
