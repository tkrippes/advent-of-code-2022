use advent_of_code_2022_day_02::{get_points_game_variant_1, get_points_game_variant_2};

fn main() {
    let file_name = "../input/input.txt";

    println!(
        "Games points (Variant 1): {}",
        get_points_game_variant_1(file_name)
    );

    println!(
        "Games points (Variant 2): {}",
        get_points_game_variant_2(file_name)
    );
}
