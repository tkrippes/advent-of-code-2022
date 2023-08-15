use crate::rucksack::{Rucksack, RucksackError};

use std::{
    error, fmt, fs,
    io::{self, BufRead},
    path,
};

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    IOError { cause: String },
    ParsingRucksackError { line_index: usize, cause: String },
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::IOError { cause } => {
                write!(f, "parsing error, IO error, {}", cause)
            }
            ParsingError::ParsingRucksackError { line_index, cause } => {
                write!(
                    f,
                    "parsing error, rucksack error at line '{}', {}",
                    line_index, cause
                )
            }
        }
    }
}

impl error::Error for ParsingError {}

impl From<io::Error> for ParsingError {
    fn from(err: io::Error) -> Self {
        ParsingError::IOError {
            cause: err.to_string(),
        }
    }
}

impl ParsingError {
    fn build_parsing_rucksack_error(line_index: usize, err: RucksackError) -> Self {
        ParsingError::ParsingRucksackError {
            line_index,
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

    pub fn try_get_rucksacks(&self) -> Result<Vec<Rucksack>, ParsingError> {
        let file = self.try_open_file()?;

        let mut rucksacks = Vec::new();

        for (line_index, line) in file.lines().enumerate() {
            let line_content = self.try_get_line_content(line)?;
            let rucksack = self.try_build_rucksack(line_index, line_content)?;
            rucksacks.push(rucksack);
        }

        Ok(rucksacks)
    }

    fn try_open_file(&self) -> Result<io::BufReader<fs::File>, ParsingError> {
        match fs::File::open(path::Path::new(&self.file_name)) {
            Ok(file) => Ok(io::BufReader::new(file)),
            Err(err) => Err(err.into()),
        }
    }

    fn try_get_line_content(
        &self,
        line: Result<String, io::Error>,
    ) -> Result<String, ParsingError> {
        match line {
            Ok(line) => Ok(line.trim().to_string()),
            Err(err) => Err(err.into()),
        }
    }

    fn try_build_rucksack(
        &self,
        line_index: usize,
        line_content: String,
    ) -> Result<Rucksack, ParsingError> {
        match Rucksack::try_build(&line_content) {
            Ok(rucksack) => Ok(rucksack),
            Err(err) => Err(ParsingError::build_parsing_rucksack_error(
                line_index + 1,
                err,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file() {
        let file_name = "../input/test_input.txt";

        let rucksack_parser = Parser::build(file_name);
        let rucksacks = rucksack_parser.try_get_rucksacks();

        let expected_rucksacks = vec![
            Rucksack::try_build("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap(),
            Rucksack::try_build("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap(),
            Rucksack::try_build("PmmdzqPrVvPwwTWBwg").unwrap(),
            Rucksack::try_build("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap(),
            Rucksack::try_build("ttgJtRGJQctTZtZT").unwrap(),
            Rucksack::try_build("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap(),
        ];

        assert_eq!(rucksacks, Ok(expected_rucksacks));
    }

    #[test]
    fn test_missing_file() {
        let file_name = "../input/missing_test_input.txt";

        let rucksack_parser = Parser::build(file_name);
        let rucksacks = rucksack_parser.try_get_rucksacks();

        assert_eq!(
            rucksacks,
            Err(ParsingError::IOError {
                cause: String::from("No such file or directory (os error 2)")
            })
        );
    }

    #[test]
    fn test_invalid_file() {
        let file_name = "../input/invalid_test_input.txt";

        let rucksack_parser = Parser::build(file_name);
        let rucksacks = rucksack_parser.try_get_rucksacks();

        assert_eq!(
            rucksacks,
            Err(ParsingError::ParsingRucksackError {
                line_index: 4,
                cause: format!(
                    "second compartment error, {}, {}",
                    "compartment error at position '5'",
                    "invalid character error, should be ascii alphanumeric (a-z, A-Z), but was '!'",
                )
            })
        );
    }
}
