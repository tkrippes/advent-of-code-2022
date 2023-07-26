#[derive(Debug, PartialEq, Clone)]
pub enum Result {
    Loss,
    Tie,
    Win,
}

impl Result {
    pub fn get_points(&self) -> u32 {
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
    fn test_loss_points() {
        let loss = Result::Loss;
        assert_eq!(loss.get_points(), 0);
    }

    #[test]
    fn test_tie_points() {
        let tie = Result::Tie;
        assert_eq!(tie.get_points(), 3);
    }

    #[test]
    fn test_win_points() {
        let win = Result::Win;
        assert_eq!(win.get_points(), 6);
    }
}
