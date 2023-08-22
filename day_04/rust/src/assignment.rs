use std::cmp::Ordering::{Equal, Greater, Less};

struct AssignmentPair {
    first_assignment: Assignment,
    second_assignment: Assignment,
}

impl AssignmentPair {
    fn build(
        first_assignment_start_section_id: u32,
        first_assignment_end_section_id: u32,
        second_assignment_start_section_id: u32,
        second_assignment_end_section_id: u32,
    ) -> Self {
        AssignmentPair {
            first_assignment: Assignment::build(
                first_assignment_start_section_id,
                first_assignment_end_section_id,
            ),
            second_assignment: Assignment::build(
                second_assignment_start_section_id,
                second_assignment_end_section_id,
            ),
        }
    }

    fn is_one_assignment_fully_contained_in_the_other_assignment(&self) -> bool {
        self.first_assignment
            .is_fully_contained_in(&self.second_assignment)
            || self
                .second_assignment
                .is_fully_contained_in(&self.first_assignment)
    }

    fn do_assignments_overlap(&self) -> bool {
        self.first_assignment
            .is_partially_contained_in(&self.second_assignment)
        // || self
        //     .second_assignment
        //     .is_partially_contained_in(&self.first_assignment)
    }
}

struct Assignment {
    start_section: Section,
    end_section: Section,
}

impl Assignment {
    fn build(start_section_id: u32, end_section_id: u32) -> Self {
        Assignment {
            start_section: Section::build(start_section_id),
            end_section: Section::build(end_section_id),
        }
    }

    fn is_fully_contained_in(&self, other: &Assignment) -> bool {
        self.start_section >= other.start_section && self.end_section <= other.end_section
    }

    fn is_partially_contained_in(&self, other: &Assignment) -> bool {
        if self.start_section <= other.start_section {
            self.end_section > other.start_section
        } else {
            self.start_section < other.end_section
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Section {
    id: u32,
}

impl Section {
    fn build(id: u32) -> Self {
        Section { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod assignment_pair {
        use super::*;

        #[test]
        fn is_fully_contained_first_section() {
            let assignment_pair = AssignmentPair::build(2, 3, 1, 4);
            assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_fully_contained_second_section() {
            let assignment_pair = AssignmentPair::build(1, 4, 2, 3);
            assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_fully_contained_unit_first_section() {
            let assignment_pair = AssignmentPair::build(2, 2, 1, 4);
            assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_fully_contained_unit_second_section() {
            let assignment_pair = AssignmentPair::build(1, 4, 2, 2);
            assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_fully_contained_same_start_first_section() {
            let assignment_pair = AssignmentPair::build(1, 3, 1, 4);
            assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_fully_contained_same_start_second_section() {
            let assignment_pair = AssignmentPair::build(1, 4, 1, 3);
            assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_fully_contained_same_end_first_section() {
            let assignment_pair = AssignmentPair::build(2, 4, 1, 4);
            assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_fully_contained_same_end_second_section() {
            let assignment_pair = AssignmentPair::build(1, 4, 2, 4);
            assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_fully_contained_same_start_and_end() {
            let assignment_pair = AssignmentPair::build(1, 4, 1, 4);
            assert!(assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_not_fully_contained_unit_first_section() {
            let assignment_pair = AssignmentPair::build(0, 0, 1, 4);
            assert!(!assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_not_fully_contained_unit_second_section() {
            let assignment_pair = AssignmentPair::build(1, 4, 0, 0);
            assert!(!assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_not_fully_contained() {
            let assignment_pair = AssignmentPair::build(0, 3, 1, 4);
            assert!(!assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment());
        }

        #[test]
        fn is_partially_contained_full_first_section() {
            let assignment_pair = AssignmentPair::build(2, 3, 1, 4);
            assert!(assignment_pair.do_assignments_overlap());
        }

        #[test]
        fn is_partially_contained_full_second_section() {
            let assignment_pair = AssignmentPair::build(1, 4, 2, 3);
            assert!(assignment_pair.do_assignments_overlap());
        }

        #[test]
        fn is_partially_contained_unit_first_section() {
            let assignment_pair = AssignmentPair::build(2, 2, 1, 4);
            assert!(assignment_pair.do_assignments_overlap());
        }

        #[test]
        fn is_partially_contained_unit_second_section() {
            let assignment_pair = AssignmentPair::build(1, 4, 2, 2);
            assert!(assignment_pair.do_assignments_overlap());
        }

        #[test]
        fn is_partially_contained_full_same_start() {
            let assignment_pair = AssignmentPair::build(1, 3, 1, 4);
            assert!(assignment_pair.do_assignments_overlap());
        }

        #[test]
        fn is_partially_contained_full_same_end() {
            let assignment_pair = AssignmentPair::build(1, 4, 2, 4);
            assert!(assignment_pair.do_assignments_overlap());
        }

        #[test]
        fn is_partially_contained_full_same_start_same_end() {
            let assignment_pair = AssignmentPair::build(1, 4, 1, 4);
            assert!(assignment_pair.do_assignments_overlap());
        }

        #[test]
        fn is_partially_contained_not_full() {
            let assignment_pair = AssignmentPair::build(0, 3, 1, 4);
            assert!(assignment_pair.do_assignments_overlap());
        }

        #[test]
        fn is_not_partially_contained_completely() {
            let assignment_pair = AssignmentPair::build(0, 1, 2, 5);
            assert!(!assignment_pair.do_assignments_overlap());
        }

        #[test]
        fn is_not_partially_contained() {
            let assignment_pair = AssignmentPair::build(0, 2, 2, 5);
            assert!(!assignment_pair.do_assignments_overlap());
        }
    }

    mod assignment {
        use super::*;
        #[test]
        fn is_fully_contained() {
            let first_assignment = Assignment::build(2, 3);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_fully_contained_in(&second_assignment));
        }

        #[test]
        fn is_fully_contained_unit() {
            let first_assignment = Assignment::build(2, 2);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_fully_contained_in(&second_assignment));
        }

        #[test]
        fn is_fully_contained_same_start() {
            let first_assignment = Assignment::build(1, 3);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_fully_contained_in(&second_assignment));
        }

        #[test]
        fn is_fully_contained_same_end() {
            let first_assignment = Assignment::build(2, 4);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_fully_contained_in(&second_assignment));
        }

        #[test]
        fn is_fully_contained_same_start_and_end() {
            let first_assignment = Assignment::build(1, 4);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_fully_contained_in(&second_assignment));
        }

        #[test]
        fn is_not_fully_contained_unit_assignment() {
            let first_assignment = Assignment::build(0, 0);
            let second_assignment = Assignment::build(1, 4);
            assert!(!first_assignment.is_fully_contained_in(&second_assignment));
        }

        #[test]
        fn is_not_fully_contained_start() {
            let first_assignment = Assignment::build(0, 3);
            let second_assignment = Assignment::build(1, 4);
            assert!(!first_assignment.is_fully_contained_in(&second_assignment));
        }

        #[test]
        fn is_not_fully_contained_end() {
            let first_assignment = Assignment::build(2, 5);
            let second_assignment = Assignment::build(1, 4);
            assert!(!first_assignment.is_fully_contained_in(&second_assignment));
        }

        #[test]
        fn is_partially_contained_full() {
            let first_assignment = Assignment::build(2, 3);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_partially_contained_unit() {
            let first_assignment = Assignment::build(2, 2);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_partially_contained_full_same_start() {
            let first_assignment = Assignment::build(1, 3);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_partially_contained_full_same_end() {
            let first_assignment = Assignment::build(2, 4);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_partially_contained_full_same_start_same_end() {
            let first_assignment = Assignment::build(1, 4);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_partially_contained_start_not_contained() {
            let first_assignment = Assignment::build(0, 3);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_partially_contained_end_not_contained() {
            let first_assignment = Assignment::build(2, 5);
            let second_assignment = Assignment::build(1, 4);
            assert!(first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_not_partially_contained_completely_before() {
            let first_assignment = Assignment::build(0, 1);
            let second_assignment = Assignment::build(2, 5);
            assert!(!first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_not_partially_contained_completely_after() {
            let first_assignment = Assignment::build(6, 7);
            let second_assignment = Assignment::build(2, 5);
            assert!(!first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_not_partially_contained_before() {
            let first_assignment = Assignment::build(0, 2);
            let second_assignment = Assignment::build(2, 5);
            assert!(!first_assignment.is_partially_contained_in(&second_assignment));
        }

        #[test]
        fn is_not_partially_contained_after() {
            let first_assignment = Assignment::build(5, 7);
            let second_assignment = Assignment::build(2, 5);
            assert!(!first_assignment.is_partially_contained_in(&second_assignment));
        }
    }
}
