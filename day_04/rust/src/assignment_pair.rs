use super::assignment::Assignment;

#[derive(Debug, PartialEq)]
pub struct AssignmentPair {
    first_assignment: Assignment,
    second_assignment: Assignment,
}

impl AssignmentPair {
    pub fn build(first_assignment: (u32, u32), second_assignment: (u32, u32)) -> Self {
        AssignmentPair {
            first_assignment: Assignment::build(first_assignment),
            second_assignment: Assignment::build(second_assignment),
        }
    }

    pub fn is_one_assignment_fully_contained_in_the_other_assignment(&self) -> bool {
        self.first_assignment
            .is_fully_contained_in(&self.second_assignment)
            || self
                .second_assignment
                .is_fully_contained_in(&self.first_assignment)
    }

    pub fn do_assignments_overlap(&self) -> bool {
        self.first_assignment
            .is_partially_contained_in(&self.second_assignment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_fully_contained_first_section() {
        let assignment_pair = AssignmentPair::build((2, 3), (1, 4));
        assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_fully_contained_second_section() {
        let assignment_pair = AssignmentPair::build((1, 4), (2, 3));
        assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_fully_contained_unit_first_section() {
        let assignment_pair = AssignmentPair::build((2, 2), (1, 4));
        assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_fully_contained_unit_second_section() {
        let assignment_pair = AssignmentPair::build((1, 4), (2, 2));
        assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_fully_contained_same_start_first_section() {
        let assignment_pair = AssignmentPair::build((1, 3), (1, 4));
        assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_fully_contained_same_start_second_section() {
        let assignment_pair = AssignmentPair::build((1, 4), (1, 3));
        assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_fully_contained_same_end_first_section() {
        let assignment_pair = AssignmentPair::build((2, 4), (1, 4));
        assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_fully_contained_same_end_second_section() {
        let assignment_pair = AssignmentPair::build((1, 4), (2, 4));
        assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_fully_contained_same_start_and_end() {
        let assignment_pair = AssignmentPair::build((1, 4), (1, 4));
        assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_not_fully_contained_unit_first_section() {
        let assignment_pair = AssignmentPair::build((0, 0), (1, 4));
        assert!(!assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_not_fully_contained_unit_second_section() {
        let assignment_pair = AssignmentPair::build((1, 4), (0, 0));
        assert!(!assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_not_fully_contained() {
        let assignment_pair = AssignmentPair::build((0, 3), (1, 4));
        assert!(!assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
    }

    #[test]
    fn is_partially_contained_full_first_section() {
        let assignment_pair = AssignmentPair::build((2, 3), (1, 4));
        assert!(assignment_pair.do_assignments_overlap());
    }

    #[test]
    fn is_partially_contained_full_second_section() {
        let assignment_pair = AssignmentPair::build((1, 4), (2, 3));
        assert!(assignment_pair.do_assignments_overlap());
    }

    #[test]
    fn is_partially_contained_unit_first_section() {
        let assignment_pair = AssignmentPair::build((2, 2), (1, 4));
        assert!(assignment_pair.do_assignments_overlap());
    }

    #[test]
    fn is_partially_contained_unit_second_section() {
        let assignment_pair = AssignmentPair::build((1, 4), (2, 2));
        assert!(assignment_pair.do_assignments_overlap());
    }

    #[test]
    fn is_partially_contained_full_same_start() {
        let assignment_pair = AssignmentPair::build((1, 3), (1, 4));
        assert!(assignment_pair.do_assignments_overlap());
    }

    #[test]
    fn is_partially_contained_full_same_end() {
        let assignment_pair = AssignmentPair::build((1, 4), (2, 4));
        assert!(assignment_pair.do_assignments_overlap());
    }

    #[test]
    fn is_partially_contained_full_same_start_same_end() {
        let assignment_pair = AssignmentPair::build((1, 4), (1, 4));
        assert!(assignment_pair.do_assignments_overlap());
    }

    #[test]
    fn is_partially_contained_not_full() {
        let assignment_pair = AssignmentPair::build((0, 3), (1, 4));
        assert!(assignment_pair.do_assignments_overlap());
    }

    #[test]
    fn is_not_partially_contained_completely() {
        let assignment_pair = AssignmentPair::build((0, 1), (2, 5));
        assert!(!assignment_pair.do_assignments_overlap());
    }

    #[test]
    fn is_not_partially_contained() {
        let assignment_pair = AssignmentPair::build((0, 2), (3, 5));
        assert!(!assignment_pair.do_assignments_overlap());
    }
}
