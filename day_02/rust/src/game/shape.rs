#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn get_points(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_points() {
        let rock = Shape::Rock;
        assert_eq!(rock.get_points(), 1);
    }

    #[test]
    fn test_paper_points() {
        let paper = Shape::Paper;
        assert_eq!(paper.get_points(), 2);
    }

    #[test]
    fn test_scissors_points() {
        let scissors = Shape::Scissors;
        assert_eq!(scissors.get_points(), 3);
    }
}
