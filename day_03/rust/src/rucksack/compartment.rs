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
            "compartment error at position '{}', {}",
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
    pub fn try_build(ids: &str) -> Result<Self, CompartmentError> {
        let mut items = Vec::new();

        for (position, id) in ids.chars().enumerate() {
            match Item::try_build(id) {
                Ok(item) => items.push(item),
                Err(err) => return Err(CompartmentError::build(position + 1, err)),
            }
        }

        Ok(Compartment { items })
    }

    pub fn get_first_common_item(&self, other: &Compartment) -> Option<&Item> {
        self.items.iter().find(|item| other.contains(item.get_id()))
    }

    fn contains(&self, item_id: char) -> bool {
        self.items.iter().any(|item| item.get_id() == item_id)
    }

    pub fn get_items(&self) -> Vec<Item> {
        self.items.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_compartment_from_nothing() {
        let ids = "";
        let compartment = Compartment::try_build(ids);

        let expected_compartment = Compartment { items: vec![] };

        assert_eq!(compartment, Ok(expected_compartment));
    }

    #[test]
    fn build_compartment_from_letters_no_duplicates() {
        let ids = "aSfiIOJFdhspoK";
        let compartment = Compartment::try_build(ids);

        let expected_compartment = Compartment {
            items: ids.chars().map(|id| Item::try_build(id).unwrap()).collect(),
        };

        assert_eq!(compartment, Ok(expected_compartment));
    }

    #[test]
    fn build_compartment_from_letters_with_duplicates() {
        let ids = "TiiaTAiATaTAiT";
        let compartment = Compartment::try_build(ids);

        let expected_compartment = Compartment {
            items: ids.chars().map(|id| Item::try_build(id).unwrap()).collect(),
        };

        assert_eq!(compartment, Ok(expected_compartment));
    }

    #[test]
    fn build_compartment_from_digits() {
        let ids = "213547685192";
        let compartment = Compartment::try_build(ids);

        assert_eq!(
            compartment,
            Err(CompartmentError {
                position: 1,
                cause: String::from(
                    "invalid character error, should be ascii alphanumeric (a-z, A-Z), but was '2'"
                )
            })
        )
    }

    #[test]
    fn build_compartment_from_special_signs() {
        let ids = "!@#$%^&*()_[]";
        let compartment = Compartment::try_build(ids);

        assert_eq!(
            compartment,
            Err(CompartmentError {
                position: 1,
                cause: String::from(
                    "invalid character error, should be ascii alphanumeric (a-z, A-Z), but was '!'"
                )
            })
        )
    }

    #[test]
    fn build_compartment_from_special_letters() {
        let ids = "èÏüñìäÜÏ";
        let compartment = Compartment::try_build(ids);

        assert_eq!(
            compartment,
            Err(CompartmentError {
                position: 1,
                cause: String::from(
                    "invalid character error, should be ascii alphanumeric (a-z, A-Z), but was 'è'"
                )
            })
        )
    }

    #[test]
    fn build_compartment_from_invalid_character_not_in_first_position() {
        let ids = "aDnvDS9DkjFn";
        let compartment = Compartment::try_build(ids);

        assert_eq!(
            compartment,
            Err(CompartmentError {
                position: 7,
                cause: String::from(
                    "invalid character error, should be ascii alphanumeric (a-z, A-Z), but was '9'"
                )
            })
        )
    }

    #[test]
    fn test_no_common_items() {
        let ids_1 = "abcd";
        let ids_2 = "efgh";
        let compartment_1 = Compartment::try_build(ids_1).unwrap();
        let compartment_2 = Compartment::try_build(ids_2).unwrap();

        assert_eq!(compartment_1.get_first_common_item(&compartment_2), None);
    }

    #[test]
    fn test_common_item_only_in_one_compartment() {
        let ids_1 = "aabb";
        let ids_2 = "ccdd";
        let compartment_1 = Compartment::try_build(ids_1).unwrap();
        let compartment_2 = Compartment::try_build(ids_2).unwrap();

        assert_eq!(compartment_1.get_first_common_item(&compartment_2), None);
    }

    #[test]
    fn test_one_common_item() {
        let ids_1 = "abcd";
        let ids_2 = "efgd";
        let compartment_1 = Compartment::try_build(ids_1).unwrap();
        let compartment_2 = Compartment::try_build(ids_2).unwrap();

        let first_common_item = compartment_1.get_first_common_item(&compartment_2).unwrap();
        assert_eq!(first_common_item.get_id(), 'd');
    }

    #[test]
    fn test_two_common_items() {
        let ids_1 = "abcd";
        let ids_2 = "ecgb";
        let compartment_1 = Compartment::try_build(ids_1).unwrap();
        let compartment_2 = Compartment::try_build(ids_2).unwrap();

        let first_common_item = compartment_1.get_first_common_item(&compartment_2).unwrap();
        assert_eq!(first_common_item.get_id(), 'b');
    }

    #[test]
    fn test_multiple_common_items() {
        let ids_1 = "abcd";
        let ids_2 = "edba";
        let compartment_1 = Compartment::try_build(ids_1).unwrap();
        let compartment_2 = Compartment::try_build(ids_2).unwrap();

        let first_common_item = compartment_1.get_first_common_item(&compartment_2).unwrap();
        assert_eq!(first_common_item.get_id(), 'a');
    }
}
