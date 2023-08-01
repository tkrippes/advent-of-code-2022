use super::item::Item;

#[derive(Debug, PartialEq)]
pub struct Compartment {
    items: Vec<Item>,
}

impl Compartment {
    fn try_build(items: &str) -> Option<Self> {
        let items: Vec<Option<Item>> = items.chars().map(Item::try_build).collect();

        if items.iter().any(|item| item.is_none()) {
            None
        } else {
            Some(Compartment {
                items: items.into_iter().map(|item| item.unwrap()).collect(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_compartment_from_letters() {
        let items = "asfiIOJFdfhspioK";
        let compartment = Compartment::try_build(items);

        assert!(compartment.is_some());
        assert_eq!(compartment.unwrap().items.len(), items.len());
    }

    #[test]
    fn build_compartment_with_digits() {
        let items = "asf1IOJFdfhspi8K";
        let compartment = Compartment::try_build(items);

        assert!(compartment.is_none());
    }

    #[test]
    fn build_compartment_with_special_signs() {
        let items = "@sf!IOJF[fhspi_K";
        let compartment = Compartment::try_build(items);

        assert!(compartment.is_none());
    }

    #[test]
    fn build_compartment_with_special_letters() {
        let items = "asfèÏOJFdfhsüiñK";
        let compartment = Compartment::try_build(items);

        assert!(compartment.is_none());
    }
}
