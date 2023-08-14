mod compartment;

use compartment::item::Item;
use compartment::Compartment;

#[derive(Debug, PartialEq)]
pub struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}

impl Rucksack {
    pub fn build(items: &str) -> Self {
        let half_position = if items.len() % 2 == 0 {
            items.len() / 2
        } else {
            items.len() / 2 + 1
        };
        let (first_items_half, second_items_half) = items.split_at(half_position);

        let first_compartment = Compartment::build(first_items_half);
        let second_compartment = Compartment::build(second_items_half);

        Rucksack {
            first_compartment,
            second_compartment,
        }
    }

    fn get_first_common_item_of_compartments(&self) -> Option<&Item> {
        self.first_compartment
            .get_first_common_item(&self.second_compartment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_rucksack_with_no_items() {
        let items = "";
        let rucksack = Rucksack::build(items);

        assert_eq!(rucksack.first_compartment.size(), 0);
        assert_eq!(rucksack.second_compartment.size(), 0);
    }

    #[test]
    fn build_rucksack_with_one_item() {
        let items = "c";
        let rucksack = Rucksack::build(items);

        assert_eq!(rucksack.first_compartment.size(), 1);
        assert_eq!(rucksack.second_compartment.size(), 0);

        assert!(rucksack.first_compartment.contains('c'))
    }

    #[test]
    fn build_rucksack_with_two_items() {
        let items = "ca";
        let rucksack = Rucksack::build(items);

        assert_eq!(rucksack.first_compartment.size(), 1);
        assert_eq!(rucksack.second_compartment.size(), 1);

        assert!(rucksack.first_compartment.contains('c'));
        assert!(rucksack.second_compartment.contains('a'));
    }

    #[test]
    fn build_rucksack_with_odd_number_of_items() {
        let items = "adflvmd";
        let rucksack = Rucksack::build(items);

        assert_eq!(rucksack.first_compartment.size(), 4);
        assert_eq!(rucksack.second_compartment.size(), 3);

        for item in "adfl".chars() {
            assert!(rucksack.first_compartment.contains(item));
        }
        for item in "vmd".chars() {
            assert!(rucksack.second_compartment.contains(item));
        }
    }

    #[test]
    fn build_rucksack_with_even_number_of_items() {
        let items = "aidjrmcnah";
        let rucksack = Rucksack::build(items);

        assert_eq!(rucksack.first_compartment.size(), 5);
        assert_eq!(rucksack.second_compartment.size(), 5);

        for item in "aidjr".chars() {
            assert!(rucksack.first_compartment.contains(item));
        }
        for item in "mcnah".chars() {
            assert!(rucksack.second_compartment.contains(item));
        }
    }

    #[test]
    fn test_no_common_item() {
        let items = "abcdefgh";
        let rucksack = Rucksack::build(items);

        assert_eq!(rucksack.get_first_common_item_of_compartments(), None);
    }

    #[test]
    fn test_common_item_only_in_one_compartment() {
        let items = "aabbccdd";
        let rucksack = Rucksack::build(items);

        assert_eq!(rucksack.get_first_common_item_of_compartments(), None);
    }

    #[test]
    fn test_one_common_item() {
        let items = "abcdefgd";
        let rucksack = Rucksack::build(items);

        let first_common_item = rucksack.get_first_common_item_of_compartments();
        assert_ne!(first_common_item, None);
        assert_eq!(first_common_item.unwrap().get_id(), 'd');
    }

    #[test]
    fn test_two_common_items() {
        let items = "abcdecgb";
        let rucksack = Rucksack::build(items);

        let first_common_item = rucksack.get_first_common_item_of_compartments();
        assert_ne!(first_common_item, None);
        assert_eq!(first_common_item.unwrap().get_id(), 'b');
    }

    #[test]
    fn test_multiple_common_items() {
        let items = "abcdedba";
        let rucksack = Rucksack::build(items);

        let first_common_item = rucksack.get_first_common_item_of_compartments();
        assert_ne!(first_common_item, None);
        assert_eq!(first_common_item.unwrap().get_id(), 'a');
    }
}
