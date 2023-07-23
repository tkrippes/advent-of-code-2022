mod elf;
mod parser;

use std::path::Path;

pub fn get_max_calories_from_one_elf(input_file_name: &Path) -> u32 {
    let elves_parser = parser::ElvesParser::build(input_file_name);
    let elves = match elves_parser.try_get_elves() {
        Ok(elves) => elves,
        Err(_) => return 0,
    };

    elves.get_max_calories_of_one_elf()
}

pub fn get_max_calories_from_three_elves(input_file_name: &Path) -> u32 {
    let elves_parser = parser::ElvesParser::build(input_file_name);
    let elves = match elves_parser.try_get_elves() {
        Ok(elves) => elves,
        Err(_) => return 0,
    };

    elves.get_max_calories_of_three_elves()
}

#[cfg(test)]
mod tests {
    use crate::parser::ElvesParsingError;

    use super::*;

    fn build_elves(input_elves: Vec<elf::Elf>) -> elf::Elves {
        let mut elves = elf::Elves::new();
        for elf in input_elves {
            elves.push(elf);
        }
        elves
    }

    fn build_elf(input_calories: Vec<u32>) -> elf::Elf {
        let mut elf = elf::Elf::new();
        for calories in input_calories {
            elf.add_calories(calories);
        }
        elf
    }

    #[test]
    fn test_input_file_elves() {
        let file_name = Path::new("../input/test_input.txt");

        let elves_parser = parser::ElvesParser::build(file_name);
        let elves = elves_parser.try_get_elves();

        let expected_elves = build_elves(vec![
            build_elf(vec![1000, 2000, 3000]),
            build_elf(vec![4000]),
            build_elf(vec![5000, 6000]),
            build_elf(vec![7000, 8000, 9000]),
            build_elf(vec![10000]),
        ]);

        assert_eq!(elves, Ok(expected_elves));
    }

    #[test]
    fn test_non_existing_input_file_elves() {
        let file_name = Path::new("../input/wrong_test_input.txt");

        let elves_parser = parser::ElvesParser::build(file_name);
        let elves = elves_parser.try_get_elves();

        let expected_error = ElvesParsingError::build(format!(
            "could not open file {:?}, No such file or directory (os error 2)",
            file_name
        ));

        assert_eq!(elves, Err(expected_error));
    }

    #[test]
    fn test_input_file_result_1() {
        let test_input_file_name = Path::new("../input/test_input.txt");
        assert_eq!(get_max_calories_from_one_elf(test_input_file_name), 24000);
    }

    #[test]
    fn test_input_file_result_2() {
        let test_input_file_name = Path::new("../input/test_input.txt");
        assert_eq!(
            get_max_calories_from_three_elves(test_input_file_name),
            45000
        );
    }
}
