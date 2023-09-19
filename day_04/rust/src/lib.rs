mod assignment;
mod assignment_pair;
mod parser;

use parser::Parser;

pub fn get_number_of_fully_contained_assignments(file_name: &str, parsing_regex: &str) -> usize {
    let parser = Parser::build(file_name, parsing_regex);

    match parser.try_get_assignment_pairs() {
        Ok(assignment_pairs) => assignment_pairs
            .iter()
            .map(|assignment_pair| {
                assignment_pair.is_one_assignment_fully_contained_in_the_other_assignment()
            })
            .filter(|is_fully_contained| *is_fully_contained)
            .count(),
        Err(err) => {
            println!(
                "Error while getting number of fully contained assignments, {}",
                err
            );
            0
        }
    }
}

pub fn get_number_of_overlapping_assignments(file_name: &str, parsing_regex: &str) -> usize {
    let parser = Parser::build(file_name, parsing_regex);

    match parser.try_get_assignment_pairs() {
        Ok(assignment_pairs) => assignment_pairs
            .iter()
            .map(|assignment_pair| assignment_pair.do_assignments_overlap())
            .filter(|do_overlap| *do_overlap)
            .count(),
        Err(err) => {
            println!(
                "Error while getting number of overlapping assignments, {}",
                err
            );
            0
        }
    }
}
