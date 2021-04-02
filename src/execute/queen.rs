use crate::execute::{bishop, rook};
use crate::{Board, OptSq, Piece, Sq, Team};

pub fn get_translations(board: &Board, from: Sq, team: Team, piece: Option<Piece>) -> Vec<Sq> {
    let mut vec = bishop::get_translations(board, from, team, piece);
    vec.append(&mut rook::get_translations(board, from, team, piece));
    vec
}

pub fn locate(board: &Board, to: Sq, from: OptSq, team: Team, piece: Piece) -> Option<Sq> {
    if let Some(sq) = bishop::locate(&board, to, from, team, piece) {
        return Some(sq);
    };
    if let Some(sq) = rook::locate(&board, to, from, team, piece) {
        return Some(sq);
    };
    None
}

#[cfg(test)]
mod tests {
    use crate::execute::*;
    use crate::run;
    #[test]
    fn test_move_queen_straight() {
        let mut board = Board::new();
        run!(board, "d4", "d5", "Qd3", "Qd6");
    }
    #[test]
    fn test_move_queen_diagonal() {
        let mut board = Board::new();
        run!(board, "e4", "e5", "Qg4", "Qg5", "Qxg5");
    }
    #[test]
    fn test_move_queen_complex() {
        let mut board = Board::new();
        run!(
            board, "e4", "Nc6", "d4", "Nf6", "Nc3", "Nb8", "e5", "g6", "exf6", "e6", "Bg5", "c6",
            "Nf3", "Bd6", "Ne5", "Qa5", "a3", "a6", "b4", "Qd8", "Be2", "Bf8", "O-O", "h6", "Qd2",
            "d6", "Nxg6", "fxg6", "Bh4", "Rg8", "g3", "b6", "f7+", "Kd7", "Bxd8", "Ra7", "fxg8=Q",
            "Rc7", "Qf7+", "Kxd8", "Qxf8+", "Kd7", "Qdxh6"
        );
    }
    #[test]
    fn test_move_queen_complex_2() {
        let mut board = Board::new();
        run!(
            board, "h4", "a6", "g4", "a5", "e3", "f6", "f4", "c6", "d4", "a4", "g5", "h5", "Be2",
            "g6", "f5", "Bg7", "fxg6", "b5", "Bxh5", "Qb6", "Bf3", "Ra6", "h5", "Kd8", "h6", "Bf8",
            "g7", "Qa5+", "Bd2", "Qb6", "gxh8=Q", "f5", "Qxg8", "Ra5", "Qxf8+", "Kc7", "Qxe7",
            "Ra6", "h7", "Kb7", "h8=Q", "Ra7", "Qee8", "Ra6", "Qxc8+", "Ka7", "g6", "Qc7", "g7",
            "Qf4", "Qh3", "Qxf3", "g8=Q", "Qf4", "Qxb8+", "Qxb8", "Qdg4", "Qb6", "Q4g7", "Qc7",
            "Qxf5", "Kb6", "Qfxd7", "Qa7", "Qge6"
        );
    }
}
