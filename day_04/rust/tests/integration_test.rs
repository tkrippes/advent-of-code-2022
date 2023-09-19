use advent_of_code_2022_day_03::{
    get_number_of_fully_contained_assignments, get_number_of_overlapping_assignments,
};

#[test]
fn test_valid_file_fully_contained() {
    let file_name = "../input/test_input.txt";
    assert_eq!(get_number_of_fully_contained_assignments(file_name), 2);
}

#[test]
fn test_missing_file_fully_contained() {
    let file_name = "../input/missing_test_input.txt";
    assert_eq!(get_number_of_fully_contained_assignments(file_name), 0);
}

#[test]
fn test_invalid_file_fully_contained() {
    let file_name = "../input/invalid_test_input.txt";
    assert_eq!(get_number_of_fully_contained_assignments(file_name), 0);
}

#[test]
fn test_valid_file_overlapping() {
    let file_name = "../input/test_input.txt";
    assert_eq!(get_number_of_overlapping_assignments(file_name), 4);
}

#[test]
fn test_missing_file_overlapping() {
    let file_name = "../input/missing_test_input.txt";
    assert_eq!(get_number_of_overlapping_assignments(file_name), 0);
}

#[test]
fn test_invalid_file_overlapping() {
    let file_name = "../input/invalid_test_input.txt";
    assert_eq!(get_number_of_overlapping_assignments(file_name), 0);
}
