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
        let ids = "";
        let rucksack = Rucksack::try_build(ids);

        let expected_rucksack = Rucksack {
            first_compartment: Compartment::try_build("").unwrap(),
            second_compartment: Compartment::try_build("").unwrap(),
        };

        assert_eq!(rucksack, Ok(expected_rucksack));
    }

    #[test]
    fn build_rucksack_with_one_item() {
        let ids = "c";
        let rucksack = Rucksack::try_build(ids);

        let expected_rucksack = Rucksack {
            first_compartment: Compartment::try_build("c").unwrap(),
            second_compartment: Compartment::try_build("").unwrap(),
        };

        assert_eq!(rucksack, Ok(expected_rucksack));
    }

    #[test]
    fn build_rucksack_with_two_items() {
        let ids = "ca";
        let rucksack = Rucksack::try_build(ids);

        let expected_rucksack = Rucksack {
            first_compartment: Compartment::try_build("c").unwrap(),
            second_compartment: Compartment::try_build("a").unwrap(),
        };

        assert_eq!(rucksack, Ok(expected_rucksack));
    }

    #[test]
    fn build_rucksack_with_odd_number_of_items() {
        let ids = "adflvmd";
        let rucksack = Rucksack::try_build(ids);

        let expected_rucksack = Rucksack {
            first_compartment: Compartment::try_build("adfl").unwrap(),
            second_compartment: Compartment::try_build("vmd").unwrap(),
        };

        assert_eq!(rucksack, Ok(expected_rucksack));
    }

    #[test]
    fn build_rucksack_with_even_number_of_items() {
        let ids = "aidjrmcnah";
        let rucksack = Rucksack::try_build(ids);

        let expected_rucksack = Rucksack {
            first_compartment: Compartment::try_build("aidjr").unwrap(),
            second_compartment: Compartment::try_build("mcnah").unwrap(),
        };

        assert_eq!(rucksack, Ok(expected_rucksack));
    }
}
