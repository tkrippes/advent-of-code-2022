mod compartment;

use std::{error, fmt};

use compartment::item::Item;
use compartment::{Compartment, CompartmentError};

#[derive(Debug, PartialEq)]
pub enum RucksackError {
    FirstCompartmentError { cause: String },
    SecondCompartmentError { cause: String },
}

impl fmt::Display for RucksackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RucksackError::FirstCompartmentError { cause } => {
                write!(f, "first compartment error, {}", cause)
            }
            RucksackError::SecondCompartmentError { cause } => {
                write!(f, "second compartment error, {}", cause)
            }
        }
    }
}

impl error::Error for RucksackError {}

impl RucksackError {
    fn build_first_compartment_error(err: CompartmentError) -> Self {
        RucksackError::FirstCompartmentError {
            cause: err.to_string(),
        }
    }

    fn build_second_compartment_error(err: CompartmentError) -> Self {
        RucksackError::SecondCompartmentError {
            cause: err.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}

impl Rucksack {
    pub fn try_build(ids: &str) -> Result<Self, RucksackError> {
        let (first_items_half, second_items_half) = ids.split_at(Self::get_mid_position(ids));

        let first_compartment = match Compartment::try_build(first_items_half) {
            Ok(first_compartment) => first_compartment,
            Err(err) => return Err(RucksackError::build_first_compartment_error(err)),
        };

        let second_compartment = match Compartment::try_build(second_items_half) {
            Ok(second_compartment) => second_compartment,
            Err(err) => return Err(RucksackError::build_second_compartment_error(err)),
        };

        Ok(Rucksack {
            first_compartment,
            second_compartment,
        })
    }

    fn get_mid_position(ids: &str) -> usize {
        if ids.len() % 2 == 0 {
            ids.len() / 2
        } else {
            ids.len() / 2 + 1
        }
    }

    pub fn get_first_common_item_of_compartments(&self) -> Option<&Item> {
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
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

        assert_eq!(rucksack.first_compartment.size(), 0);
        assert_eq!(rucksack.second_compartment.size(), 0);
    }

    #[test]
    fn build_rucksack_with_one_item() {
        let items = "c";
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

        assert_eq!(rucksack.first_compartment.size(), 1);
        assert_eq!(rucksack.second_compartment.size(), 0);

        assert!(rucksack.first_compartment.contains('c'))
    }

    #[test]
    fn build_rucksack_with_two_items() {
        let items = "ca";
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

        assert_eq!(rucksack.first_compartment.size(), 1);
        assert_eq!(rucksack.second_compartment.size(), 1);

        assert!(rucksack.first_compartment.contains('c'));
        assert!(rucksack.second_compartment.contains('a'));
    }

    #[test]
    fn build_rucksack_with_odd_number_of_items() {
        let items = "adflvmd";
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

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
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

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
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

        assert_eq!(rucksack.get_first_common_item_of_compartments(), None);
    }

    #[test]
    fn test_common_item_only_in_one_compartment() {
        let items = "aabbccdd";
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

        assert_eq!(rucksack.get_first_common_item_of_compartments(), None);
    }

    #[test]
    fn test_one_common_item() {
        let items = "abcdefgd";
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

        let first_common_item = rucksack.get_first_common_item_of_compartments();
        assert_ne!(first_common_item, None);
        assert_eq!(first_common_item.unwrap().get_id(), 'd');
    }

    #[test]
    fn test_two_common_items() {
        let items = "abcdecgb";
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

        let first_common_item = rucksack.get_first_common_item_of_compartments();
        assert_ne!(first_common_item, None);
        assert_eq!(first_common_item.unwrap().get_id(), 'b');
    }

    #[test]
    fn test_multiple_common_items() {
        let items = "abcdedba";
        let rucksack = Rucksack::try_build(items);

        assert!(rucksack.is_ok());
        let rucksack = rucksack.unwrap();

        let first_common_item = rucksack.get_first_common_item_of_compartments();
        assert_ne!(first_common_item, None);
        assert_eq!(first_common_item.unwrap().get_id(), 'a');
    }
}
