use super::result::Result;
use super::shape::Shape;
use super::Game;

#[derive(Debug, PartialEq)]
pub struct GameVariant1 {
    opponent_shape: Shape,
    own_shape: Shape,
}

impl Game for GameVariant1 {
    fn get_points(&self) -> u32 {
        self.get_shape_points() + self.get_game_result_points()
    }
}

impl GameVariant1 {
    pub fn build(opponent_shape: Shape, own_shape: Shape) -> Self {
        GameVariant1 {
            opponent_shape,
            own_shape,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_rock_points() {
        let game = GameVariant1 {
            opponent_shape: Shape::Rock,
            own_shape: Shape::Rock,
        };
        assert_eq!(game.get_game_result_points(), 3);
        assert_eq!(game.get_points(), 4);
    }

    #[test]
    fn test_rock_paper_points() {
        let game = GameVariant1 {
            opponent_shape: Shape::Rock,
            own_shape: Shape::Paper,
        };
        assert_eq!(game.get_game_result_points(), 6);
        assert_eq!(game.get_points(), 8);
    }

    #[test]
    fn test_rock_scissors_points() {
        let game = GameVariant1 {
            opponent_shape: Shape::Rock,
            own_shape: Shape::Scissors,
        };
        assert_eq!(game.get_game_result_points(), 0);
        assert_eq!(game.get_points(), 3);
    }

    #[test]
    fn test_paper_rock_points() {
        let game = GameVariant1 {
            opponent_shape: Shape::Paper,
            own_shape: Shape::Rock,
        };
        assert_eq!(game.get_game_result_points(), 0);
        assert_eq!(game.get_points(), 1);
    }

    #[test]
    fn test_paper_paper_points() {
        let game = GameVariant1 {
            opponent_shape: Shape::Paper,
            own_shape: Shape::Paper,
        };
        assert_eq!(game.get_game_result_points(), 3);
        assert_eq!(game.get_points(), 5);
    }

    #[test]
    fn test_paper_scissors_points() {
        let game = GameVariant1 {
            opponent_shape: Shape::Paper,
            own_shape: Shape::Scissors,
        };
        assert_eq!(game.get_game_result_points(), 6);
        assert_eq!(game.get_points(), 9);
    }

    #[test]
    fn test_scissors_rock_points() {
        let game = GameVariant1 {
            opponent_shape: Shape::Scissors,
            own_shape: Shape::Rock,
        };
        assert_eq!(game.get_game_result_points(), 6);
        assert_eq!(game.get_points(), 7);
    }

    #[test]
    fn test_scissors_paper_points() {
        let game = GameVariant1 {
            opponent_shape: Shape::Scissors,
            own_shape: Shape::Paper,
        };
        assert_eq!(game.get_game_result_points(), 0);
        assert_eq!(game.get_points(), 2);
    }

    #[test]
    fn test_scissors_scissors_points() {
        let game = GameVariant1 {
            opponent_shape: Shape::Scissors,
            own_shape: Shape::Scissors,
        };
        assert_eq!(game.get_game_result_points(), 3);
        assert_eq!(game.get_points(), 6);
    }
}
