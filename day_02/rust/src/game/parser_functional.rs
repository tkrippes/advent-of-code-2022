use super::{result::Result as GameResult, shape::Shape, variant::Variant as GameVariant, Game};
use std::{
    fs,
    io::{self, BufRead},
    path,
};

pub fn try_get_games_variant_1(file_name: &str) -> Option<Vec<Game>> {
    try_get_games(file_name, GameVariant::V1)
}

pub fn try_get_games_variant_2(file_name: &str) -> Option<Vec<Game>> {
    try_get_games(file_name, GameVariant::V2)
}

fn try_get_games(file_name: &str, game_variant: GameVariant) -> Option<Vec<Game>> {
    let file = try_open_file(file_name)?;

    let games = file
        .lines()
        .map(try_get_line)
        .filter_map(|line| try_get_game(line, &game_variant))
        .collect();

    Some(games)
}

fn try_open_file(file_name: &str) -> Option<io::BufReader<fs::File>> {
    match fs::File::open(path::Path::new(file_name)) {
        Ok(file) => Some(io::BufReader::new(file)),
        Err(_) => None,
    }
}

fn try_get_line(line: Result<String, io::Error>) -> Option<String> {
    match line {
        Ok(line) => Some(line.trim().to_string()),
        Err(_) => None,
    }
}

fn try_get_game(line: Option<String>, game_variant: &GameVariant) -> Option<Game> {
    match line {
        Some(line) => match game_variant {
            GameVariant::V1 => try_get_game_variant_1(line),
            GameVariant::V2 => try_get_game_variant_2(line),
        },
        None => None,
    }
}

fn try_get_game_variant_1(line: String) -> Option<Game> {
    let (opponent_shape, own_shape) = try_get_game_inputs(line)?;

    let opponent_shape = try_get_shape_game_variant_1(&opponent_shape)?;
    let own_shape = try_get_shape_game_variant_1(&own_shape)?;

    Some(Game::build_v1(opponent_shape, own_shape))
}

fn try_get_game_variant_2(line: String) -> Option<Game> {
    let (opponent_shape, result) = try_get_game_inputs(line)?;

    let opponent_shape = try_get_shape_game_variant_2(&opponent_shape)?;
    let result = try_get_result_game_variant_2(&result)?;

    Some(Game::build_v2(opponent_shape, result))
}

fn try_get_game_inputs(line: String) -> Option<(String, String)> {
    let splitted_line: Vec<String> = line.split(' ').map(String::from).collect();

    match splitted_line.len() {
        2 => Some((splitted_line[0].to_owned(), splitted_line[1].to_owned())),
        _ => None,
    }
}

fn try_get_shape_game_variant_1(input: &str) -> Option<Shape> {
    match input {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

fn try_get_shape_game_variant_2(input: &str) -> Option<Shape> {
    match input {
        "A" => Some(Shape::Rock),
        "B" => Some(Shape::Paper),
        "C" => Some(Shape::Scissors),
        _ => None,
    }
}

fn try_get_result_game_variant_2(input: &str) -> Option<GameResult> {
    match input {
        "X" => Some(GameResult::Loss),
        "Y" => Some(GameResult::Tie),
        "Z" => Some(GameResult::Win),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure_game_variant_1() {
        let file_name = "../input/test_input.txt";

        let games = try_get_games_variant_1(file_name);

        let expected_games = vec![
            Game::build_v1(Shape::Rock, Shape::Paper),
            Game::build_v1(Shape::Paper, Shape::Rock),
            Game::build_v1(Shape::Scissors, Shape::Scissors),
        ];

        assert_eq!(games, Some(expected_games));
    }

    #[test]
    fn test_missing_input_file_game_variant_1() {
        let file_name = "../input/wrong_test_input.txt";

        let games = try_get_games_variant_1(file_name);

        assert_eq!(games, None);
    }

    #[test]
    fn test_structure_game_variant_2() {
        let file_name = "../input/test_input.txt";

        let games = try_get_games_variant_2(file_name);

        let expected_games = vec![
            Game::build_v2(Shape::Rock, GameResult::Tie),
            Game::build_v2(Shape::Paper, GameResult::Loss),
            Game::build_v2(Shape::Scissors, GameResult::Win),
        ];

        assert_eq!(games, Some(expected_games));
    }

    #[test]
    fn test_missing_input_file_game_variant_2() {
        let file_name = "../input/wrong_test_input.txt";

        let games = try_get_games_variant_2(file_name);

        assert_eq!(games, None);
    }
}
