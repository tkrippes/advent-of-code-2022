use super::{result::Result as GameResult, shape::Shape, variant::Variant as GameVariant, Game};
use std::{
    error, fmt, fs,
    io::{self, BufRead},
    path,
};

#[derive(Debug, PartialEq)]
pub struct ParsingError {
    cause: String,
}

impl ParsingError {
    pub fn build(cause: String) -> Self {
        ParsingError { cause }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse games, {}", self.cause)
    }
}

impl error::Error for ParsingError {}

pub struct Parser {
    file_name: String,
    current_line: String,
    current_line_index: usize,
    game_variant: GameVariant,
}

impl Parser {
    pub fn build_v1(file_name: &str) -> Self {
        Parser {
            file_name: String::from(file_name),
            current_line: String::new(),
            current_line_index: 0,
            game_variant: GameVariant::V1,
        }
    }

    pub fn build_v2(file_name: &str) -> Self {
        Parser {
            file_name: String::from(file_name),
            current_line: String::new(),
            current_line_index: 0,
            game_variant: GameVariant::V2,
        }
    }

    pub fn try_get_games(&mut self) -> Result<Vec<Game>, ParsingError> {
        let file = self.try_open_file()?;
        let mut games = Vec::new();

        for line in file.lines() {
            self.try_set_current_line_and_index(line)?;

            let game = self.try_get_game()?;
            games.push(game);
        }

        Ok(games)
    }

    fn try_open_file(&self) -> Result<io::BufReader<fs::File>, ParsingError> {
        match fs::File::open(path::Path::new(&self.file_name)) {
            Ok(file) => Ok(io::BufReader::new(file)),
            Err(err) => Err(ParsingError::build(format!(
                "could not open file '{}', {}",
                self.file_name, err
            ))),
        }
    }

    fn try_set_current_line_and_index(
        &mut self,
        line: Result<String, io::Error>,
    ) -> Result<(), ParsingError> {
        self.current_line_index += 1;

        self.current_line = match line {
            Ok(line) => line.trim().to_string(),
            Err(err) => {
                return Err(ParsingError::build(format!(
                    "error in file '{}' on line {}, {}",
                    self.file_name, self.current_line_index, err
                )))
            }
        };

        Ok(())
    }

    fn try_get_game(&mut self) -> Result<Game, ParsingError> {
        let game = match self.game_variant {
            GameVariant::V1 => self.try_get_game_variant_1()?,
            GameVariant::V2 => self.try_get_game_variant_2()?,
        };

        Ok(game)
    }

    fn try_get_game_variant_1(&mut self) -> Result<Game, ParsingError> {
        let (opponent_shape, own_shape) = self.try_get_game_inputs()?;

        let opponent_shape = self.try_get_shape_game_variant_1(opponent_shape)?;
        let own_shape = self.try_get_shape_game_variant_1(own_shape)?;

        let game = Game::build_v1(opponent_shape, own_shape);

        Ok(game)
    }

    fn try_get_game_variant_2(&mut self) -> Result<Game, ParsingError> {
        let (opponent_shape, result) = self.try_get_game_inputs()?;

        let opponent_shape = self.try_get_shape_game_variant_2(opponent_shape)?;
        let result = self.try_get_result_game_variant_2(result)?;

        let game = Game::build_v2(opponent_shape, result);

        Ok(game)
    }

    fn try_get_game_inputs(&self) -> Result<(&str, &str), ParsingError> {
        let splitted_line: Vec<&str> = self.current_line.split(' ').collect();

        match splitted_line.len() {
            2 => Ok((splitted_line[0], splitted_line[1])),
            _ => Err(ParsingError::build(format!(
                "error in file '{}' on line {}, expected 2 inputs, got {}",
                self.file_name,
                self.current_line_index,
                splitted_line.len()
            ))),
        }
    }

    fn try_get_shape_game_variant_1(&self, input: &str) -> Result<Shape, ParsingError> {
        match input {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            input => Err(ParsingError::build(format!(
                "error in file '{}' on line {}, expected 'A', 'B', 'C', 'X', 'Y' or 'Z' for shape, got {}",
                self.file_name, self.current_line_index, input
            ))),
        }
    }

    fn try_get_shape_game_variant_2(&self, input: &str) -> Result<Shape, ParsingError> {
        match input {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            input => Err(ParsingError::build(format!(
                "error in file '{}' on line {}, expected 'A', 'B' or 'C' for shape, got {}",
                self.file_name, self.current_line_index, input
            ))),
        }
    }

    fn try_get_result_game_variant_2(&self, input: &str) -> Result<GameResult, ParsingError> {
        match input {
            "X" => Ok(GameResult::Loss),
            "Y" => Ok(GameResult::Tie),
            "Z" => Ok(GameResult::Win),
            input => Err(ParsingError::build(format!(
                "error in file '{}' on line {}, expected 'X', 'Y' or 'Z' for result, got {}",
                self.file_name, self.current_line_index, input
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure_game_variant_1() {
        let file_name = "../input/test_input.txt";

        let mut game_variant_1_parser = Parser::build_v1(file_name);
        let games = game_variant_1_parser.try_get_games();

        let expected_games = vec![
            Game::build_v1(Shape::Rock, Shape::Paper),
            Game::build_v1(Shape::Paper, Shape::Rock),
            Game::build_v1(Shape::Scissors, Shape::Scissors),
        ];

        assert_eq!(games, Ok(expected_games));
    }

    #[test]
    fn test_missing_input_file_game_variant_1() {
        let file_name = "../input/wrong_test_input.txt";
        let mut game_variant_1_parser = Parser::build_v1(file_name);
        let games = game_variant_1_parser.try_get_games();

        assert_eq!(
            games,
            Err(ParsingError::build(format!(
                "could not open file '{}', No such file or directory (os error 2)",
                file_name,
            )))
        );
    }

    #[test]
    fn test_structure_game_variant_2() {
        let file_name = "../input/test_input.txt";

        let mut game_variant_2_parser = Parser::build_v2(file_name);
        let games = game_variant_2_parser.try_get_games();

        let expected_games = vec![
            Game::build_v2(Shape::Rock, GameResult::Tie),
            Game::build_v2(Shape::Paper, GameResult::Loss),
            Game::build_v2(Shape::Scissors, GameResult::Win),
        ];

        assert_eq!(games, Ok(expected_games));
    }

    #[test]
    fn test_missing_input_file_game_variant_2() {
        let file_name = "../input/wrong_test_input.txt";
        let mut game_variant_2_parser = Parser::build_v2(file_name);
        let games = game_variant_2_parser.try_get_games();

        assert_eq!(
            games,
            Err(ParsingError::build(format!(
                "could not open file '{}', No such file or directory (os error 2)",
                file_name,
            )))
        );
    }
}
