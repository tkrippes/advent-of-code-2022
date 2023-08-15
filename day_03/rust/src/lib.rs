mod parser;
mod rucksack;

use parser::Parser;
use rucksack::Rucksack;

pub fn get_sum_of_properties_1(file_name: &str) -> u32 {
    let rucksack_parser = Parser::build(file_name);
    let rucksacks = rucksack_parser.try_get_rucksacks();

    match rucksacks {
        Ok(rucksacks) => rucksacks
            .iter()
            .filter_map(|rucksack| rucksack.get_first_common_item_of_compartments())
            .map(|item| item.get_priority())
            .sum(),
        Err(err) => {
            println!("Error while getting sum of properties, {}", err);
            0
        }
    }
}

pub fn get_sum_of_properties_2(file_name: &str) -> u32 {
    let rucksack_parser = Parser::build(file_name);
    let rucksacks = rucksack_parser.try_get_rucksacks();

    let rucksacks = match rucksacks {
        Ok(rucksacks) => rucksacks,
        Err(err) => {
            println!("Error while getting sum of properties, {}", err);
            return 0;
        }
    };

    let mut sum = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        let rucksack_1 = &rucksacks[i];
        let rucksack_2 = &rucksacks[i + 1];
        let rucksack_3 = &rucksacks[i + 2];

        let common_items = match Rucksack::get_common_items(rucksack_1, rucksack_2, rucksack_3) {
            Some(common_items) => common_items,
            None => continue,
        };

        for common_item in common_items {
            sum += common_item.get_priority();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file_1() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_sum_of_properties_1(file_name), 157);
    }

    #[test]
    fn test_missing_file_1() {
        let file_name = "../input/missing_test_input.txt";
        assert_eq!(get_sum_of_properties_1(file_name), 0);
    }

    #[test]
    fn test_invalid_file_1() {
        let file_name = "../input/invalid_test_input.txt";
        assert_eq!(get_sum_of_properties_1(file_name), 0);
    }

    #[test]
    fn test_valid_file_2() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_sum_of_properties_2(file_name), 70);
    }

    #[test]
    fn test_missing_file_2() {
        let file_name = "../input/missing_test_input.txt";
        assert_eq!(get_sum_of_properties_2(file_name), 0);
    }

    #[test]
    fn test_invalid_file_2() {
        let file_name = "../input/invalid_test_input.txt";
        assert_eq!(get_sum_of_properties_2(file_name), 0);
    }
}
