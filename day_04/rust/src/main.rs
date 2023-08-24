use advent_of_code_2022_day_03::{
    get_number_of_fully_contained_assignments, get_number_of_overlapping_assignments,
};

fn main() {
    let file_name = "../input/input.txt";
    let parsing_regex = r"^(\d+)+-(\d+)+,(\d+)+-(\d+)+$";

    println!(
        "The number of fully contained assignments is {}",
        get_number_of_fully_contained_assignments(file_name, parsing_regex)
    );

    println!(
        "The number of overlapping assignments is {}",
        get_number_of_overlapping_assignments(file_name, parsing_regex)
    )
}
