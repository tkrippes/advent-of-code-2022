mod game;

use game::parser::Parser;

pub fn get_game_variant_1_points(file_name: &str) -> u32 {
    let mut game_variant_1_parser = Parser::build_v1(file_name);
    let games = match game_variant_1_parser.try_get_games() {
        Ok(games) => games,
        Err(_) => return 0,
    };

    games.iter().map(|game| game.get_points()).sum()
}

pub fn get_game_variant_2_points(file_name: &str) -> u32 {
    let mut game_variant_2_parser = Parser::build_v2(file_name);
    let games = match game_variant_2_parser.try_get_games() {
        Ok(games) => games,
        Err(_) => return 0,
    };

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
    fn test_result_game_variant_2() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_game_variant_2_points(file_name), 12);
    }
}
