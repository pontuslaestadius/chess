use crate::Sq;
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

pub fn letter_index(ch: char) -> Option<usize> {
    match ch.to_digit(18) {
        Some(x) => Some(x as usize - 10),
        None => None,
    }
}

pub fn to_index_pos(chars: &mut Rev<Chars>) -> Result<[usize; 2]> {
    let mut digit_raw = match chars.next() {
        Some(d) => d,
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "not enough chars in input for digit indice",
            ));
        }
    };

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
