use crate::rucksack::Rucksack;

use std::{
    error::Error,
    fmt, fs,
    io::{self, BufRead},
    path,
};

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    IOError { cause: String },
    ParsingRucksackError,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::IOError { cause } => {
                write!(f, "Error while parsing, got IO error, cause: {}", cause)
            }
            ParsingError::ParsingRucksackError => {
                write!(f, "Error while parsing, got rucksack error")
            }
        }
    }
}

impl Error for ParsingError {}

impl From<io::Error> for ParsingError {
    fn from(err: io::Error) -> Self {
        ParsingError::IOError {
            cause: err.to_string(),
        }
    }
}

pub struct Parser {
    file_name: String,
}

impl Parser {
    pub fn build(file_name: &str) -> Parser {
        Parser {
            file_name: String::from(file_name),
        }
    }

    pub fn get_rucksacks(&self) -> Result<Vec<Rucksack>, ParsingError> {
        let file = self.try_open_file()?;

        let rucksacks_results: Vec<Result<Rucksack, ParsingError>> = file
            .lines()
            .map(|line| self.try_get_line(line))
            .map(|line| self.try_build_rucksack(line))
            .collect();

        let mut rucksacks = Vec::new();
        for result in rucksacks_results {
            match result {
                Ok(rucksack) => rucksacks.push(rucksack),
                Err(err) => return Err(err),
            };
        }

        Ok(rucksacks)
    }

    fn try_open_file(&self) -> Result<io::BufReader<fs::File>, ParsingError> {
        match fs::File::open(path::Path::new(&self.file_name)) {
            Ok(file) => Ok(io::BufReader::new(file)),
            Err(err) => Err(ParsingError::from(err)),
        }
    }

    fn try_get_line(&self, line: Result<String, io::Error>) -> Result<String, ParsingError> {
        match line {
            Ok(line) => Ok(line.trim().to_string()),
            Err(err) => Err(ParsingError::from(err)),
        }
    }

    fn try_build_rucksack(
        &self,
        line: Result<String, ParsingError>,
    ) -> Result<Rucksack, ParsingError> {
        match line {
            Ok(line) => Ok(Rucksack::build(&line)),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let file_name = "../input/test_input.txt";

        let rucksack_parser = Parser::build(file_name);
        let rucksacks = rucksack_parser.get_rucksacks();

        let expected_rucksacks = vec![
            Rucksack::build("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::build("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::build("PmmdzqPrVvPwwTWBwg"),
            Rucksack::build("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Rucksack::build("ttgJtRGJQctTZtZT"),
            Rucksack::build("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        assert_eq!(rucksacks, Ok(expected_rucksacks));
    }

    #[test]
    fn test_missing_file() {
        let file_name = "../input/wrong_test_input.txt";

        let rucksack_parser = Parser::build(file_name);
        let rucksacks = rucksack_parser.get_rucksacks();

        assert_eq!(
            rucksacks,
            Err(ParsingError::IOError {
                cause: String::from("No such file or directory (os error 2)")
            })
        );
    }
}
