#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Section {
    id: u32,
}

impl Section {
    pub fn build(id: u32) -> Self {
        Section { id }
    }
}
