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
}
