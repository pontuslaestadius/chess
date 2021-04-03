use crate::input::{chars_to_sq, position_indicator};
use crate::{Board, Entity, OptSq, Piece, Sq, Team};
use std::io::{Error, ErrorKind, Result};
use std::iter::Rev;
use std::str::Chars;

pub fn castle(mut board: &mut Board, str: &str) -> Result<()> {
    let team = board.turn_order;
    let rank = match team {
        Team::White => 0,
        Team::Black => 7,
    };
    let short: bool = !str.starts_with("O-O-O");
    let [king_file, rook_from_file, rook_to_file] = match short {
        true => [6, 7, 5],
        false => [2, 0, 3],
    };

    // TODO: Validate that the pieces haven't moved previously.
    match board.find(
        Sq::new(rank, rook_from_file),
        Some(&board.turn_order),
        Some(Piece::Rook),
    ) {
        Some(_rook_sq) => {
            let king_from = Sq::new(rank, 4);
            let king_to = Sq::new(rank, king_file);
            let rook_to = Sq::new(rank, rook_to_file);
            match board.find(king_from, Some(&team), Some(Piece::King)) {
                Some(_king_sq) => {
                    // Check that the to position is not occupied.
                    let mut other = board.clone();
                    match other.get(king_to) {
                        Some(ent) => {
                            let msg = format!(
                                "Castling blocked, King square {} occupied by {:?}",
                                king_to, ent
                            );
                            return Err(Error::new(ErrorKind::Other, msg));
                        }
                        None => match other.get(rook_to) {
                            Some(ent) => {
                                let msg = format!(
                                    "Castling blocked, Rook square {} occupied by {:?}",
                                    rook_to, ent
                                );
                                return Err(Error::new(ErrorKind::Other, msg));
                            }
                            None => {
                                let ent = Entity::new(Piece::King, team);
                                // Pretend that the King and Rook spot are both occupied by a King, to see that we cannot castle into check.
                                other.place(king_to, ent);
                                other.place(rook_to, ent);
                                if other.in_check(team) {
                                    return Err(Error::new(
                                        ErrorKind::Other,
                                        "Cannot castle into check",
                                    ));
                                }
                            }
                        },
                    }

                    board.translate(king_from, king_to)?;
                    board.turn_order = team;
                    board.translate(Sq::new(rank, rook_from_file), rook_to)?;
                    board.history.push(
                        team,
                        Piece::King,
                        king_from,
                        king_to,
                        Some(str.to_string()),
                    );

                    Ok(())
                }
                None => Err(Error::new(ErrorKind::Other, "no King to castle with")),
            }
        }
        None => Err(Error::new(ErrorKind::Other, "no Rook to castle with")),
    }
}

/*
Expected input:
cxd8=Q+
cxd8=Q
d8=Q
*/
pub fn promote(board: &mut Board, mut chars: &mut Rev<Chars>) -> Result<()> {
    let piece = match Piece::from_char(chars.next().unwrap()) {
        Some(p) => p,
        None => match Piece::from_char(chars.next().unwrap()) {
            Some(p) => p,
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "promotions with check, invalid format",
                ))
            }
        },
    };

    if chars.next().unwrap() != '=' {
        return Err(Error::new(
            ErrorKind::Other,
            "invalid promotion chars, second to last char must be '='",
        ));
    }

    let target = chars_to_sq(&mut chars)?;
    let mut opt_sq = OptSq::new();
    if let Some(ch) = chars.next() {
        if ch == 'x' {
            let next = chars.next().unwrap();
            opt_sq = position_indicator(next);
        }
    }

    if target.digit == 7 {
        opt_sq.digit = Some(6);
    } else if target.digit == 0 {
        opt_sq.digit = Some(1);
    } else {
        return Err(Error::new(ErrorKind::Other, "not in position to promote"));
    }

    let from = target.union(opt_sq);

    // Run execute on the remaining chars, if that succeeds, we transform that to Piece.
    #[cfg(test)]
    println!(
        "[execute/special]: promotion from {} to {} into {:?}",
        from, target, piece
    );

    board.translate(from, target)?;
    board.place(target, Entity::new(piece, board.turn_order.not()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::board::history::Move;
    use crate::execute::*;
    use crate::input::*;
    use crate::run;
    use crate::{Board, Entity};
    use std::io::Result;
    #[test]
    fn test_short_castle() {
        let mut board = Board::new();
        run!(board, "e4", "e5", "Be2", "Be7", "Nf3", "Nf6", "O-O", "O-O", "Re1", "Re8");
    }
    #[test]
    fn test_short_castle_verify_history() -> Result<()> {
        let mut board = Board::new();
        run!(board, "e4", "e5", "Be2", "Be7", "Nf3", "Nf6", "O-O");
        let last_move = board.history.last(Team::White).unwrap();
        let expected_move = Move {
            piece: Piece::King,
            from: Sq::notation("e1")?,
            to: Sq::notation("g1")?,
            label: Some("O-O".to_string()),
        };
        assert_eq!(
            last_move, &expected_move,
            "Last move should be registered as 'O-O'"
        );
        assert_eq!(board.history.len(Team::White), 4);
        assert_eq!(board.history.len(Team::Black), 3);

        Ok(())
    }
    #[test]
    fn test_cannot_castle_into_check() {
        let mut board = Board::new();
        run!(board, "e4", "d5", "Ba6", "Bd7", "Nf3", "Bb5");
        // Verify that the board is the same after failing to castle.
        let before_board = board.clone();
        match execute(&mut board, "O-O".chars()) {
            Ok(_r) => panic!("Should not be able to castle, because of Bb5 is blocking the path."),
            Err(_e) => (),
        }
        let after_board = board;
        assert_eq!(
            before_board, after_board,
            "If we fail to castle, the board should remain the same"
        );
    }
    #[test]
    fn test_long_castle() {
        let mut board = Board::new();
        run!(
            board, "d4", "d5", "Bf4", "Bf5", "Nc3", "Nc6", "Qd3", "Qd6", "O-O-O", "O-O-O", "Kb1",
            "Kb8", "Rd2", "Rd7"
        );
    }
    #[test]
    fn test_promotion() {
        let mut board = Board::new();
        run!(board, "a4", "b5", "axb5", "a6", "bxa6", "Bb7", "axb7", "Qc8", "bxc8=Q#");
        assert_eq!(
            board.get(chars_to_sq(&mut "c8".chars().rev()).unwrap()),
            Some(Entity::new(Piece::Queen, Team::White))
        );
        assert_eq!(
            board.check_mate(Team::Black),
            true,
            "black should be in check mate after bxc8=Q#"
        )
    }
    #[test]
    fn test_promotion_pairs() {
        let mut board = Board::new();
        board.clear();
        board.place(Sq::new(0, 0), Entity::new(Piece::Pawn, Team::White));
        board.place(Sq::new(7, 7), Entity::new(Piece::Pawn, Team::Black));

        run!(
            board, "a2", "h7", "a3", "h6", "a4", "h5", "a5", "h4", "a6", "h3", "a7", "h2", "a8=Q",
            "h1=N"
        );
        assert_eq!(
            board.get(chars_to_sq(&mut "a8".chars().rev()).unwrap()),
            Some(Entity::new(Piece::Queen, Team::White))
        );
        assert_eq!(
            board.get(chars_to_sq(&mut "h1".chars().rev()).unwrap()),
            Some(Entity::new(Piece::Knight, Team::Black))
        );
    }
}
