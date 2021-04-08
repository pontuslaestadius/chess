use crate::{Board, OptSq, Piece, Sq, Team};

pub fn get_translations(board: &Board, from: Sq, team: Team, piece: Option<Piece>) -> Vec<Sq> {
    let mut vec = Vec::new();

    if board.find(from, Some(&team), piece).is_none() {
        return vec;
    }

    let mut last_move: Option<Sq> = None;

    // If current pawn move is in position to en passant.
    if team == Team::Black && from.digit == 3 || team == Team::White && from.digit == 4 {
        // Get last move for opposite team.
        last_move = match board.history.last(board.not_turn()) {
            Some(mov) => {
                // Only pawn moves that moved 2 squares.
                let two_or_not_to_two = isize::abs(mov.from.digit as isize - mov.to.digit as isize);
                if mov.piece == Piece::Pawn
                    && two_or_not_to_two == 2
                    && board.en_passant_target_square.is_some()
                {
                    #[cfg(test)]
                    println!("[execute/pawn]: en passant allowed");
                    Some(Sq::new((mov.to.digit + mov.from.digit) / 2, mov.to.letter))
                } else {
                    None
                }
            }
            None => None,
        };
    }

    let mut lambda = |digit, letter| {
        let sq2 = Sq::new(digit, letter);

        // One peasant
        if let Some(sq) = last_move {
            #[cfg(test)]
            println!("[execute/pawn]: sq2: {}, last_move {}", sq2, sq);
            // If the capture sqaure is the one just occupied by the pawn, it is en passant.
            if sq2 == sq {
                vec.push(sq2);
                return;
            }
        }

        if let Some(entity) = board.get(sq2) {
            if entity.team != team {
                vec.push(sq2);
            }
        }
    };

    // check corners.
    let mul: isize = match team {
        Team::White => 1,
        Team::Black => -1,
    };
    let sq = Sq::new((from.digit as isize + mul) as usize, from.letter);
    // Check diagonals for captures.
    // ioob checks.
    if sq.letter > 0 {
        lambda(sq.digit, sq.letter - 1);
    }
    if sq.letter < 7 {
        lambda(sq.digit, sq.letter + 1);
    }

    if board.find(sq, None, None).is_none() {
        vec.push(sq);
        // We can only go 2 squares if the first one is empty.
        if team == Team::Black && from.digit == 6 || team == Team::White && from.digit == 1 {
            let sq2 = Sq::new((from.digit as isize + 2isize * mul) as usize, from.letter);
            if board.find(sq2, None, None).is_none() {
                vec.push(sq2);
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
            let translations = get_translations(&board, from, team, Some(piece));
            if translations.contains(&to) {
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
