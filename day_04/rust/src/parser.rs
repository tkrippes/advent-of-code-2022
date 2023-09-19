use crate::assignment_pair::AssignmentPair;

use std::{
    error, fmt, fs,
    io::{self, BufRead},
    path,
};

use regex::{Captures, Regex};

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    IOError { cause: String },
    ParsingAssignmentPairsError { line_index: usize, cause: String },
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::IOError { cause } => write!(f, "parsing error, IO error, {}", cause),
            ParsingError::ParsingAssignmentPairsError { line_index, cause } => {
                write!(f, "parsing error at line {}, {}", line_index, cause)
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
    fn build_parsing_assignment_pairs_error(line_index: usize, cause: String) -> Self {
        ParsingError::ParsingAssignmentPairsError { line_index, cause }
    }
}

pub struct Parser {
    file_name: String,
    parsing_regex: Regex,
}

impl Parser {
    pub fn build(file_name: &str, parsing_regex: &str) -> Self {
        Parser {
            file_name: String::from(file_name),
            parsing_regex: Regex::new(parsing_regex).unwrap(),
        }
    }

    pub fn try_get_assignment_pairs(&self) -> Result<Vec<AssignmentPair>, ParsingError> {
        let file = self.try_open_file()?;

        let mut assignment_pairs = Vec::new();

        for (line_index, line) in file.lines().enumerate() {
            let line_content = self.try_get_line_content(line)?;
            let assignment_pair = self.try_build_assignment_pair(line_index, &line_content)?;
            assignment_pairs.push(assignment_pair);
        }

        Ok(assignment_pairs)
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

    fn try_build_assignment_pair(
        &self,
        line_index: usize,
        line_content: &str,
    ) -> Result<AssignmentPair, ParsingError> {
        let captures = self.parsing_regex.captures(line_content);

        if captures.is_none() {
            return Err(ParsingError::build_parsing_assignment_pairs_error(
                line_index + 1,
                format!(
                    "regex '{}' could not find matches in '{}'",
                    self.parsing_regex, line_content
                ),
            ));
        }

        let (
            first_assignment_start_section_id,
            first_assignment_end_section_id,
            second_assignment_start_section_id,
            second_assignment_end_section_id,
        ) = self.try_parse_section_ids(captures, line_index, line_content)?;

        if first_assignment_start_section_id > first_assignment_end_section_id {
            return Err(ParsingError::build_parsing_assignment_pairs_error(
                line_index + 1,
                format!(
                    "start section of first assignment is greater than start section of first assignment in '{}'",
                    line_content
                ),
            ));
        }

        if second_assignment_start_section_id > second_assignment_end_section_id {
            return Err(ParsingError::build_parsing_assignment_pairs_error(
                line_index + 1,
                format!(
                    "start section of second assignment is greater than start section of second assignment in '{}'",
                    line_content
                ),
            ));
        }

        Ok(AssignmentPair::build(
            (
                first_assignment_start_section_id,
                first_assignment_end_section_id,
            ),
            (
                second_assignment_start_section_id,
                second_assignment_end_section_id,
            ),
        ))
    }

    fn try_parse_section_ids(
        &self,
        captures: Option<Captures<'_>>,
        line_index: usize,
        line_content: &str,
    ) -> Result<(u32, u32, u32, u32), ParsingError> {
        let Some((_, [first_assignment_start_section_id,
            first_assignment_end_section_id,
            second_assignment_start_section_id,
            second_assignment_end_section_id])) = captures.map(|caps| caps.extract())
            else { return Err(ParsingError::build_parsing_assignment_pairs_error(
                            line_index + 1,
                            format!(
                                "regex '{}' could not find exactly 4 matches in '{}'",
                                self.parsing_regex, line_content
                            ),
                        )) };

        let first_assignment_start_section_id =
            self.try_parse_section_id(first_assignment_start_section_id, line_index, line_content)?;
        let first_assignment_end_section_id =
            self.try_parse_section_id(first_assignment_end_section_id, line_index, line_content)?;
        let second_assignment_start_section_id = self.try_parse_section_id(
            second_assignment_start_section_id,
            line_index,
            line_content,
        )?;
        let second_assignment_end_section_id =
            self.try_parse_section_id(second_assignment_end_section_id, line_index, line_content)?;

        Ok((
            first_assignment_start_section_id,
            first_assignment_end_section_id,
            second_assignment_start_section_id,
            second_assignment_end_section_id,
        ))
    }

    fn try_parse_section_id(
        &self,
        section_id: &str,
        line_index: usize,
        line_content: &str,
    ) -> Result<u32, ParsingError> {
        match section_id.parse::<u32>() {
            Ok(section_id) => Ok(section_id),
            Err(err) => Err(ParsingError::build_parsing_assignment_pairs_error(
                line_index + 1,
                format!(
                    "could not parse section id '{}' to u32 in {}, {}",
                    section_id, line_content, err
                ),
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
        let parsing_regex = r"^(\d+)+-(\d+)+,(\d+)+-(\d+)+$";

        let assignment_parser = Parser::build(file_name, parsing_regex);
        let assignment_pairs = assignment_parser.try_get_assignment_pairs();

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
        let parsing_regex = r"^(\d+)+-(\d+)+,(\d+)+-(\d+)+$";

        let assignment_parser = Parser::build(file_name, parsing_regex);
        let assignment_pairs = assignment_parser.try_get_assignment_pairs();

        assert_eq!(
            assignment_pairs,
            Err(ParsingError::IOError {
                cause: String::from("No such file or directory (os error 2)")
            })
        );
    }

    #[test]
    fn test_invalid_file() {
        let file_name = "../input/invalid_test_input.txt";
        let parsing_regex = r"^(\d+)+-(\d+)+,(\d+)+-(\d+)+$";

        let assignment_parser = Parser::build(file_name, parsing_regex);
        let rucksacks = assignment_parser.try_get_assignment_pairs();

        assert_eq!(
            rucksacks,
            Err(ParsingError::ParsingAssignmentPairsError {
                line_index: 1,
                cause: format!(
                    "regex '{}' could not find matches in '{}'",
                    parsing_regex, "2-a,6-8"
                )
            })
        );
    }
}
