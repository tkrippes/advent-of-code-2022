#[derive(Debug)]
pub struct Elves {
    elves: Vec<Elf>,
}

impl Elves {
    pub fn new() -> Self {
        Elves { elves: Vec::new() }
    }

    pub fn push(&mut self, elf: Elf) {
        self.elves.push(elf);
    }

    pub fn has_last_elf_calories(&mut self) -> bool {
        match self.elves.last() {
            Some(elf) => elf.has_calories(),
            None => false,
        }
    }

    pub fn add_calories_to_last_elf(&mut self, calories: u32) {
        if let Some(elf) = self.elves.last_mut() {
            elf.add_calories(calories);
        }
    }

    pub fn get_max_calories_of_one_elf(&self) -> u32 {
        self.elves
            .iter()
            .map(|elf| elf.get_calories())
            .max()
            .unwrap_or_default()
    }

    pub fn get_max_calories_of_three_elves(&self) -> u32 {
        let mut calories_list: Vec<u32> = self.elves.iter().map(|elf| elf.get_calories()).collect();

        calories_list.sort();
        calories_list.reverse();

        calories_list.iter().take(3).sum()
    }
}

impl PartialEq for Elves {
    fn eq(&self, other: &Self) -> bool {
        self.elves == other.elves
    }
}

#[derive(Debug)]
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
