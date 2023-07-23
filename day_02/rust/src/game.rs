#[derive(Debug, PartialEq)]
pub struct Game {
    opponent_shape: Shape,
    own_shape: Shape,
}

impl Game {
    pub fn build(opponent_shape: Shape, own_shape: Shape) -> Self {
        Game {
            opponent_shape,
            own_shape,
        }
    }

    pub fn get_points(&self) -> u32 {
        self.get_shape_points() + self.get_game_result_points()
    }

    fn get_shape_points(&self) -> u32 {
        self.own_shape.get_points()
    }

    fn get_game_result_points(&self) -> u32 {
        self.get_game_result().get_points()
    }

    fn get_game_result(&self) -> Result {
        match (&self.opponent_shape, &self.own_shape) {
            (Shape::Rock, Shape::Rock) => Result::Tie,
            (Shape::Rock, Shape::Paper) => Result::Win,
            (Shape::Rock, Shape::Scissors) => Result::Loss,
            (Shape::Paper, Shape::Rock) => Result::Loss,
            (Shape::Paper, Shape::Paper) => Result::Tie,
            (Shape::Paper, Shape::Scissors) => Result::Win,
            (Shape::Scissors, Shape::Rock) => Result::Win,
            (Shape::Scissors, Shape::Paper) => Result::Loss,
            (Shape::Scissors, Shape::Scissors) => Result::Tie,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn get_points(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum Result {
    Loss,
    Tie,
    Win,
}

impl Result {
    fn get_points(&self) -> u32 {
        match self {
            Result::Loss => 0,
            Result::Tie => 3,
            Result::Win => 6,
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

    #[test]
    fn test_rock_rock_points() {
        let game = Game {
            opponent_shape: Shape::Rock,
            own_shape: Shape::Rock,
        };
        assert_eq!(game.get_game_result_points(), 3);
        assert_eq!(game.get_points(), 4);
    }

    #[test]
    fn test_rock_paper_points() {
        let game = Game {
            opponent_shape: Shape::Rock,
            own_shape: Shape::Paper,
        };
        assert_eq!(game.get_game_result_points(), 6);
        assert_eq!(game.get_points(), 8);
    }

    #[test]
    fn test_rock_scissors_points() {
        let game = Game {
            opponent_shape: Shape::Rock,
            own_shape: Shape::Scissors,
        };
        assert_eq!(game.get_game_result_points(), 0);
        assert_eq!(game.get_points(), 3);
    }

    #[test]
    fn test_paper_rock_points() {
        let game = Game {
            opponent_shape: Shape::Paper,
            own_shape: Shape::Rock,
        };
        assert_eq!(game.get_game_result_points(), 0);
        assert_eq!(game.get_points(), 1);
    }

    #[test]
    fn test_paper_paper_points() {
        let game = Game {
            opponent_shape: Shape::Paper,
            own_shape: Shape::Paper,
        };
        assert_eq!(game.get_game_result_points(), 3);
        assert_eq!(game.get_points(), 5);
    }

    #[test]
    fn test_paper_scissors_points() {
        let game = Game {
            opponent_shape: Shape::Paper,
            own_shape: Shape::Scissors,
        };
        assert_eq!(game.get_game_result_points(), 6);
        assert_eq!(game.get_points(), 9);
    }

    #[test]
    fn test_scissors_rock_points() {
        let game = Game {
            opponent_shape: Shape::Scissors,
            own_shape: Shape::Rock,
        };
        assert_eq!(game.get_game_result_points(), 6);
        assert_eq!(game.get_points(), 7);
    }

    #[test]
    fn test_scissors_paper_points() {
        let game = Game {
            opponent_shape: Shape::Scissors,
            own_shape: Shape::Paper,
        };
        assert_eq!(game.get_game_result_points(), 0);
        assert_eq!(game.get_points(), 2);
    }

    #[test]
    fn test_scissors_scissors_points() {
        let game = Game {
            opponent_shape: Shape::Scissors,
            own_shape: Shape::Scissors,
        };
        assert_eq!(game.get_game_result_points(), 3);
        assert_eq!(game.get_points(), 6);
    }
}
