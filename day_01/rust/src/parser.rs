use crate::elf::Elf;
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

pub struct ElvesParser {
    file_name: String,
    current_line: String,
    current_line_index: usize,
    elves: Vec<Elf>,
}

impl ElvesParser {
    pub fn build(file_name: &str) -> Self {
        ElvesParser {
            file_name: String::from(file_name),
            current_line: String::new(),
            current_line_index: 1,
            elves: Vec::new(),
        }
    }

    pub fn try_get_elves(&mut self) -> Result<Vec<Elf>, ElvesParsingError> {
        let file = self.try_open_file()?;

        for (index, line) in file.lines().enumerate() {
            self.try_set_current_line_and_index(line, index)?;

            self.add_new_elf_if_current_line_is_first_line();

            if self.current_line.is_empty() {
                self.add_new_elf_if_current_elf_has_calories();
                continue;
            }

            self.try_add_calories_to_current_elf()?
        }

        Ok(self.elves.clone())
    }

    fn try_open_file(&self) -> Result<io::BufReader<fs::File>, ElvesParsingError> {
        match fs::File::open(path::Path::new(&self.file_name)) {
            Ok(file) => Ok(io::BufReader::new(file)),
            Err(err) => Err(ElvesParsingError::build(format!(
                "could not open file {}, {}",
                self.file_name, err
            ))),
        }
    }

    fn try_set_current_line_and_index(
        &mut self,
        line: Result<String, io::Error>,
        index: usize,
    ) -> Result<(), ElvesParsingError> {
        self.current_line = match line {
            Ok(line) => line.trim().to_string(),
            Err(err) => {
                return Err(ElvesParsingError::build(format!(
                    "error in file '{}' on line {}, {}",
                    self.file_name, self.current_line_index, err
                )))
            }
        };

        self.current_line_index = index + 1;

        Ok(())
    }

    fn add_new_elf_if_current_line_is_first_line(&mut self) {
        if self.current_line_index == 1 {
            self.elves.push(Elf::new());
        }
    }

    fn add_new_elf_if_current_elf_has_calories(&mut self) {
        if let Some(elf) = self.elves.last() {
            if elf.has_calories() {
                self.elves.push(Elf::new());
            }
        }
    }

    fn try_add_calories_to_current_elf(&mut self) -> Result<(), ElvesParsingError> {
        let calories = match self.current_line.parse::<u32>() {
            Ok(calories) => calories,
            Err(err) => {
                return Err(ElvesParsingError::build(format!(
                    "error in file '{}' on line {}, {}",
                    self.file_name, self.current_line_index, err
                )))
            }
        };

        if let Some(elf) = self.elves.last_mut() {
            elf.add_calories(calories);
        }

        Ok(())
    }
}
