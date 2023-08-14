mod parser;
mod rucksack;

use parser::Parser;

pub fn get_sum_of_properties(file_name: &str) -> u32 {
    let rucksack_parser = Parser::build(file_name);
    let rucksacks = rucksack_parser.get_rucksacks();

    match rucksacks {
        Ok(rucksacks) => rucksacks
            .iter()
            .filter_map(|rucksack| rucksack.get_first_common_item_of_compartments())
            .map(|item| item.get_priority())
            .sum(),
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result() {
        let file_name = "../input/test_input.txt";
        assert_eq!(get_sum_of_properties(file_name), 157);
    }
}
