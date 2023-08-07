pub mod item;

use item::Item;

#[derive(Debug, PartialEq)]
pub struct Compartment {
    items: Vec<Item>,
}

impl Compartment {
    pub fn build(items: &str) -> Self {
        let items: Vec<Item> = items.chars().filter_map(Item::try_build).collect();

        Compartment { items }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn contains(&self, item_id: char) -> bool {
        self.items.iter().any(|item| item.get_id() == item_id)
    }

    fn get_number_of_items(&self, item_id: char) -> usize {
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
        let compartment = Compartment::build(items);

        assert_eq!(compartment.size(), 0);
    }

    #[test]
    fn build_compartment_from_letters_no_duplicates() {
        let items = "aSfiIOJFdhspoK";
        let compartment = Compartment::build(items);

        assert_eq!(compartment.size(), items.len());

        for item in items.chars() {
            assert_eq!(
                compartment.get_number_of_items(item),
                1,
                "the number of items for '{}' is wrong",
                item
            );
        }
    }

    #[test]
    fn build_compartment_from_letters_with_duplicates() {
        let items = "TiiaTAiATaTAiT";
        let compartment = Compartment::build(items);

        assert_eq!(compartment.size(), items.len());

        assert_eq!(
            compartment.get_number_of_items('a'),
            2,
            "the number of items for 'a' is wrong"
        );
        assert_eq!(
            compartment.get_number_of_items('A'),
            3,
            "the number of items for 'A' is wrong"
        );
        assert_eq!(
            compartment.get_number_of_items('i'),
            4,
            "the number of items for 'i' is wrong"
        );
        assert_eq!(
            compartment.get_number_of_items('T'),
            5,
            "the number of items for 'T' is wrong"
        );
    }

    #[test]
    fn build_compartment_from_digits() {
        let items = "213547685192";
        let compartment = Compartment::build(items);

        assert_eq!(compartment.size(), 0)
    }

    #[test]
    fn build_compartment_from_special_signs() {
        let items = "!@#$%^&*()_[]";
        let compartment = Compartment::build(items);

        assert_eq!(compartment.size(), 0);
    }

    #[test]
    fn build_compartment_from_special_letters() {
        let items = "èÏüñìäÜÏ";
        let compartment = Compartment::build(items);

        assert_eq!(compartment.size(), 0);
    }
}
