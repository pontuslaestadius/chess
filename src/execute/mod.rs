use crate::input;
use crate::{Board, KingStatus, OptSq, Piece, Sq, Team};
use std::io::{Error, ErrorKind, Result};
use std::str::Chars;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;
pub mod special;

#[macro_export]
macro_rules! run {
    ($board:ident, $( $x:expr ),* ) => {
        {
            $(
                match execute(&mut $board, $x.chars()) {
                    Ok(_) => (),
                    Err(e) => {
                        crate::display::present(&$board);
                        panic!("{} failed with error: '{}'", $x, e);
                    },
                };
            )*
        }
    };
}

fn locator(board: &Board, to: Sq, from: OptSq, team: Team, piece: Piece) -> Option<Sq> {
    match piece.get_locate()(&board, to, from, team, piece) {
        Some(t) => Some(t),
        None => None,
    }
}

fn locator_from_char(board: &Board, to: Sq, from: OptSq, piece: char) -> Option<Sq> {
    match Piece::from_char(piece) {
        Some(p) => locator(board, to, from, board.turn_order, p),
        None => None,
    }
}

pub enum EResult {
    Ok,
    Stalemate,
    Checkmate,
}

pub fn execute(mut board: &mut Board, input: Chars) -> Result<EResult> {
    let str_input = input.as_str();
    let mut input = input.rev();
    let king_status = input::get_king_status(str_input);

    // let in_check = board.in_check(board.turn_order);

    // FIXME: use rev chars.
    if str_input.contains('=') {
        // piece promotion.
        special::promote(&mut board, &mut input)?;
        return Ok(EResult::Ok);
    } else if str_input.starts_with("O-O") {
        // castling
        special::castle(&mut board, str_input)?;
        return Ok(EResult::Ok);
    }

    let target = input::chars_to_sq(&mut input)?;

    #[cfg(test)]
    println!(
        "[execute/mod]: {:?} - target {:?} remaining {:?}",
        str_input, target, input
    );

    let mut hold: Option<Sq> = None;
    let mut from: OptSq = OptSq::new();

    match input.next() {
        // Captures, piece moves, pawn moves with file indicator.
        Some(next) => match locator_from_char(board, target, from, next) {
            Some(sq) => hold = Some(sq),
            None => {
                let mut next = next;
                // It's probably a take. cxd4, Baxd4, R4xd8 or something.

                if next == 'x' {
                    #[cfg(test)]
                    println!("[execute/mod]: 'bout to cap");
                    // we can unwrap since a take will always have another input.
                    next = input.next().unwrap();
                }

                // It has an rank/file indicator, or is a pawn.
                if Piece::from_char(next).is_none() {
                    let next_after = input.next();
                    // Pawn move, not an indicator.
                    #[cfg(test)]
                    println!("[execute/mod]: {:?} is a descriptor", next);
                    from.overwrite(input::position_indicator(next));
                    if let Some(n) = next_after {
                        next = n;
                    }
                }

                // Check for second indicator.
                if Piece::from_char(next).is_none() {
                    let next_after = input.next();
                    #[cfg(test)]
                    println!("[execute/mod]: {:?} is 2nd descriptor", next);
                    from.overwrite(input::position_indicator(next));
                    if let Some(n) = next_after {
                        next = n;
                    }
                }

                match locator_from_char(board, target, from, next) {
                    Some(sq) => hold = Some(sq),
                    None => {
                        // if let Some(f) = input::letter_index(next) {
                        //     from.letter = Some(f)
                        // };
                        hold = pawn::locate(&board, target, from, board.turn_order, Piece::Pawn);
                    }
                }
            }
        },
        None => {
            // pawn move
            if let Some(sq) = pawn::locate(&board, target, from, board.turn_order, Piece::Pawn) {
                hold = Some(sq)
            }
        }
    }

    match hold {
        Some(from) => match board.checked_translate(from, target) {
            Ok(()) => (),
            Err(e) => return Err(e),
        },
        None => return Err(Error::new(ErrorKind::Other, "nothing to move")),
    };

    let did_i_check_them = board.in_check(board.turn_order);

    // If I expected to have checked them, but the results differ from my expectation,
    // These are´for testing purposes.
    match king_status {
        KingStatus::Safe => {
            // Check for stalemate
            if did_i_check_them {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Your move went through, but you checked when not expected to",
                ));
            } else if board.stalemate(board.turn_order) {
                return Ok(EResult::Stalemate);
            }
        }
        KingStatus::Mate => {
            if !did_i_check_them || !board.check_mate(board.turn_order) {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Your move went through, but your move did not checkmate",
                ));
            }
        }
        _ => {
            if !did_i_check_them {
                // Print some debug moves.

                return Err(Error::new(
                    ErrorKind::Other,
                    "Your move went through, but your move did not check",
                ));
            }
        }
    }

    Ok(EResult::Ok)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Entity;
    #[test]
    fn test_macro() {
        let mut board = Board::new();
        run!(board, "e4", "e5");
    }
    #[test]
    fn test_in_check() {
        let mut board = Board::new();
        run!(board, "e4", "f5", "Qh5+");
        assert_eq!(
            board.in_check(Team::Black),
            true,
            "Black should be in check after Qh5+"
        );
        run!(board, "g6", "Qxg6+", "hxg6", "f4", "Rh3", "d4", "Re3+");
        assert_eq!(
            board.in_check(Team::White),
            true,
            "White should be in check after Re3+"
        );
    }
    #[test]
    fn test_in_check2() {
        let mut board = Board::new();
        run!(board, "d4", "Nf6", "Nc3", "d5", "Nf3", "e6", "e3", "c5", "Bb5+");
        assert_eq!(
            board.in_check(Team::Black),
            true,
            "Black should be in check after Bb5+"
        );
        match execute(&mut board, "a6".chars()) {
            Ok(_r) => panic!("Cannot evade check"),
            Err(_e) => (),
        }
    }
    #[test]
    fn test_in_fake_mate() {
        let mut board = Board::new();
        match execute(&mut board, "e4#".chars()) {
            Ok(_) => panic!(format!("e4 is not mate you dummy")),
            Err(_) => (),
        }
    }
    #[test]
    fn test_in_check3() {
        let mut board = Board::new();
        run!(
            board, "d4", "e6", "c4", "Nf6", "Nf3", "Ne4", "Bf4", "Nxf2", "Kxf2", "d5", "e3", "e5",
            "Bxe5", "f6", "Bg3", "Bd6", "Nc3", "O-O", "Nb5", "a6", "Nxd6", "cxd6", "cxd5", "Rf7",
            "Rc1", "Qe7", "Rxc8+", "Rf8", "Rxf8+", "Kxf8", "Bd3", "Nd7", "Qb3", "Re8", "Re1", "b5",
            "Qb4", "Kg8", "Qxd6", "Qd8", "Qxa6", "b4", "Bb5", "h6", "Bxd7", "Qxd7", "Rc1", "Qe7",
            "Qe6+"
        );
        let res = board.in_check(Team::Black);
        assert_eq!(res, true, "Black should be in check after Qe6+");
        match execute(&mut board, "Kh8".chars()) {
            Ok(_r) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn test_inital_pawns() {
        let mut board = Board::new();
        for team in [Team::White, Team::Black].iter() {
            let [digit, mul]: [isize; 2] = match team {
                Team::White => [1, 1],
                Team::Black => [6, -1],
            };
            board.turn_order = *team;
            for letter in 0..8 {
                let sq = Sq::new(digit as usize, letter);
                let translations =
                    pawn::get_translations(&board, sq, board.turn_order, Some(Piece::Pawn));
                let msg = format!(
                    "{:?} pawn at {} has incorrect amount of translations ({:?})",
                    team, sq, translations
                );
                assert_eq!(translations.len(), 2, "{}", msg);
                for x in 1..=2 {
                    let new_digit = (sq.digit as isize + (mul * x as isize)) as usize;
                    let sq2 = Sq::new(new_digit, sq.letter);
                    let msg = format!(
                        "{:?} pawn at {} should translate to {} (translations: {:?})",
                        team, sq, sq2, translations
                    );
                    assert!(translations.contains(&sq2), "{}", msg);
                }
            }
        }
    }

    #[test]
    fn test_white_pawn_blocked() {
        for team in [Team::White, Team::Black].iter() {
            for t in [Team::White, Team::Black].iter() {
                let mut board = Board::new();
                board.clear();
                board.board[0][0] = Some(Entity::new(Piece::Pawn, Team::White));
                board.board[1][0] = Some(Entity::new(Piece::Pawn, *t));
                let sq = Sq::new(0, 0);
                let translations =
                    pawn::get_translations(&board, sq, board.turn_order, Some(Piece::Pawn));
                let msg = format!("{:?} pawn at {} should be blocked", team, sq);
                assert_eq!(translations.len(), 0, "{}", msg);
            }
        }
    }

    #[test]
    fn test_pawn_can_capture() {
        let mut board = Board::new();
        board.clear();
        board.board[0][0] = Some(Entity::new(Piece::Pawn, Team::White));
        let black_sq = Sq::new(1, 1);
        board.place(black_sq, Entity::new(Piece::Pawn, Team::Black));
        let sq = Sq::new(0, 0);
        let team = Team::White;
        let translations = pawn::get_translations(&board, sq, board.turn_order, Some(Piece::Pawn));
        let msg = format!(
            "{:?} pawn at {} should be able to capture Black pawn at {} ({:?})",
            team, sq, black_sq, translations
        );
        assert!(translations.contains(&Sq::new(1, 1)), msg);
    }

    #[test]
    fn test_bishop_pretending_to_be_a_pawn() {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::new(0, 0);
        board.place(sq, Entity::new(Piece::Bishop, Team::White));
        let translations = pawn::get_translations(&board, sq, board.turn_order, Some(Piece::Pawn));
        assert_eq!(
            translations.len(),
            0,
            "Bishop shouldn't be treated as a Pawn"
        );
    }
}
