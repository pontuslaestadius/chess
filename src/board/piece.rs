use crate::execute::{bishop, king, knight, pawn, queen, rook};
use crate::{Board, OptSq, Sq, Team};
use std::convert::From;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl FromStr for Piece {
    type Err = ();

    fn from_str(input: &str) -> Result<Piece, Self::Err> {
        match input {
            "B" => Ok(Piece::Bishop),
            "K" => Ok(Piece::King),
            "R" => Ok(Piece::Rook),
            "P" => Ok(Piece::Pawn),
            "Q" => Ok(Piece::Queen),
            "N" => Ok(Piece::Knight),
            _ => Err(()),
        }
    }
}

impl From<char> for Piece {
    fn from(item: char) -> Self {
        match item {
            'B' | 'b' => Piece::Bishop,
            'K' | 'k' => Piece::King,
            'R' | 'r' => Piece::Rook,
            'P' | 'p' => Piece::Pawn,
            'Q' | 'q' => Piece::Queen,
            'N' | 'n' => Piece::Knight,
            _ => panic!("cannot cast char '{}' into Piece", item),
        }
    }
}

impl From<Piece> for &str {
    fn from(item: Piece) -> Self {
        match item {
            Piece::Rook => "R",
            Piece::Bishop => "B",
            Piece::Knight => "N",
            Piece::King => "K",
            Piece::Queen => "Q",
            Piece::Pawn => "P",
        }
    }
}

impl Piece {
    pub fn from_char(input: char) -> Option<Piece> {
        match input {
            'B' => Some(Piece::Bishop),
            'K' => Some(Piece::King),
            'R' => Some(Piece::Rook),
            'P' => Some(Piece::Pawn),
            'Q' => Some(Piece::Queen),
            'N' => Some(Piece::Knight),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Piece::Rook => "R",
            Piece::Bishop => "B",
            Piece::Knight => "N",
            Piece::King => "K",
            Piece::Queen => "Q",
            Piece::Pawn => "",
        }
    }
    pub fn get_translations(
        &self,
    ) -> &'static (dyn Fn(&Board, Sq, Team, Option<Piece>) -> Vec<Sq> + 'static) {
        match self {
            Piece::Rook => &rook::get_translations,
            Piece::Bishop => &bishop::get_translations,
            Piece::Knight => &knight::get_translations,
            Piece::King => &king::get_translations,
            Piece::Queen => &queen::get_translations,
            Piece::Pawn => &pawn::get_translations,
        }
    }
    pub fn get_locate(&self) -> &'static (dyn Fn(&Board, Sq, OptSq, Team, Piece) -> Option<Sq>) {
        match self {
            Piece::Rook => &rook::locate,
            Piece::Bishop => &bishop::locate,
            Piece::Knight => &knight::locate,
            Piece::King => &king::locate,
            Piece::Queen => &queen::locate,
            Piece::Pawn => &pawn::locate,
        }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> usize {
        match self {
            Piece::Rook => 5,
            Piece::Bishop => 3,
            Piece::Knight => 3,
            Piece::King => 15,
            Piece::Queen => 10,
            Piece::Pawn => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_bishop() {
        assert_eq!(Piece::from_str("B"), Ok(Piece::Bishop));
    }
    #[test]
    fn test_parse_king() {
        assert_eq!(Piece::from_str("K"), Ok(Piece::King));
    }
    #[test]
    fn test_parse_rook() {
        assert_eq!(Piece::from_str("R"), Ok(Piece::Rook));
    }
    #[test]
    fn test_parse_pawn() {
        assert_eq!(Piece::from_str("P"), Ok(Piece::Pawn));
    }
    #[test]
    fn test_parse_queen() {
        assert_eq!(Piece::from_str("Q"), Ok(Piece::Queen));
    }
    #[test]
    fn test_parse_knight() {
        assert_eq!(Piece::from_str("N"), Ok(Piece::Knight));
    }
    #[test]
    fn test_parse_invalid() {
        assert_eq!(Piece::from_str("Z").is_err(), true);
    }
    #[test]
    fn test_char_illegal() {
        assert_eq!(Piece::from_char(' '), None);
        assert_eq!(Piece::from_char('x'), None);
        assert_eq!(Piece::from_char('d'), None);
    }
    #[test]
    fn test_char_legal() {
        assert_eq!(Piece::from_char('B'), Some(Piece::Bishop));
        assert_eq!(Piece::from_char('P'), Some(Piece::Pawn));
        assert_eq!(Piece::from_char('Q'), Some(Piece::Queen));
        assert_eq!(Piece::from_char('N'), Some(Piece::Knight));
        assert_eq!(Piece::from_char('K'), Some(Piece::King));
        assert_eq!(Piece::from_char('R'), Some(Piece::Rook));
    }
}
