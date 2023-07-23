use core::fmt;
use std::{error, fs, io, io::BufRead, path};

use crate::game;

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

pub struct GamesParser<'a> {
    file_name: &'a path::Path,
}

impl<'a> GamesParser<'a> {
    pub fn build(file_name: &'a str) -> Self {
        GamesParser {
            file_name: path::Path::new(file_name),
        }
    }

    pub fn try_get_games(&self) -> Result<Vec<game::Game>, GamesParsingError> {
        let file = self.try_open_file()?;
        let mut games = Vec::new();

        for line in file.lines() {
            let line_content = try_get_line_content(line)?;
            let line_content = line_content.trim();

            let splitted_line_content: Vec<&str> = line_content.split(' ').collect();
            let game = try_get_game(splitted_line_content)?;
            games.push(game);
        }

        Ok(games)
    }

    fn try_open_file(&self) -> Result<io::BufReader<fs::File>, GamesParsingError> {
        match fs::File::open(self.file_name) {
            Ok(file) => Ok(io::BufReader::new(file)),
            Err(err) => Err(GamesParsingError::build(format!(
                "could not open file {:?}, {}",
                self.file_name, err
            ))),
        }
    }
}

fn try_get_line_content(line: Result<String, io::Error>) -> Result<String, GamesParsingError> {
    match line {
        Ok(line) => Ok(line),
        Err(err) => Err(GamesParsingError::build(format!("error in file, {}", err))),
    }
}

fn try_get_game(splitted_line: Vec<&str>) -> Result<game::Game, GamesParsingError> {
    if splitted_line.len() != 2 {
        return Err(GamesParsingError::build(format!(
            "error in file for {:?}, expected 2 shapes, not {}",
            splitted_line,
            splitted_line.len()
        )));
    }

    let opponent_shape = get_shape_from_input(splitted_line[0])?;
    let own_shape = get_shape_from_input(splitted_line[1])?;

    Ok(game::Game::build(opponent_shape, own_shape))
}

fn get_shape_from_input(input: &str) -> Result<game::Shape, GamesParsingError> {
    match input {
        "A" | "X" => Ok(game::Shape::Rock),
        "B" | "Y" => Ok(game::Shape::Paper),
        "C" | "Z" => Ok(game::Shape::Scissors),
        s => Err(GamesParsingError::build(format!(
            "error in file, expected 'A', 'B', 'C', 'X', 'Y' or 'Z', not {}",
            s
        ))),
    }
}
