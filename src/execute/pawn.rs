use crate::{Board, OptSq, Piece, Sq, SqLike, Team};

pub fn get_translations<S: SqLike>(board: &Board, from: Sq, team: Team, piece: Piece) -> Vec<S> {
    let mut vec: Vec<S> = Vec::new();

    if board.find(from, Some(&team), Some(piece)).is_none() {
        return vec;
    }

    let mut can_en_passant_to: Option<Sq> = None;

    // If current pawn move is in position to en passant.
    if team == Team::Black && from.digit == 3 || team == Team::White && from.digit == 4 {
        if let Some(last_sq) = board.en_passant_target_square {
            can_en_passant_to = Some(last_sq);
        }
    }

    let mut lambda = |rank: isize, file: isize| {
        if let Some(sq2) = Sq::try_into(rank, file) {
            if let Some(en_pass_sq) = can_en_passant_to {
                if sq2 == en_pass_sq {
                    vec.push(S::into(sq2, Some(Piece::Pawn)));
                    return;
                }
            };

            if let Some(entity) = board.get(sq2) {
                if entity.team != team {
                    vec.push(S::into(sq2, Some(entity.kind)));
                }
            }
        };
    };

    // check corners.
    let mul: isize = match team {
        Team::White => 1,
        Team::Black => -1,
    };
    let sq = Sq::new((from.digit as isize + mul) as usize, from.letter);
    // Check diagonals for captures.
    lambda(sq.digit as isize, sq.letter as isize - 1);
    lambda(sq.digit as isize, sq.letter as isize + 1);

    if board.find(sq, None, None).is_none() {
        vec.push(S::into(sq, None));
        // We can only go 2 squares if the first one is empty.
        if team == Team::Black && from.digit == 6 || team == Team::White && from.digit == 1 {
            let sq2 = Sq::new((from.digit as isize + 2isize * mul) as usize, from.letter);
            if board.find(sq2, None, None).is_none() {
                vec.push(S::into(sq2, None));
            }
        }
    }

    vec
}

pub fn locate(board: &Board, to: Sq, from: OptSq, team: Team, piece: Piece) -> Option<Sq> {
    // Locate suitable pawn
    for x in 0..8 {
        let from = Sq::new(x, from.letter.or(Some(to.letter)).unwrap());
        if let Some(_entity) = board.find(from, Some(&board.turn_order), Some(piece)) {
            if get_translations(&board, from, team, piece).contains(&to) {
                return Some(from);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::execute::*;
    use crate::run;
    #[test]
    fn test_many_generic_pawn_moves() {
        let mut board = Board::new();
        run!(board, "d4", "e5", "dxe5", "f6", "exf6", "gxf6");
    }
    #[test]
    fn test_pawn_wall() {
        let mut board = Board::new();
        run!(board, "a4", "a5", "b4", "b5", "c4", "c5");
    }
    #[test]
    fn test_en_passant_white() {
        let mut board = Board::new();
        run!(board, "e4", "d5", "e5", "f5", "exf6");
    }
    #[test]
    fn test_en_passant_black() {
        let mut board = Board::new();
        run!(board, "e4", "d5", "e5", "d4", "c4", "dxc3");
    }

    #[test]
    fn test_en_passant_white_2() {
        let mut board = Board::new();
        run!(board, "e4", "a6", "e5", "d5", "exd6");
    }
    #[test]
    fn test_en_passant_black_verify_board() {
        let mut board = Board::new();
        run!(board, "c3", "d5", "c4", "d4", "e4", "dxe3");
        assert_eq!(
            board.find(
                Sq::notation("e4").unwrap(),
                Some(&Team::White),
                Some(Piece::Pawn)
            ),
            None,
            "Pawn should be removed when en passanted"
        )
    }
}
