#[derive(Debug, PartialEq)]
pub struct Item {
    id: char,
}

impl Item {
    pub fn try_build(id: char) -> Option<Self> {
        if id.is_ascii_alphabetic() {
            Some(Item { id })
        } else {
            None
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
        assert_eq!(Item::try_build(' '), None);
    }

    #[test]
    fn build_item_from_lowercase_letters() {
        for lowercase_letter in 'a'..='z' {
            assert_eq!(
                Item::try_build(lowercase_letter),
                Some(Item {
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
                Some(Item {
                    id: uppercase_letter
                })
            );
        }
    }

    #[test]
    fn build_item_from_digit() {
        for digit in '0'..='9' {
            assert_eq!(Item::try_build(digit), None);
        }
    }

    #[test]
    fn build_item_from_special_signs() {
        assert_eq!(Item::try_build(' '), None);
        assert_eq!(Item::try_build('!'), None);
        assert_eq!(Item::try_build('@'), None);
        assert_eq!(Item::try_build('#'), None);
        assert_eq!(Item::try_build('$'), None);
        assert_eq!(Item::try_build('%'), None);
        assert_eq!(Item::try_build('^'), None);
        assert_eq!(Item::try_build('&'), None);
        assert_eq!(Item::try_build('*'), None);
        assert_eq!(Item::try_build('('), None);
        assert_eq!(Item::try_build(')'), None);
        assert_eq!(Item::try_build('_'), None);
        assert_eq!(Item::try_build('-'), None);
        assert_eq!(Item::try_build('+'), None);
        assert_eq!(Item::try_build('='), None);
        assert_eq!(Item::try_build('{'), None);
        assert_eq!(Item::try_build('}'), None);
        assert_eq!(Item::try_build('['), None);
        assert_eq!(Item::try_build(']'), None);
    }

    #[test]
    fn build_item_from_special_letters() {
        assert_eq!(Item::try_build('è'), None);
        assert_eq!(Item::try_build('ñ'), None);
        assert_eq!(Item::try_build('à'), None);
        assert_eq!(Item::try_build('ù'), None);
        assert_eq!(Item::try_build('é'), None);
        assert_eq!(Item::try_build('á'), None);
        assert_eq!(Item::try_build('ú'), None);
        assert_eq!(Item::try_build('ä'), None);
        assert_eq!(Item::try_build('ö'), None);
        assert_eq!(Item::try_build('ü'), None);
        assert_eq!(Item::try_build('ß'), None);
        assert_eq!(Item::try_build('ç'), None);
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
