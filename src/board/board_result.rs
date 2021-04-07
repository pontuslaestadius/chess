
pub struct BoardResult {
    winner: Option<Team>,
    result: EndState,
}

pub enum EndState {
    Checkmate,
    Active,
    Draw(DrawState),
}

pub enum DrawState {
    Repetion,
    FiftyMoveRule,
    Stalemate,
}
