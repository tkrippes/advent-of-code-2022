mod game;

use game::{parser::Parser, parser_functional, Game};

pub fn get_game_variant_1_points(file_name: &str) -> u32 {
    let mut game_variant_1_parser = Parser::build_v1(file_name);
    let games = match game_variant_1_parser.try_get_games() {
        Ok(games) => games,
        Err(_) => return 0,
    };

    get_games_points(games)
}

pub fn get_game_variant_1_points_functional(file_name: &str) -> u32 {
    let games = match parser_functional::try_get_games_variant_1(file_name) {
        Some(games) => games,
        None => return 0,
    };

    get_games_points(games)
}

pub fn get_game_variant_2_points(file_name: &str) -> u32 {
    let mut game_variant_2_parser = Parser::build_v2(file_name);
    let games = match game_variant_2_parser.try_get_games() {
        Ok(games) => games,
        Err(_) => return 0,
    };

    get_games_points(games)
}

pub fn get_game_variant_2_points_functional(file_name: &str) -> u32 {
    let games = match parser_functional::try_get_games_variant_2(file_name) {
        Some(games) => games,
        None => return 0,
    };

    get_games_points(games)
}

fn get_games_points(games: Vec<Game>) -> u32 {
    games.iter().map(|game| game.get_points()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_game_variant_1() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_game_variant_1_points(file_name), 15);
    }

    #[test]
    fn test_result_game_variant_1_functional() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_game_variant_1_points_functional(file_name), 15);
    }

    #[test]
    fn test_result_game_variant_2() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_game_variant_2_points(file_name), 12);
    }

    #[test]
    fn test_result_game_variant_2_functional() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_game_variant_2_points_functional(file_name), 12);
    }
}
