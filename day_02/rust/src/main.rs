use advent_of_code_2022_day_02::{get_game_variant_1_points, get_game_variant_2_points};

fn main() {
    let file_name = "../input/input.txt";

    println!(
        "Games variant 1 points: {}",
        get_game_variant_1_points(file_name)
    );

    println!(
        "Games variant 2 points: {}",
        get_game_variant_2_points(file_name)
    );
}
