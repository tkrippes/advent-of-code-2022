use advent_of_code_2022_day_03::{get_sum_of_properties_1, get_sum_of_properties_2};

fn main() {
    let file_name = "../input/input.txt";

    println!(
        "The sum of the properties (part 1) is: {}",
        get_sum_of_properties_1(file_name)
    );

    println!(
        "The sum of the properties (part 2) is: {}",
        get_sum_of_properties_2(file_name)
    );
}
