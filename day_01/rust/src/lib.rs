use std::{cell::RefCell, fs::File, io::BufRead, io::BufReader, path::Path};
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug)]
struct Elf {
    calories: RefCell<Vec<i32>>,
}

impl Elf {
    fn get_total_calories(&self) -> i32 {
        self.calories.borrow().iter().sum()
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}

pub fn get_calories_from_elf_with_most(input_file_name: &Path) -> Option<i32> {
    setup_logging(Level::WARN);

    let elves = parse_file(input_file_name)?;

    get_max_calories(elves)
}

fn setup_logging(level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn parse_file(input_file_name: &Path) -> Option<Vec<Elf>> {
    let file = match File::open(input_file_name) {
        Ok(file) => file,
        Err(err) => {
            error!("could not open file, {}", err);
            return None;
        }
    };

    parse_elves_from_file(file)
}

fn parse_elves_from_file(input_file: File) -> Option<Vec<Elf>> {
    let file_reader = BufReader::new(input_file);
    let mut elves = Vec::new();

    for (index, line) in file_reader.lines().enumerate() {
        if index == 0 {
            elves.push(Elf {
                calories: RefCell::new(Vec::new()),
            });
        }

        let number = match line {
            Ok(line) => {
                if line.is_empty() {
                    if !elves.last().unwrap().calories.borrow().is_empty() {
                        elves.push(Elf {
                            calories: RefCell::new(Vec::new()),
                        });
                    }
                    continue;
                }

                match line.parse::<i32>() {
                    Ok(number) => number,
                    Err(err) => {
                        error!(
                            "could not parse elves from file, error in lines {}, {}",
                            index + 1,
                            err
                        );
                        return None;
                    }
                }
            }
            Err(err) => {
                error!(
                    "could not parse elves from file, error in line {}, {}",
                    index + 1,
                    err
                );
                return None;
            }
        };

        let mut last_elf_calories = elves.last().unwrap().calories.borrow_mut();
        last_elf_calories.push(number);
    }

    Some(elves)
}

fn get_max_calories(elves: Vec<Elf>) -> Option<i32> {
    elves.iter().map(|elf| elf.get_total_calories()).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input_file_name = Path::new("../input/test_input.txt");

        let elves = parse_file(test_input_file_name);

        let expected_elves = vec![
            Elf {
                calories: RefCell::new(vec![1000, 2000, 3000]),
            },
            Elf {
                calories: RefCell::new(vec![4000]),
            },
            Elf {
                calories: RefCell::new(vec![5000, 6000]),
            },
            Elf {
                calories: RefCell::new(vec![7000, 8000, 9000]),
            },
            Elf {
                calories: RefCell::new(vec![10000]),
            },
        ];

        assert_eq!(elves, Some(expected_elves));
    }

    #[test]
    fn test_input_result() {
        let test_input_file_name = Path::new("../input/test_input.txt");
        assert_eq!(
            get_calories_from_elf_with_most(test_input_file_name),
            Some(24000)
        );
    }
}
