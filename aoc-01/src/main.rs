use std::fs;

fn main() {
    let file_path = "calories_input.txt";

    let calories_input = fs::read_to_string(file_path).expect("Couldn't read file");

    let mut all_calory_loads: Vec<i32> = calories_input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .split('\n')
                .filter_map(|calory_value| {
                    calory_value
                        .parse::<i32>()
                        .ok()
                })
                .sum()
        }).collect();


    let largest_calory_load: i32 = all_calory_loads.iter().max().unwrap().clone();

    let mut top_three_loads: i32 = 0;
    let mut times: i32 = 0;

    while times < 3 {
        let current_largest: i32;
        current_largest = all_calory_loads.iter().max().unwrap().clone();

        top_three_loads += current_largest;
        all_calory_loads.retain(|load| *load != current_largest);
        times+= 1;
    };

    dbg!(top_three_loads);

    dbg!(largest_calory_load);
}
