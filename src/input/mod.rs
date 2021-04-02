use crate::{KingStatus, OptSq, Sq};
use std::io;
use std::io::{Error, ErrorKind, Result};
use std::iter::Rev;
use std::str::Chars;

pub mod pgn;

pub fn read() -> Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(e),
    }
}

pub fn position_indicator(ch: char) -> OptSq {
    // It has an rank/file indicator.
    let mut opt_sq = OptSq::new();
    match ch {
        'a'..='h' => opt_sq.letter = Some(letter_index(ch).unwrap()),
        '1'..='8' => {
            if let Some(dig) = ch.to_digit(10) {
                opt_sq.digit = Some(dig as usize - 1);
            }
        }
        _ => panic!("invalid rank/file indicator {}", ch),
    }
    #[cfg(test)]
    println!("[input/mod]: position_indicator: {:?} ", opt_sq);
    opt_sq
}

pub fn letter_index(ch: char) -> Option<usize> {
    match ch.to_digit(18) {
        Some(x) => Some(x as usize - 10),
        None => None,
    }
}

pub fn get_king_status(s: &str) -> KingStatus {
    if s.contains('+') {
        KingStatus::Check
    } else if s.contains('#') {
        KingStatus::Mate
    } else {
        KingStatus::Safe
    }
}

pub fn to_index_pos(chars: &mut Rev<Chars>) -> Result<[usize; 2]> {
    // #[cfg(test)]
    // println!("[input/mod]: {:?}", chars);

    let mut digit_raw = match chars.next() {
        Some(d) => d,
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "not enough chars in input for digit indice",
            ));
        }
    };

    // TODO: Ignore checks and mate for now, implement later.
    match digit_raw {
        '+' | '#' => {
            digit_raw = match chars.next() {
                Some(d) => d,
                None => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "not enough chars in input after check/mate",
                    ))
                }
            }
        }
        _ => (),
    }

    let digit = match digit_raw.to_digit(10) {
        Some(d) => d as usize,
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("last char ({}) not digit", digit_raw),
            ));
        }
    };

    let letter_raw = match chars.next() {
        Some(l) => l,
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "not enough chars in input for letter indice",
            ));
        }
    };

    let letter = letter_index(letter_raw).unwrap();
    Ok([digit - 1, letter])
}

pub fn chars_to_sq(mut chars: &mut Rev<Chars>) -> Result<Sq> {
    let [digit, letter] = to_index_pos(&mut chars)?;
    Ok(Sq::new(digit, letter))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;

    #[test]
    fn test_letter_index() {
        assert_eq!(letter_index('a'), Some(0), "'a' should be index 0");
        assert_eq!(letter_index('f'), Some(5), "'f' should be index 5");
        assert_eq!(letter_index('h'), Some(7), "'h' should be index 7");
        assert_eq!(letter_index('j'), None, "'j' should be out of bound");
    }
    #[test]
    fn test_position_indicator() {
        assert_eq!(
            position_indicator('a'),
            OptSq {
                digit: None,
                letter: Some(0)
            }
        );
        assert_eq!(
            position_indicator('h'),
            OptSq {
                digit: None,
                letter: Some(7)
            }
        );
        assert_eq!(
            position_indicator('f'),
            OptSq {
                digit: None,
                letter: Some(5)
            }
        );
        assert_eq!(
            position_indicator('1'),
            OptSq {
                digit: Some(0),
                letter: None
            }
        );
        assert_eq!(
            position_indicator('8'),
            OptSq {
                digit: Some(7),
                letter: None
            }
        );
    }
    #[test]
    fn test_to_index_pos_valid() -> Result<()> {
        assert_eq!(
            to_index_pos(&mut "a1".chars().rev())?,
            [0 as usize, 0 as usize]
        );
        assert_eq!(
            to_index_pos(&mut "a2".chars().rev())?,
            [1 as usize, 0 as usize]
        );
        assert_eq!(
            to_index_pos(&mut "b4".chars().rev())?,
            [3 as usize, 1 as usize]
        );
        assert_eq!(
            to_index_pos(&mut "d2".chars().rev())?,
            [1 as usize, 3 as usize]
        );
        assert_eq!(
            to_index_pos(&mut "a8".chars().rev())?,
            [7 as usize, 0 as usize]
        );
        assert_eq!(
            to_index_pos(&mut "h7".chars().rev())?,
            [6 as usize, 7 as usize]
        );
        Ok(())
    }
}
