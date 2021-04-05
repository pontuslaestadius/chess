use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Team {
    White,
    Black,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            Team::White => "White",
            Team::Black => "Black",
        };
        write!(f, "{}", t)
    }
}

impl Team {
    pub fn not(&self) -> Team {
        match self {
            Team::White => Team::Black,
            Team::Black => Team::White,
        }
    }
}

impl Team {
    #[allow(dead_code)]
    pub fn abrev(&self) -> &str {
        match self {
            Team::White => "W",
            Team::Black => "B",
        }
    }
}
