pub mod item;

use item::{InvalidCharacterError, Item};
use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub struct CompartmentError {
    position: usize,
    cause: String,
}

impl fmt::Display for CompartmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid compartment, error at position {}, cause: {}",
            self.position, self.cause
        )
    }
}

impl error::Error for CompartmentError {}

impl CompartmentError {
    fn build(position: usize, err: InvalidCharacterError) -> Self {
        CompartmentError {
            position,
            cause: err.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Compartment {
    items: Vec<Item>,
}

impl Compartment {
    pub fn try_build(items: &str) -> Result<Self, CompartmentError> {
        let items_results: Vec<Result<Item, InvalidCharacterError>> =
            items.chars().map(Item::try_build).collect();

        let mut items = Vec::new();
        for (position, result) in items_results.into_iter().enumerate() {
            match result {
                Ok(item) => items.push(item),
                Err(err) => return Err(CompartmentError::build(position, err)),
            }
        }

        Ok(Compartment { items })
    }

    pub fn get_first_common_item(&self, other: &Compartment) -> Option<&Item> {
        self.items
            .iter()
            .find(|&item| other.contains(item.get_id()))
    }

    pub fn contains(&self, item_id: char) -> bool {
        self.items.iter().any(|item| item.get_id() == item_id)
    }

    // only used in unit tests
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.items.len()
    }

    // only used in unit tests
    #[allow(dead_code)]
    fn get_number_of_items_with_id(&self, item_id: char) -> usize {
        self.items
            .iter()
            .filter(|item| item.get_id() == item_id)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_compartment_from_nothing() {
        let items = "";
        let compartment = Compartment::try_build(items);

        assert!(compartment.is_ok());
        let compartment = compartment.unwrap();

        assert_eq!(compartment.size(), 0);
    }

    #[test]
    fn build_compartment_from_letters_no_duplicates() {
        let items = "aSfiIOJFdhspoK";
        let compartment = Compartment::try_build(items);

        assert!(compartment.is_ok());
        let compartment = compartment.unwrap();

        assert_eq!(compartment.size(), items.len());

        for item in items.chars() {
            assert_eq!(
                compartment.get_number_of_items_with_id(item),
                1,
                "the number of items for '{}' is wrong",
                item
            );
        }
    }

    #[test]
    fn build_compartment_from_letters_with_duplicates() {
        let items = "TiiaTAiATaTAiT";
        let compartment = Compartment::try_build(items);

        assert!(compartment.is_ok());
        let compartment = compartment.unwrap();

        assert_eq!(compartment.size(), items.len());

        assert_eq!(
            compartment.get_number_of_items_with_id('a'),
            2,
            "the number of items for 'a' is wrong"
        );
        assert_eq!(
            compartment.get_number_of_items_with_id('A'),
            3,
            "the number of items for 'A' is wrong"
        );
        assert_eq!(
            compartment.get_number_of_items_with_id('i'),
            4,
            "the number of items for 'i' is wrong"
        );
        assert_eq!(
            compartment.get_number_of_items_with_id('T'),
            5,
            "the number of items for 'T' is wrong"
        );
    }

    #[test]
    fn build_compartment_from_digits() {
        let items = "213547685192";
        let compartment = Compartment::try_build(items);

        assert_eq!(
            compartment,
            Err(CompartmentError {
                position: 0,
                cause: String::from(
                    "Invalid character, character must be ascii alphanumeric (a-z, A-Z), but was 2"
                )
            })
        )
    }

    #[test]
    fn build_compartment_from_special_signs() {
        let items = "!@#$%^&*()_[]";
        let compartment = Compartment::try_build(items);

        assert_eq!(
            compartment,
            Err(CompartmentError {
                position: 0,
                cause: String::from(
                    "Invalid character, character must be ascii alphanumeric (a-z, A-Z), but was !"
                )
            })
        )
    }

    #[test]
    fn build_compartment_from_special_letters() {
        let items = "abcèÏüñìäÜÏ";
        let compartment = Compartment::try_build(items);

        assert_eq!(
            compartment,
            Err(CompartmentError {
                position: 3,
                cause: String::from(
                    "Invalid character, character must be ascii alphanumeric (a-z, A-Z), but was è"
                )
            })
        )
    }
}
