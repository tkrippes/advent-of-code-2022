use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterError {
    invalid_character: char,
}

impl fmt::Display for InvalidCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid character error, should be ascii alphanumeric (a-z, A-Z), but was '{}'",
            self.invalid_character
        )
    }
}

impl error::Error for InvalidCharacterError {}

impl InvalidCharacterError {
    fn build(invalid_character: char) -> Self {
        InvalidCharacterError { invalid_character }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    id: char,
}

impl Item {
    pub fn try_build(id: char) -> Result<Self, InvalidCharacterError> {
        if id.is_ascii_alphabetic() {
            Ok(Item { id })
        } else {
            Err(InvalidCharacterError::build(id))
        }
    }

    pub fn get_id(&self) -> char {
        self.id
    }

    pub fn get_priority(&self) -> u32 {
        if self.id.is_lowercase() {
            self.id as u32 - 'a' as u32 + 1
        } else {
            self.id as u32 - 'A' as u32 + 27
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_item_from_space() {
        assert_eq!(
            Item::try_build(' '),
            Err(InvalidCharacterError {
                invalid_character: ' '
            })
        );
    }

    #[test]
    fn build_item_from_lowercase_letters() {
        for lowercase_letter in 'a'..='z' {
            assert_eq!(
                Item::try_build(lowercase_letter),
                Ok(Item {
                    id: lowercase_letter
                })
            );
        }
    }

    #[test]
    fn build_item_from_uppercase_letters() {
        for uppercase_letter in 'A'..='Z' {
            assert_eq!(
                Item::try_build(uppercase_letter),
                Ok(Item {
                    id: uppercase_letter
                })
            );
        }
    }

    #[test]
    fn build_item_from_digit() {
        for digit in '0'..='9' {
            assert_eq!(
                Item::try_build(digit),
                Err(InvalidCharacterError {
                    invalid_character: digit,
                })
            );
        }
    }

    #[test]
    fn build_item_from_special_signs() {
        for special_sign in " !@#$%^&*()_-+={}[]".chars() {
            assert_eq!(
                Item::try_build(special_sign),
                Err(InvalidCharacterError {
                    invalid_character: special_sign,
                })
            )
        }
    }

    #[test]
    fn build_item_from_special_letters() {
        for special_letter in "èñàùéáúäöüßç".chars() {
            assert_eq!(
                Item::try_build(special_letter),
                Err(InvalidCharacterError {
                    invalid_character: special_letter,
                })
            )
        }
    }

    #[test]
    fn test_lowercase_priorities() {
        let mut expected_priority = 1;

        for character in 'a'..='z' {
            assert_eq!(
                Item::try_build(character).unwrap().get_priority(),
                expected_priority,
                "the priority of '{}' is wrong",
                character
            );
            expected_priority += 1;
        }
    }

    #[test]
    fn test_uppercase_priorities() {
        let mut expected_priority = 27;

        for character in 'A'..='Z' {
            assert_eq!(
                Item::try_build(character).unwrap().get_priority(),
                expected_priority,
                "the priority of '{}' is wrong",
                character
            );
            expected_priority += 1;
        }
    }
}
