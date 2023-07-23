mod game;
mod parser;

pub fn get_games_points(file_name: &str) -> u32 {
    let game_parser = parser::GamesParser::build(file_name);
    let games = match game_parser.try_get_games() {
        Ok(games) => games,
        Err(_) => return 0,
    };

    games.iter().map(|game| game.get_points()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_file_games() {
        let file_name = "../input/test_input.txt";

        let game_parser = parser::GamesParser::build(file_name);
        let games = game_parser.try_get_games();

        let expected_games = vec![
            game::Game::build(game::Shape::Rock, game::Shape::Paper),
            game::Game::build(game::Shape::Paper, game::Shape::Rock),
            game::Game::build(game::Shape::Scissors, game::Shape::Scissors),
        ];

        assert_eq!(games, Ok(expected_games));
    }

    #[test]
    fn test_input_file_result() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_games_points(file_name), 15);
    }
}
