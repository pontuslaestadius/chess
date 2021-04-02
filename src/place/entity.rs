use crate::place::sq::Sq;
use crate::{Piece, Team};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entity {
    pub kind: Piece,
    pub team: Team,
}

pub struct SqEntity {
    pub entity: Entity,
    pub sq: Sq,
}

impl Entity {
    pub fn new(kind: Piece, team: Team) -> Self {
        Entity { kind, team }
    }
}
