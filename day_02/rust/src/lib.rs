mod game;
mod parser;

use game::GameVariant;
use parser::GamesParser;

pub fn get_points_game_variant_1(file_name: &str) -> u32 {
    let mut game_parser = GamesParser::build(file_name, GameVariant::V1);
    let games = match game_parser.try_get_games() {
        Ok(games) => games,
        Err(_) => return 0,
    };

    games.iter().map(|game| game.get_points()).sum()
}

pub fn get_points_game_variant_2(file_name: &str) -> u32 {
    let mut game_parser = GamesParser::build(file_name, GameVariant::V2);
    let games = match game_parser.try_get_games() {
        Ok(games) => games,
        Err(_) => return 0,
    };

    games.iter().map(|game| game.get_points()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use game::result::Result;
    use game::shape::Shape;
    use game::Game;

    #[test]
    fn test_input_file_game_variant_1() {
        let file_name = "../input/test_input.txt";

        let mut game_parser = GamesParser::build(file_name, GameVariant::V1);
        let games = game_parser.try_get_games();

        let expected_games = vec![
            Game::build_v1(Shape::Rock, Shape::Paper),
            Game::build_v1(Shape::Paper, Shape::Rock),
            Game::build_v1(Shape::Scissors, Shape::Scissors),
        ];

        assert_eq!(games, Ok(expected_games));
    }

    #[test]
    fn test_input_file_result_game_variant_1() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_points_game_variant_1(file_name), 15);
    }

    #[test]
    fn test_input_file_game_variant_2() {
        let file_name = "../input/test_input.txt";

        let mut game_parser = GamesParser::build(file_name, GameVariant::V2);
        let games = game_parser.try_get_games();

        let expected_games = vec![
            Game::build_v2(Shape::Rock, Result::Tie),
            Game::build_v2(Shape::Paper, Result::Loss),
            Game::build_v2(Shape::Scissors, Result::Win),
        ];

        assert_eq!(games, Ok(expected_games));
    }

    #[test]
    fn test_input_file_result_game_variant_2() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_points_game_variant_2(file_name), 12);
    }
}
