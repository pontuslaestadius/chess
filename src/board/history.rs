use crate::{Piece, Sq, Team};

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub from: Sq,
    pub to: Sq,
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct History {
    white_moves: Vec<Move>,
    black_moves: Vec<Move>,
}

impl History {
    pub fn new() -> Self {
        History {
            white_moves: Vec::new(),
            black_moves: Vec::new(),
        }
    }

    fn resolve_type(&self, team: Team) -> &Vec<Move> {
        match team {
            Team::White => &self.white_moves,
            Team::Black => &self.black_moves,
        }
    }

    fn resolve_type_mut(&mut self, team: Team) -> &mut Vec<Move> {
        match team {
            Team::White => &mut self.white_moves,
            Team::Black => &mut self.black_moves,
        }
    }

    pub fn last(&self, team: Team) -> Option<&Move> {
        let vec = self.resolve_type(team);
        vec.last()
    }

    pub fn push(&mut self, team: Team, piece: Piece, from: Sq, to: Sq, label: Option<String>) {
        let mov = Move {
            piece,
            from,
            to,
            label,
        };
        self.resolve_type_mut(team).push(mov);
    }

    #[allow(dead_code)]
    pub fn pop(&mut self, team: Team) -> Option<Move> {
        self.resolve_type_mut(team).pop()
    }

    #[allow(dead_code)]
    pub fn tuple(&self, idx: usize) -> [Option<&Move>; 2] {
        [self.white_moves.get(idx), self.black_moves.get(idx)]
    }

    pub fn get(&self, team: Team, idx: usize) -> Option<&Move> {
        self.resolve_type(team).get(idx)
    }

    pub fn len(&self, team: Team) -> usize {
        self.resolve_type(team).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute::*;
    use crate::{run, Board, Piece, Sq, Team};
    #[test]
    fn test_last_history() -> Result<(), std::io::Error> {
        let mut board = Board::new();
        let last_move = board.history.last(board.turn_order);
        assert_eq!(last_move, None);
        let last_move = board.history.last(board.not_turn());
        assert_eq!(last_move, None);
        run!(board, "a4");
        let last_move = board.history.last(Team::White);
        let expected_mov = Move {
            piece: Piece::Pawn,
            from: Sq::notation("a2")?,
            to: Sq::notation("a4")?,
            label: Some("a4".to_string()),
        };
        assert_eq!(last_move, Some(&expected_mov));
        Ok(())
    }
}
