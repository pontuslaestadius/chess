use crate::input::{chars_to_sq, position_indicator};
use crate::{Board, Entity, OptSq, Piece, Sq, Team};
use std::io::{Error, ErrorKind, Result};
use std::iter::Rev;
use std::str::Chars;

pub fn castle(mut board: &mut Board, str: &str) -> Result<()> {
    let rank = match board.turn_order {
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
            match board.find(Sq::new(rank, 4), Some(&board.turn_order), Some(Piece::King)) {
                Some(_king_sq) => {
                    // TODO: Validate no pieces are in between the two of them.
                    let tmp = board.turn_order;
                    board.translate(Sq::new(rank, 4), Sq::new(rank, king_file))?;
                    board.turn_order = tmp;
                    board.translate(Sq::new(rank, rook_from_file), Sq::new(rank, rook_to_file))?;
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
    }

    let from = target.union(opt_sq);

    // Run execute on the remaining chars, if that succeeds, we transform that to Piece.
    #[cfg(test)]
    println!(
        "[execute/special]: promotion from {} to {} into {:?}",
        from, target, piece
    );

    let turn_order = board.turn_order;
    board.translate(from, target)?;
    board.place(target, Entity::new(piece, turn_order));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::execute::*;
    use crate::input::*;
    use crate::run;
    use crate::{Board, Entity};
    #[test]
    fn test_short_castle() {
        let mut board = Board::new();
        run!(board, "e4", "e5", "Be2", "Be7", "Nf3", "Nf6", "O-O", "O-O", "Re1", "Re8");
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
