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
    fn build_compartment_from_letters() {
        let items = "asfiIOJFdfhspioK";
        let compartment = Compartment::build(items);

        assert_eq!(compartment.size(), items.len());
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
