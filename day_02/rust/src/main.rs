use advent_of_code_2022_day_02::{
    get_game_variant_1_points, get_game_variant_1_points_functional, get_game_variant_2_points,
    get_game_variant_2_points_functional,
};

fn main() {
    let file_name = "../input/input.txt";

    println!(
        "Games variant 1 points: {}",
        get_game_variant_1_points(file_name)
    );

    println!(
        "Games variant 1 points (functional): {}",
        get_game_variant_1_points_functional(file_name)
    );

    println!(
        "Games variant 2 points: {}",
        get_game_variant_2_points(file_name)
    );

    println!(
        "Games variant 2 points (functional): {}",
        get_game_variant_2_points_functional(file_name)
    );
}
