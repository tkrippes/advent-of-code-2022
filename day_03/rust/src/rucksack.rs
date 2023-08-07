mod compartment;

use compartment::item::Item;
use compartment::Compartment;

struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}

impl Rucksack {
    fn build(items: &str) -> Self {
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

    fn get_first_common_item(&self) -> Option<Item> {
        todo!()
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
}
