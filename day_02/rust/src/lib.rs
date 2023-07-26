mod game;
mod parser;

use parser::GamesParser;

pub fn get_games_points(file_name: &str) -> u32 {
    let game_parser = GamesParser::build(file_name);
    let games = match game_parser.try_get_games() {
        Ok(games) => games,
        Err(_) => return 0,
    };

    games.iter().map(|game| game.get_points()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use game::shape::Shape;
    use game::Game;

    #[test]
    fn test_input_file_games_1() {
        let file_name = "../input/test_input.txt";

        let game_parser = GamesParser::build(file_name);
        let games = game_parser.try_get_games();

        let expected_games = vec![
            Game::build_v1(Shape::Rock, Shape::Paper),
            Game::build_v1(Shape::Paper, Shape::Rock),
            Game::build_v1(Shape::Scissors, Shape::Scissors),
        ];

        assert_eq!(games, Ok(expected_games));
    }

    #[test]
    fn test_input_file_result_1() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_games_points(file_name), 15);
    }
}
