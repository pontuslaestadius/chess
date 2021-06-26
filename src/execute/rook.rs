use crate::{Board, OptSq, Piece, Sq, SqLike, Team};

pub fn get_translations<S: SqLike>(board: &Board, from: Sq, team: Team, piece: Piece) -> Vec<S> {
    let mut vec: Vec<S> = Vec::new();

    let team = match board.find(from, Some(&team), Some(piece)) {
        Some(entity) => entity.team,
        None => return vec,
    };

    let mut lambda = |digit: usize, letter: usize| -> bool {
        let target = Sq::new(digit, letter);
        match board.find(target, None, None) {
            None => {
                vec.push(S::into(target, None));
                true
            }
            Some(entity) => {
                // If it is an enemy piece, we can capture it, but then not go further as it is blocking.
                if entity.team != team {
                    vec.push(S::into(target, Some(entity.kind)));
                }
                false
            }
        }
    };

    // up
    for n in from.digit + 1..8 {
        if !lambda(n, from.letter) {
            break;
        }
    }
    // down
    if from.digit > 0 {
        for n in (0..from.digit).rev() {
            if !lambda(n, from.letter) {
                break;
            }
        }
    }
    // right
    for n in from.letter + 1..8 {
        if !lambda(from.digit, n) {
            break;
        }
    }
    // left
    if from.letter > 0 {
        for n in (0..from.letter).rev() {
            if !lambda(from.digit, n) {
                break;
            }
        }
    }

    vec
}

pub fn locate(board: &Board, to: Sq, from: OptSq, team: Team, piece: Piece) -> Option<Sq> {
    for x in 0..8 {
        let target = Sq::new(from.digit.or(Some(x))?, from.letter.or(Some(to.letter))?);
        if let Some(sq) = board.legal_target(target, to, team, piece) {
            return Some(sq);
        }
    }
    for x in 0..8 {
        let target = Sq::new(from.digit.or(Some(to.digit))?, from.letter.or(Some(x))?);
        if let Some(sq) = board.legal_target(target, to, team, piece) {
            return Some(sq);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::*;
    use crate::Entity;
    #[test]
    fn test_rook_no_translations() {
        let board = Board::new();
        let trans: Vec<Sq> = get_translations(&board, Sq::new(0, 0), board.turn_order, Piece::Rook);
        assert_eq!(trans, Vec::new());
    }
    #[test]
    fn test_rook_move_up() {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::new(0, 0);
        let team = Team::White;
        board.place(sq, Entity::new(Piece::Rook, team));
        let trans: Vec<Sq> = get_translations(&board, sq, board.turn_order, Piece::Rook);
        assert_eq!(
            trans.len(),
            14,
            "Rook at {} translations incorrect, received {:?}",
            sq,
            trans
        );
    }
    #[test]
    fn test_rook_move_down() {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::new(7, 7);
        let team = Team::Black;
        board.turn_order = team;
        board.place(sq, Entity::new(Piece::Rook, team));
        let trans: Vec<Sq> = get_translations(&board, sq, board.turn_order, Piece::Rook);
        assert_eq!(
            trans.len(),
            14,
            "Rook at {} translations incorrect, received {:?}",
            sq,
            trans
        );
    }
    #[test]
    fn test_rook_center_translations() {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::new(4, 4);
        let team = Team::Black;
        board.turn_order = team;
        board.place(sq, Entity::new(Piece::Rook, team));
        let trans: Vec<Sq> = get_translations(&board, sq, board.turn_order, Piece::Rook);
        assert_eq!(
            trans.len(),
            14,
            "Rook at {} translations incorrect, received {:?}",
            sq,
            trans
        );
    }
    #[test]
    fn test_rook_blocked_friendly() {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::new(4, 4);
        let team = Team::Black;
        board.turn_order = team;
        board.place(sq, Entity::new(Piece::Rook, team));
        let pawn = Entity::new(Piece::Pawn, team);
        board.place(sq - Sq::new(1, 0), pawn);
        board.place(sq - Sq::new(0, 1), pawn);
        board.place(sq + Sq::new(0, 1), pawn);
        board.place(sq + Sq::new(1, 0), pawn);

        let trans: Vec<Sq> = get_translations(&board, sq, board.turn_order, Piece::Rook);
        assert_eq!(
            trans.len(),
            0,
            "Rook at {} translations incorrect, received {:?}",
            sq,
            trans
        );
    }
    #[test]
    fn test_rook_blocked_friendly_longer() {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::new(4, 4);
        let team = Team::Black;
        board.turn_order = team;
        board.place(sq, Entity::new(Piece::Rook, team));
        let pawn = Entity::new(Piece::Pawn, team);
        board.place(sq - Sq::new(2, 0), pawn);
        board.place(sq - Sq::new(0, 2), pawn);
        board.place(sq + Sq::new(0, 2), pawn);
        board.place(sq + Sq::new(2, 0), pawn);

        let trans: Vec<Sq> = get_translations(&board, sq, board.turn_order, Piece::Rook);
        assert_eq!(
            trans.len(),
            4,
            "Rook at {} translations incorrect, received {:?}",
            sq,
            trans
        );
    }
    #[test]
    fn test_rook_wrong_piece() {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::new(4, 4);
        let team = Team::White;
        board.place(sq, Entity::new(Piece::Rook, team));
        let pawn = Entity::new(Piece::Pawn, Team::White);
        board.place(sq - Sq::new(2, 0), pawn);
        board.place(sq - Sq::new(0, 2), pawn);
        board.place(sq + Sq::new(0, 2), pawn);
        board.place(sq + Sq::new(2, 0), pawn);

        let trans: Vec<Sq> = get_translations(&board, sq, board.turn_order, Piece::Queen);
        assert_eq!(trans.len(), 0, "shouldn't be able to move enemy Rook");
    }
    #[test]
    fn test_rook_blocked_enemy() {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::new(4, 4);
        let team = Team::Black;
        board.turn_order = team;
        board.place(sq, Entity::new(Piece::Rook, team));
        let pawn = Entity::new(Piece::Pawn, Team::White);
        board.place(sq - Sq::new(2, 0), pawn);
        board.place(sq - Sq::new(0, 2), pawn);
        board.place(sq + Sq::new(0, 2), pawn);
        board.place(sq + Sq::new(2, 0), pawn);

        let trans: Vec<Sq> = get_translations(&board, sq, board.turn_order, Piece::Rook);
        assert_eq!(
            trans.len(),
            8,
            "Rook at {} translations incorrect, received {:?}",
            sq,
            trans
        );
    }
}
