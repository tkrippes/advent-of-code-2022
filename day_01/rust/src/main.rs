use std::path::Path;

use advent_of_code_2022_day_01::get_calories_from_elf_with_most;

fn main() {
    let input_file_name = Path::new("../input/input.txt");
    println!(
        "Most calories carried by an elf: {:?}",
        get_calories_from_elf_with_most(input_file_name)
    );
}
