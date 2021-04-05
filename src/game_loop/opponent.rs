use crate::computer;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Opponent {
    Player,
    Computer,
}

impl FromStr for Opponent {
    type Err = ();

    fn from_str(input: &str) -> Result<Opponent, Self::Err> {
        match input {
            "player" => Ok(Opponent::Player),
            "computer" => Ok(Opponent::Computer),
            _ => Err(()),
        }
    }
}

impl Opponent {
    pub fn init(self) -> Box<dyn computer::Playable> {
        match self {
            Opponent::Player => Box::new(computer::player::Player::new()),
            Opponent::Computer => Box::new(computer::Computer::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_player_from_string() {
        assert_eq!(Opponent::from_str("player"), Ok(Opponent::Player));
    }
    #[test]
    fn test_parse_computer_from_string() {
        assert_eq!(Opponent::from_str("computer"), Ok(Opponent::Computer));
    }
    #[test]
    fn test_parse_invalid_from_string() {
        assert_eq!(Opponent::from_str("zebra").is_err(), true);
    }
}
