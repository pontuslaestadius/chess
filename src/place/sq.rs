use std::io::{Error, ErrorKind, Result};
use std::ops::{Add, Sub};
use std::{char, cmp, fmt};

use crate::OptSq;

#[derive(Clone, Copy, PartialEq)]
pub struct Sq {
    pub digit: usize,
    pub letter: usize,
}

// #[derive(Clone, Copy, PartialEq)]
// pub strict WeightedSq {
//     sq: sq,
//     weight: usize,
// }

impl Sq {
    pub fn new(digit: usize, letter: usize) -> Self {
        Sq {
            digit: cmp::min(7, digit),
            letter: cmp::min(7, letter),
        }
    }
    #[allow(dead_code)]
    pub fn can_add(&self, oth: &Sq) -> bool {
        self.digit + oth.digit < 8 && self.letter + oth.letter < 8
    }
    #[allow(dead_code)]
    pub fn can_sub(&self, oth: &Sq) -> bool {
        oth.digit <= self.digit && oth.letter <= self.letter
    }
    #[allow(dead_code)]
    pub fn notation(notation: &str) -> Result<Self> {
        let mut chars = notation.chars().rev();
        let rank = match chars.next() {
            Some(c) => c,
            None => return Err(Error::new(ErrorKind::Other, "len was 0, must be 2")),
        };
        let rank = match rank.to_digit(10) {
            Some(d) => (d as usize - 1),
            None => return Err(Error::new(ErrorKind::Other, "invalid notation")),
        };
        let file = match chars.next() {
            Some(c) => c,
            None => return Err(Error::new(ErrorKind::Other, "len was 1, must be 2")),
        };
        let file = match file.to_digit(18) {
            Some(d) => d as usize - 10,
            None => return Err(Error::new(ErrorKind::Other, "not letter")),
        };

        Ok(Sq::new(rank, file))
    }
    pub fn dark_square(&self) -> bool {
        self.digit % 2 == 0 && self.letter % 2 == 0 || self.digit % 2 != 0 && self.letter % 2 != 0
    }
    pub fn valid_idx<T: Into<isize>>(rank: T, file: T) -> bool {
        let rank = rank.into();
        let file = file.into();
        !(rank > 7 || rank < 0 || file > 7 || file < 0)
    }
    pub fn union(&self, other: OptSq) -> Sq {
        let mut sq: Sq = *self;
        if let Some(digit) = other.digit {
            sq.digit = digit;
        }
        if let Some(letter) = other.letter {
            sq.letter = letter;
        }
        sq
    }
    pub fn mutate(&self, rank: isize, file: isize) -> Option<Sq> {
        let i_rank: isize = self.digit as isize + rank as isize;
        let i_file: isize = self.letter as isize + file as isize;
        if !Sq::valid_idx(i_rank, i_file) {
            return None;
        };
        Some(Sq::new(i_rank as usize, i_file as usize))
    }
}

impl Sub for Sq {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            digit: self.digit.checked_sub(other.digit).or(Some(0)).unwrap(),
            letter: self.letter.checked_sub(other.letter).or(Some(0)).unwrap(),
        }
    }
}

impl Add for Sq {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            digit: cmp::min(7, self.digit + other.digit),
            letter: cmp::min(7, self.letter + other.letter),
        }
    }
}

impl fmt::Debug for Sq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", (97 + self.letter as u8) as char, self.digit + 1)
    }
}

impl fmt::Display for Sq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", (97 + self.letter as u8) as char, self.digit + 1)
    }
}
