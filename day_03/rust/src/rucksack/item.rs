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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
