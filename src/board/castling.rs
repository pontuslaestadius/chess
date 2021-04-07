use crate::Team;

#[derive(Clone, Debug, PartialEq)]
pub struct CastlingAvailability {
    pub long: bool,
    pub short: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Castling {
    pub white: CastlingAvailability,
    pub black: CastlingAvailability,
}

impl From<&str> for Castling {
    fn from(item: &str) -> Self {
        let mut ca = Castling {
            white: CastlingAvailability {
                long: false,
                short: false,
            },
            black: CastlingAvailability {
                long: false,
                short: false,
            },
        };
        for ch in item.chars() {
            match ch {
                'q' => ca.black.long = true,
                'Q' => ca.white.long = true,
                'k' => ca.black.short = true,
                'K' => ca.white.short = true,
                '-' => break,
                _ => break,
            }
        }
        ca
    }
}

impl Castling {
    pub fn new() -> Self {
        Castling {
            white: CastlingAvailability::new(),
            black: CastlingAvailability::new(),
        }
    }
    pub fn revoke(&mut self, team: Team) {
        match team {
            Team::White => self.white.revoke(),
            Team::Black => self.black.revoke(),
        }
    }
    pub fn fen(&self) -> String {
        let mut res = String::new();
        if !self.white.short && !self.white.long && !self.black.short && !self.black.long {
            res.push('-');
        } else {
            if self.white.short {
                res.push('K');
            }
            if self.white.long {
                res.push('Q');
            }
            if self.black.short {
                res.push('k');
            }
            if self.black.long {
                res.push('q');
            }
        }
        res
    }
}

impl CastlingAvailability {
    pub fn new() -> Self {
        CastlingAvailability {
            short: true,
            long: true,
        }
    }
    pub fn revoke(&mut self) {
        self.short = false;
        self.long = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initial_state() {
        let castling = Castling::new();
        assert_eq!(castling.fen(), "KQkq");
    }
    #[test]
    fn test_no_castling() {
        let mut castling = Castling::new();
        castling.revoke(Team::White);
        castling.revoke(Team::Black);
        assert_eq!(castling.fen(), "-");
    }
    #[test]
    fn test_from_str() {
        let expected_castling = Castling::new();
        let actual_castling: Castling = "KQkq".into();
        assert_eq!(expected_castling, actual_castling);
    }
    #[test]
    fn test_from_str_2() {
        let expected_castling = Castling {
            white: CastlingAvailability {
                long: false,
                short: false,
            },
            black: CastlingAvailability {
                long: false,
                short: false,
            },
        };
        let actual_castling: Castling = "-".into();
        assert_eq!(expected_castling, actual_castling);
    }
}
