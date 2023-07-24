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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_elf_no_calories() {
        let elf = Elf::new();
        assert_eq!(elf.get_calories(), 0);
    }

    #[test]
    fn test_empty_elf_has_no_calories() {
        let elf = Elf::new();
        assert!(!elf.has_calories());
    }

    #[test]
    fn test_elf_one_calories() {
        let mut elf = Elf::new();
        elf.add_calories(100);
        assert_eq!(elf.get_calories(), 100);
    }

    #[test]
    fn test_elf_one_calories_has_calories() {
        let mut elf = Elf::new();
        elf.add_calories(200);
        assert!(elf.has_calories());
    }

    #[test]
    fn test_elf_multiple_calories() {
        let mut elf = Elf::new();
        elf.add_calories(100);
        elf.add_calories(200);
        elf.add_calories(300);
        assert_eq!(elf.get_calories(), 600);
    }

    #[test]
    fn test_elf_multiple_calories_has_calories() {
        let mut elf = Elf::new();
        elf.add_calories(100);
        elf.add_calories(200);
        elf.add_calories(300);
        assert!(elf.has_calories());
    }
}
