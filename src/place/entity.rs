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

impl From<char> for Entity {
    fn from(item: char) -> Self {
        let team: Team = match item {
            'a'..='z' => Team::Black,
            'A'..='Z' => Team::White,
            _ => panic!("Cannot decode Entity from char: {}", item),
        };
        let kind: Piece = item.into();

        Entity { kind, team }
    }
}
