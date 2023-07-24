mod elf;
mod parser;

use parser::ElvesParser;

pub fn get_max_calories_from_one_elf(file_name: &str) -> u32 {
    let mut elves_parser = ElvesParser::build(file_name);
    let elves = match elves_parser.try_get_elves() {
        Ok(elves) => elves,
        Err(_) => return 0,
    };

    elves
        .iter()
        .map(|elf| elf.get_calories())
        .max()
        .unwrap_or_default()
}

pub fn get_max_calories_from_three_elves(file_name: &str) -> u32 {
    let mut elves_parser = ElvesParser::build(file_name);
    let elves = match elves_parser.try_get_elves() {
        Ok(elves) => elves,
        Err(_) => return 0,
    };

    let mut calories_list: Vec<u32> = elves.iter().map(|elf| elf.get_calories()).collect();
    calories_list.sort();
    calories_list.reverse();
    calories_list.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::elf::Elf;
    use crate::parser::{ElvesParser, ElvesParsingError};

    use super::*;

    fn build_elf(input_calories: Vec<u32>) -> Elf {
        let mut elf = Elf::new();
        for calories in input_calories {
            elf.add_calories(calories);
        }
        elf
    }

    #[test]
    fn test_input_file_elves() {
        let file_name = "../input/test_input.txt";

        let elves = ElvesParser::build(file_name).try_get_elves();

        let expected_elves = vec![
            build_elf(vec![1000, 2000, 3000]),
            build_elf(vec![4000]),
            build_elf(vec![5000, 6000]),
            build_elf(vec![7000, 8000, 9000]),
            build_elf(vec![10000]),
        ];

        assert_eq!(elves, Ok(expected_elves));
    }

    #[test]
    fn test_non_existing_input_file_elves() {
        let file_name = "../input/wrong_test_input.txt";

        let elves = ElvesParser::build(file_name).try_get_elves();

        let expected_error = ElvesParsingError::build(format!(
            "could not open file {}, No such file or directory (os error 2)",
            file_name
        ));

        assert_eq!(elves, Err(expected_error));
    }

    #[test]
    fn test_input_file_result_1() {
        let test_file_name = "../input/test_input.txt";
        assert_eq!(get_max_calories_from_one_elf(test_file_name), 24000);
    }

    #[test]
    fn test_input_file_result_2() {
        let test_file_name = "../input/test_input.txt";
        assert_eq!(get_max_calories_from_three_elves(test_file_name), 45000);
    }
}
