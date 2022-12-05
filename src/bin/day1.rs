use std::fs;

fn main() {
    let file_path = "src/inputs/day1.txt";

    let calories_input = fs::read_to_string(file_path).expect("Couldn't read file");

    let mut all_calory_loads: Vec<i32> = calories_input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .filter_map(|calory_value| calory_value.parse::<i32>().ok())
                .sum()
        })
        .collect();

    all_calory_loads.sort();
    let largest_calory_load: i32 = *all_calory_loads.iter().max().unwrap();
    let top_three_loads: i32 = all_calory_loads[all_calory_loads.len() - 1]
        + all_calory_loads[all_calory_loads.len() - 2]
        + all_calory_loads[all_calory_loads.len() - 3];

    dbg!(top_three_loads);
    dbg!(largest_calory_load);
}
