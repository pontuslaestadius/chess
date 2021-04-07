use crate::input;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OptSq {
    pub digit: Option<usize>,
    pub letter: Option<usize>,
}

impl OptSq {
    pub fn new() -> Self {
        OptSq {
            digit: None,
            letter: None,
        }
    }
    #[allow(dead_code)]
    pub fn overwrite(&mut self, other: OptSq) {
        #[cfg(test)]
        println!("[place/optsq]: {:?} ", other);
        if let Some(digit) = other.digit {
            self.digit = Some(digit);
        }
        if let Some(letter) = other.letter {
            self.letter = Some(letter);
        }
    }
}

impl From<char> for OptSq {
    fn from(item: char) -> Self {
        let mut opt_sq = OptSq::new();
        match item {
            'a'..='h' => opt_sq.letter = Some(input::letter_index(item).unwrap()),
            '1'..='8' => {
                if let Some(dig) = item.to_digit(10) {
                    opt_sq.digit = Some(dig as usize - 1);
                }
            }
            _ => panic!("invalid rank/file indicator {}", item),
        }
        opt_sq
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_char() {
        let opt_sq: OptSq = 'a'.into();
        assert_eq!(
            opt_sq,
            OptSq {
                digit: None,
                letter: Some(0)
            }
        );

        let opt_sq: OptSq = 'h'.into();
        assert_eq!(
            opt_sq,
            OptSq {
                digit: None,
                letter: Some(7)
            }
        );

        let opt_sq: OptSq = 'f'.into();
        assert_eq!(
            opt_sq,
            OptSq {
                digit: None,
                letter: Some(5)
            }
        );

        let opt_sq: OptSq = '1'.into();
        assert_eq!(
            opt_sq,
            OptSq {
                digit: Some(0),
                letter: None
            }
        );

        let opt_sq: OptSq = '8'.into();
        assert_eq!(
            opt_sq,
            OptSq {
                digit: Some(7),
                letter: None
            }
        );
    }
}
