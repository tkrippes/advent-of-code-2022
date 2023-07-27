pub mod result;
pub mod shape;

use result::Result;
use shape::Shape;

pub enum GameVariant {
    V1,
    V2,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Game {
    V1 {
        opponent_shape: Shape,
        own_shape: Shape,
    },
    V2 {
        opponent_shape: Shape,
        result: Result,
    },
}

impl Game {
    pub fn build_v1(opponent_shape: Shape, own_shape: Shape) -> Self {
        Game::V1 {
            opponent_shape,
            own_shape,
        }
    }

    pub fn build_v2(opponent_shape: Shape, result: Result) -> Self {
        Game::V2 {
            opponent_shape,
            result,
        }
    }

    pub fn get_points(&self) -> u32 {
        self.get_own_shape().get_points() + self.get_game_result().get_points()
    }

    fn get_own_shape(&self) -> &Shape {
        match self {
            Game::V1 {
                opponent_shape: _,
                own_shape,
            } => own_shape,
            Game::V2 {
                opponent_shape,
                result,
            } => match (opponent_shape, result) {
                (Shape::Rock, Result::Loss) => &Shape::Scissors,
                (Shape::Rock, Result::Tie) => &Shape::Rock,
                (Shape::Rock, Result::Win) => &Shape::Paper,
                (Shape::Paper, Result::Loss) => &Shape::Rock,
                (Shape::Paper, Result::Tie) => &Shape::Paper,
                (Shape::Paper, Result::Win) => &Shape::Scissors,
                (Shape::Scissors, Result::Loss) => &Shape::Paper,
                (Shape::Scissors, Result::Tie) => &Shape::Scissors,
                (Shape::Scissors, Result::Win) => &Shape::Rock,
            },
        }
    }

    fn get_game_result(&self) -> &Result {
        match self {
            Game::V1 {
                opponent_shape,
                own_shape,
            } => match (opponent_shape, own_shape) {
                (Shape::Rock, Shape::Rock) => &Result::Tie,
                (Shape::Rock, Shape::Paper) => &Result::Win,
                (Shape::Rock, Shape::Scissors) => &Result::Loss,
                (Shape::Paper, Shape::Rock) => &Result::Loss,
                (Shape::Paper, Shape::Paper) => &Result::Tie,
                (Shape::Paper, Shape::Scissors) => &Result::Win,
                (Shape::Scissors, Shape::Rock) => &Result::Win,
                (Shape::Scissors, Shape::Paper) => &Result::Loss,
                (Shape::Scissors, Shape::Scissors) => &Result::Tie,
            },
            Game::V2 {
                opponent_shape: _,
                result,
            } => result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_rock_points() {
        let game = Game::V1 {
            opponent_shape: Shape::Rock,
            own_shape: Shape::Rock,
        };
        assert_eq!(game.get_game_result().get_points(), 3);
        assert_eq!(game.get_points(), 4);
    }

    #[test]
    fn test_rock_paper_points() {
        let game = Game::V1 {
            opponent_shape: Shape::Rock,
            own_shape: Shape::Paper,
        };
        assert_eq!(game.get_game_result().get_points(), 6);
        assert_eq!(game.get_points(), 8);
    }

    #[test]
    fn test_rock_scissors_points() {
        let game = Game::V1 {
            opponent_shape: Shape::Rock,
            own_shape: Shape::Scissors,
        };
        assert_eq!(game.get_game_result().get_points(), 0);
        assert_eq!(game.get_points(), 3);
    }

    #[test]
    fn test_paper_rock_points() {
        let game = Game::V1 {
            opponent_shape: Shape::Paper,
            own_shape: Shape::Rock,
        };
        assert_eq!(game.get_game_result().get_points(), 0);
        assert_eq!(game.get_points(), 1);
    }

    #[test]
    fn test_paper_paper_points() {
        let game = Game::V1 {
            opponent_shape: Shape::Paper,
            own_shape: Shape::Paper,
        };
        assert_eq!(game.get_game_result().get_points(), 3);
        assert_eq!(game.get_points(), 5);
    }

    #[test]
    fn test_paper_scissors_points() {
        let game = Game::V1 {
            opponent_shape: Shape::Paper,
            own_shape: Shape::Scissors,
        };
        assert_eq!(game.get_game_result().get_points(), 6);
        assert_eq!(game.get_points(), 9);
    }

    #[test]
    fn test_scissors_rock_points() {
        let game = Game::V1 {
            opponent_shape: Shape::Scissors,
            own_shape: Shape::Rock,
        };
        assert_eq!(game.get_game_result().get_points(), 6);
        assert_eq!(game.get_points(), 7);
    }

    #[test]
    fn test_scissors_paper_points() {
        let game = Game::V1 {
            opponent_shape: Shape::Scissors,
            own_shape: Shape::Paper,
        };
        assert_eq!(game.get_game_result().get_points(), 0);
        assert_eq!(game.get_points(), 2);
    }

    #[test]
    fn test_scissors_scissors_points() {
        let game = Game::V1 {
            opponent_shape: Shape::Scissors,
            own_shape: Shape::Scissors,
        };
        assert_eq!(game.get_game_result().get_points(), 3);
        assert_eq!(game.get_points(), 6);
    }

    #[test]
    fn test_rock_loss_points() {
        let game = Game::V2 {
            opponent_shape: Shape::Rock,
            result: Result::Loss,
        };
        assert_eq!(game.get_own_shape().get_points(), 3);
        assert_eq!(game.get_points(), 3);
    }

    #[test]
    fn test_rock_tie_points() {
        let game = Game::V2 {
            opponent_shape: Shape::Rock,
            result: Result::Tie,
        };
        assert_eq!(game.get_own_shape().get_points(), 1);
        assert_eq!(game.get_points(), 4);
    }

    #[test]
    fn test_rock_win_points() {
        let game = Game::V2 {
            opponent_shape: Shape::Rock,
            result: Result::Win,
        };
        assert_eq!(game.get_own_shape().get_points(), 2);
        assert_eq!(game.get_points(), 8);
    }

    #[test]
    fn test_paper_loss_points() {
        let game = Game::V2 {
            opponent_shape: Shape::Paper,
            result: Result::Loss,
        };
        assert_eq!(game.get_own_shape().get_points(), 1);
        assert_eq!(game.get_points(), 1);
    }

    #[test]
    fn test_paper_tie_points() {
        let game = Game::V2 {
            opponent_shape: Shape::Paper,
            result: Result::Tie,
        };
        assert_eq!(game.get_own_shape().get_points(), 2);
        assert_eq!(game.get_points(), 5);
    }

    #[test]
    fn test_paper_win_points() {
        let game = Game::V2 {
            opponent_shape: Shape::Paper,
            result: Result::Win,
        };
        assert_eq!(game.get_own_shape().get_points(), 3);
        assert_eq!(game.get_points(), 9);
    }

    #[test]
    fn test_scissors_loss_points() {
        let game = Game::V2 {
            opponent_shape: Shape::Scissors,
            result: Result::Loss,
        };
        assert_eq!(game.get_own_shape().get_points(), 2);
        assert_eq!(game.get_points(), 2);
    }

    #[test]
    fn test_scissors_tie_points() {
        let game = Game::V2 {
            opponent_shape: Shape::Scissors,
            result: Result::Tie,
        };
        assert_eq!(game.get_own_shape().get_points(), 3);
        assert_eq!(game.get_points(), 6);
    }

    #[test]
    fn test_scissors_win_points() {
        let game = Game::V2 {
            opponent_shape: Shape::Scissors,
            result: Result::Win,
        };
        assert_eq!(game.get_own_shape().get_points(), 1);
        assert_eq!(game.get_points(), 7);
    }
}
