use crate::elf;
use std::{
    error, fmt, fs,
    io::{self, BufRead},
    path,
};

#[derive(Debug, PartialEq)]
pub struct ElvesParsingError {
    cause: String,
}

impl ElvesParsingError {
    pub fn build(cause: String) -> Self {
        ElvesParsingError { cause }
    }
}

impl error::Error for ElvesParsingError {}

impl fmt::Display for ElvesParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse elves, {}", self.cause)
    }
}

pub struct ElvesParser<'a> {
    file_name: &'a path::Path,
}

impl<'a> ElvesParser<'a> {
    pub fn build(file_name: &'a path::Path) -> Self {
        ElvesParser { file_name }
    }

    pub fn try_get_elves(&self) -> Result<elf::Elves, ElvesParsingError> {
        let file = self.try_open_file()?;
        let mut elves = elf::Elves::new();

        for (index, line) in file.lines().enumerate() {
            let line_content = try_get_line_content(line)?;
            let line_content = line_content.trim();

            add_elf_on_first_line(&mut elves, index + 1);

            if line_content.is_empty() {
                add_elf_if_current_elf_has_calories(&mut elves);
                continue;
            }

            let calories = try_get_calories(line_content)?;
            add_calories_to_current_elf(&mut elves, calories);
        }

        Ok(elves)
    }

    fn try_open_file(&self) -> Result<io::BufReader<fs::File>, ElvesParsingError> {
        match fs::File::open(self.file_name) {
            Ok(file) => Ok(io::BufReader::new(file)),
            Err(err) => Err(ElvesParsingError::build(format!(
                "could not open file {:?}, {}",
                self.file_name, err
            ))),
        }
    }
}

fn try_get_line_content(line: Result<String, io::Error>) -> Result<String, ElvesParsingError> {
    match line {
        Ok(line) => Ok(line),
        Err(err) => Err(ElvesParsingError::build(format!("error in file, {}", err))),
    }
}

fn add_elf_on_first_line(elves: &mut elf::Elves, line_number: usize) {
    if line_number == 1 {
        elves.push(elf::Elf::new());
    }
}

fn add_elf_if_current_elf_has_calories(elves: &mut elf::Elves) {
    if elves.has_last_elf_calories() {
        elves.push(elf::Elf::new());
    }
}

fn try_get_calories(line: &str) -> Result<u32, ElvesParsingError> {
    match line.parse::<u32>() {
        Ok(calories) => Ok(calories),
        Err(err) => Err(ElvesParsingError::build(format!(
            "error in file for '{}', {}",
            line, err
        ))),
    }
}

fn add_calories_to_current_elf(elves: &mut elf::Elves, calories: u32) {
    elves.add_calories_to_last_elf(calories);
}
