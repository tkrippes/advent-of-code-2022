use crate::game::result::Result as GameResult;
use crate::game::{shape::Shape, Game, GameVariant};
use core::fmt;
use std::{error, fs, io, io::BufRead, path};

#[derive(Debug, PartialEq)]
pub struct GamesParsingError {
    cause: String,
}

impl GamesParsingError {
    fn build(cause: String) -> Self {
        GamesParsingError { cause }
    }
}

impl fmt::Display for GamesParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse games, {}", self.cause)
    }
}

impl error::Error for GamesParsingError {}

pub struct GamesParser {
    file_name: String,
    current_line: String,
    current_line_index: usize,
    game_variant: GameVariant,
    games: Vec<Game>,
}

impl GamesParser {
    pub fn build(file_name: &str, game_variant: GameVariant) -> Self {
        GamesParser {
            file_name: String::from(file_name),
            current_line: String::new(),
            current_line_index: 1,
            game_variant,
            games: Vec::new(),
        }
    }

    pub fn try_get_games(&mut self) -> Result<Vec<Game>, GamesParsingError> {
        let file = self.try_open_file()?;

        for (index, line) in file.lines().enumerate() {
            self.try_set_current_line_and_index(line, index)?;
            self.try_add_game()?;
        }

        Ok(self.games.clone())
    }

    fn try_open_file(&self) -> Result<io::BufReader<fs::File>, GamesParsingError> {
        match fs::File::open(path::Path::new(&self.file_name)) {
            Ok(file) => Ok(io::BufReader::new(file)),
            Err(err) => Err(GamesParsingError::build(format!(
                "could not open file '{}', {}",
                self.file_name, err
            ))),
        }
    }

    fn try_set_current_line_and_index(
        &mut self,
        line: Result<String, io::Error>,
        index: usize,
    ) -> Result<(), GamesParsingError> {
        self.current_line = match line {
            Ok(line) => line.trim().to_string(),
            Err(err) => {
                return Err(GamesParsingError::build(format!(
                    "error in file, '{}' on line {}, {}",
                    self.file_name, self.current_line_index, err
                )))
            }
        };

        self.current_line_index = index + 1;

        Ok(())
    }

    fn try_add_game(&mut self) -> Result<(), GamesParsingError> {
        match self.game_variant {
            GameVariant::V1 => self.try_add_game_variant_1()?,
            GameVariant::V2 => self.try_add_game_variant_2()?,
        };

        Ok(())
    }

    fn try_add_game_variant_1(&mut self) -> Result<(), GamesParsingError> {
        let (opponent_shape, own_shape) = self.try_get_game_inputs()?;

        let opponent_shape = self.try_get_shape_from_input(opponent_shape)?;
        let own_shape = self.try_get_shape_from_input(own_shape)?;

        self.games.push(Game::build_v1(opponent_shape, own_shape));

        Ok(())
    }

    fn try_add_game_variant_2(&mut self) -> Result<(), GamesParsingError> {
        let (opponent_shape, result) = self.try_get_game_inputs()?;

        let opponent_shape = self.try_get_shape_from_input(opponent_shape)?;
        let result = self.try_get_result_from_input(result)?;

        self.games.push(Game::build_v2(opponent_shape, result));

        Ok(())
    }

    fn try_get_game_inputs(&self) -> Result<(&str, &str), GamesParsingError> {
        let splitted_line: Vec<&str> = self.current_line.split(' ').collect();

        match splitted_line.len() {
            2 => Ok((splitted_line[0], splitted_line[1])),
            _ => Err(GamesParsingError::build(format!(
                "error in file '{}' on line {}, expected 2 inputs, got {}",
                self.file_name,
                self.current_line_index,
                splitted_line.len()
            ))),
        }
    }

    fn try_get_shape_from_input(&self, input: &str) -> Result<Shape, GamesParsingError> {
        match self.game_variant {
            GameVariant::V1 => match input {
                "A" | "X" => Ok(Shape::Rock),
                "B" | "Y" => Ok(Shape::Paper),
                "C" | "Z" => Ok(Shape::Scissors),
                input => Err(GamesParsingError::build(format!(
                    "error in file '{}' on line {}, expected 'A', 'B', 'C', 'X', 'Y' or 'Z', not {}",
                    self.file_name, self.current_line_index, input
                ))),
            },
            GameVariant::V2 => match input {
                "A" => Ok(Shape::Rock),
                "B" => Ok(Shape::Paper),
                "C" => Ok(Shape::Scissors),
                input => Err(GamesParsingError::build(format!(
                    "error in file '{}' on line {}, expected 'A', 'B' or 'C', not {}",
                    self.file_name, self.current_line_index, input
                ))),
            }
        }
    }

    fn try_get_result_from_input(&self, input: &str) -> Result<GameResult, GamesParsingError> {
        match self.game_variant {
            GameVariant::V1 => Err(GamesParsingError::build(String::from(
                "error in parser, result parsing not supported for game variant 1",
            ))),
            GameVariant::V2 => match input {
                "X" => Ok(GameResult::Loss),
                "Y" => Ok(GameResult::Tie),
                "Z" => Ok(GameResult::Win),
                input => Err(GamesParsingError::build(format!(
                    "error in file '{}' on line {}, expected 'X', 'Y' or 'Z', not {}",
                    self.file_name, self.current_line_index, input
                ))),
            },
        }
    }
}
