mod section;

use section::Section;

#[derive(Debug, PartialEq)]
pub struct Assignment {
    start_section: Section,
    end_section: Section,
}

impl Assignment {
    pub fn build(sections: (u32, u32)) -> Self {
        Assignment {
            start_section: Section::build(sections.0),
            end_section: Section::build(sections.1),
        }
    }

    pub fn is_fully_contained_in(&self, other: &Assignment) -> bool {
        self.start_section >= other.start_section && self.end_section <= other.end_section
    }

    pub fn is_partially_contained_in(&self, other: &Assignment) -> bool {
        if self.start_section <= other.start_section {
            self.end_section >= other.start_section
        } else {
            self.start_section <= other.end_section
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_fully_contained() {
        let first_assignment = Assignment::build((2, 3));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_fully_contained_in(&second_assignment));
    }

    #[test]
    fn is_fully_contained_unit() {
        let first_assignment = Assignment::build((2, 2));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_fully_contained_in(&second_assignment));
    }

    #[test]
    fn is_fully_contained_same_start() {
        let first_assignment = Assignment::build((1, 3));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_fully_contained_in(&second_assignment));
    }

    #[test]
    fn is_fully_contained_same_end() {
        let first_assignment = Assignment::build((2, 4));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_fully_contained_in(&second_assignment));
    }

    #[test]
    fn is_fully_contained_same_start_and_end() {
        let first_assignment = Assignment::build((1, 4));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_fully_contained_in(&second_assignment));
    }

    #[test]
    fn is_not_fully_contained_unit_assignment() {
        let first_assignment = Assignment::build((0, 0));
        let second_assignment = Assignment::build((1, 4));
        assert!(!first_assignment.is_fully_contained_in(&second_assignment));
    }

    #[test]
    fn is_not_fully_contained_start() {
        let first_assignment = Assignment::build((0, 3));
        let second_assignment = Assignment::build((1, 4));
        assert!(!first_assignment.is_fully_contained_in(&second_assignment));
    }

    #[test]
    fn is_not_fully_contained_end() {
        let first_assignment = Assignment::build((2, 5));
        let second_assignment = Assignment::build((1, 4));
        assert!(!first_assignment.is_fully_contained_in(&second_assignment));
    }

    #[test]
    fn is_partially_contained_full() {
        let first_assignment = Assignment::build((2, 3));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_partially_contained_in(&second_assignment));
    }

    #[test]
    fn is_partially_contained_unit() {
        let first_assignment = Assignment::build((2, 2));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_partially_contained_in(&second_assignment));
    }

    #[test]
    fn is_partially_contained_full_same_start() {
        let first_assignment = Assignment::build((1, 3));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_partially_contained_in(&second_assignment));
    }

    #[test]
    fn is_partially_contained_full_same_end() {
        let first_assignment = Assignment::build((2, 4));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_partially_contained_in(&second_assignment));
    }

    #[test]
    fn is_partially_contained_full_same_start_same_end() {
        let first_assignment = Assignment::build((1, 4));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_partially_contained_in(&second_assignment));
    }

    #[test]
    fn is_partially_contained_start_not_contained() {
        let first_assignment = Assignment::build((0, 3));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_partially_contained_in(&second_assignment));
    }

    #[test]
    fn is_partially_contained_end_not_contained() {
        let first_assignment = Assignment::build((2, 5));
        let second_assignment = Assignment::build((1, 4));
        assert!(first_assignment.is_partially_contained_in(&second_assignment));
    }

    #[test]
    fn is_not_partially_contained_before() {
        let first_assignment = Assignment::build((0, 1));
        let second_assignment = Assignment::build((2, 5));
        assert!(!first_assignment.is_partially_contained_in(&second_assignment));
    }

    #[test]
    fn is_not_partially_contained_after() {
        let first_assignment = Assignment::build((6, 7));
        let second_assignment = Assignment::build((2, 5));
        assert!(!first_assignment.is_partially_contained_in(&second_assignment));
    }
}
