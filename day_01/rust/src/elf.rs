#[derive(Debug, Clone)]
pub struct Elf {
    calories: u32,
}

impl Elf {
    pub fn new() -> Self {
        Elf { calories: 0 }
    }

    pub fn has_calories(&self) -> bool {
        self.calories != 0
    }

    pub fn add_calories(&mut self, calories: u32) {
        self.calories += calories
    }

    pub fn get_calories(&self) -> u32 {
        self.calories
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}
