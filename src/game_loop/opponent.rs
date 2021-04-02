use crate::computer;
use crate::Board;
use std::io;
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
    #[allow(dead_code)]
    fn opponent_to_action(&self) -> &'static dyn Fn(&mut Board) -> io::Result<()> {
        match self {
            Opponent::Player => &computer::player::action,
            Opponent::Computer => &computer::dummy::action,
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
