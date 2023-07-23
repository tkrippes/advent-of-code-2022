use std::path::Path;

use advent_of_code_2022_day_01::{
    get_max_calories_from_one_elf, get_max_calories_from_three_elves,
};

fn main() {
    let input_file_name = Path::new("../input/input.txt");
    println!(
        "Most calories carried by an elf: {:?}",
        get_max_calories_from_one_elf(input_file_name)
    );

    println!(
        "Most calories carried by three elves: {:?}",
        get_max_calories_from_three_elves(input_file_name)
    )
}
