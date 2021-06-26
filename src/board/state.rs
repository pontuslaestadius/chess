use crate::Team;

#[derive(Clone, Debug, PartialEq)]
pub struct BoardState {
    winner: Option<Team>,
    result: GameState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameState {
    Checkmate,
    Active,
    Draw(DrawState),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DrawState {
    ThreefoldRepetion,
    FiftyMoveRule,
    Stalemate,
    Agreement,
    InsufficientMatingMaterial,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            winner: None,
            result: GameState::Active,
        }
    }
}
