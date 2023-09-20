mod error;

use error::Error;

use crate::assignment_pair::AssignmentPair;

use std::{
    fs,
    io::{self, BufRead},
    path,
};

use regex::Regex;

type Assignment = (u32, u32);

pub struct Parser {
    file_name: String,
    parsing_regex: Regex,
}

impl Parser {
    pub fn build(file_name: &str) -> Self {
        Parser {
            file_name: String::from(file_name),
            parsing_regex: Regex::new(r"^(\d+)+-(\d+)+,(\d+)+-(\d+)+$").unwrap(),
        }
    }

    pub fn try_parse_assignment_pairs(&self) -> Result<Vec<AssignmentPair>, Error> {
        let file = self.try_open_file()?;

        let mut assignment_pairs = Vec::new();

        for (line_index, line) in file.lines().enumerate() {
            let line_content = self.try_get_line_content(line)?;
            let assignment_pair = self.try_parse_assignment_pair(line_index, &line_content)?;
            assignment_pairs.push(assignment_pair);
        }

        Ok(assignment_pairs)
    }

    fn try_open_file(&self) -> Result<io::BufReader<fs::File>, Error> {
        match fs::File::open(path::Path::new(&self.file_name)) {
            Ok(file) => Ok(io::BufReader::new(file)),
            Err(err) => Err(err.into()),
        }
    }

    fn try_get_line_content(&self, line: Result<String, io::Error>) -> Result<String, Error> {
        match line {
            Ok(line) => Ok(line.trim().to_string()),
            Err(err) => Err(err.into()),
        }
    }

    fn try_parse_assignment_pair(
        &self,
        line_index: usize,
        line_content: &str,
    ) -> Result<AssignmentPair, Error> {
        let (first_assignment, second_assignment) =
            self.try_parse_assignments(line_index, line_content)?;

        self.validate_assignments(
            first_assignment,
            second_assignment,
            line_index,
            line_content,
        )?;

        Ok(AssignmentPair::build(first_assignment, second_assignment))
    }

    fn try_parse_assignments(
        &self,
        line_index: usize,
        line_content: &str,
    ) -> Result<(Assignment, Assignment), Error> {
        let sections = self.try_parse_sections(line_index, line_content)?;

        let first_assignment = (
            self.try_parse_section(sections[0], line_index, line_content)?,
            self.try_parse_section(sections[1], line_index, line_content)?,
        );

        let second_assignment = (
            self.try_parse_section(sections[2], line_index, line_content)?,
            self.try_parse_section(sections[3], line_index, line_content)?,
        );

        Ok((first_assignment, second_assignment))
    }

    fn try_parse_sections<'a>(
        &self,
        line_index: usize,
        line_content: &'a str,
    ) -> Result<[&'a str; 4], Error> {
        let (_, sections) = match self.parsing_regex.captures(line_content) {
            Some(captures) => captures.extract(),
            None => {
                return Err((
                    line_index + 1,
                    format!(
                        "regex '{}' could not find matches in '{}'",
                        self.parsing_regex, line_content
                    ),
                )
                    .into())
            }
        };

        Ok(sections)
    }

    fn try_parse_section(
        &self,
        section: &str,
        line_index: usize,
        line_content: &str,
    ) -> Result<u32, Error> {
        match section.parse::<u32>() {
            Ok(section) => Ok(section),
            Err(err) => Err((
                line_index + 1,
                format!(
                    "could not parse section '{}' to u32 in {}, {}",
                    section, line_content, err
                ),
            )
                .into()),
        }
    }

    fn validate_assignments(
        &self,
        first_assignment: Assignment,
        second_assignment: Assignment,
        line_index: usize,
        line_content: &str,
    ) -> Result<(), Error> {
        if first_assignment.0 > first_assignment.1 {
            return Err((
                line_index + 1,
                format!(
                    "start section of first assignment is greater than start section of first assignment in '{}'",
                    line_content
                ),
            ).into());
        }

        if second_assignment.0 > second_assignment.1 {
            return Err((
                line_index + 1,
                format!(
                    "start section of second assignment is greater than start section of second assignment in '{}'",
                    line_content
                ),
            ).into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file() {
        let file_name = "../input/test_input.txt";

        let assignment_parser = Parser::build(file_name);
        let assignment_pairs = assignment_parser.try_parse_assignment_pairs();

        let expected_rucksacks = vec![
            AssignmentPair::build((2, 4), (6, 8)),
            AssignmentPair::build((2, 3), (4, 5)),
            AssignmentPair::build((5, 7), (7, 9)),
            AssignmentPair::build((2, 8), (3, 7)),
            AssignmentPair::build((6, 6), (4, 6)),
            AssignmentPair::build((2, 6), (4, 8)),
        ];

        assert_eq!(assignment_pairs, Ok(expected_rucksacks));
    }

    #[test]
    fn test_missing_file() {
        let file_name = "../input/missing_test_input.txt";

        let assignment_parser = Parser::build(file_name);
        let assignment_pairs = assignment_parser.try_parse_assignment_pairs();

        assert_eq!(
            assignment_pairs,
            Err(Error::IOError {
                cause: String::from("No such file or directory (os error 2)")
            })
        );
    }

    #[test]
    fn test_invalid_file() {
        let file_name = "../input/invalid_test_input.txt";

        let assignment_parser = Parser::build(file_name);
        let rucksacks = assignment_parser.try_parse_assignment_pairs();

        assert_eq!(
            rucksacks,
            Err(Error::ParsingAssignmentPairsError {
                line_index: 1,
                cause: format!(
                    "regex '{}' could not find matches in '{}'",
                    assignment_parser.parsing_regex, "2-a,6-8"
                )
            })
        );
    }
}
