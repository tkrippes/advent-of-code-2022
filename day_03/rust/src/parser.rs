use crate::rucksack::Rucksack;

use std::fs;
use std::io::{self, BufRead};
use std::path;

pub struct Parser {
    file_name: String,
}

impl Parser {
    pub fn build(file_name: &str) -> Parser {
        Parser {
            file_name: String::from(file_name),
        }
    }

    pub fn get_rucksacks(&self) -> Option<Vec<Rucksack>> {
        let file = self.try_open_file()?;

        let rucksacks = file
            .lines()
            .filter_map(|line| self.get_line(line))
            .map(|line| Rucksack::build(&line))
            .collect();

        Some(rucksacks)
    }

    fn try_open_file(&self) -> Option<io::BufReader<fs::File>> {
        match fs::File::open(path::Path::new(&self.file_name)) {
            Ok(file) => Some(io::BufReader::new(file)),
            Err(_) => None,
        }
    }

    fn get_line(&self, line: Result<String, io::Error>) -> Option<String> {
        match line {
            Ok(line) => Some(line.trim().to_string()),
            Err(_) => None,
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

        assert_eq!(rucksacks, Some(expected_rucksacks));
    }

    #[test]
    fn test_missing_file() {
        let file_name = "../input/wrong_test_input.txt";

        let rucksack_parser = Parser::build(file_name);
        let rucksacks = rucksack_parser.get_rucksacks();

        assert_eq!(rucksacks, None);
    }
}
