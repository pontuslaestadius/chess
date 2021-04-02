use crate::Board;
use std::io;

pub fn action(board: &mut Board) -> io::Result<()> {
    let moves = board.find_by_team(board.turn_order);
    let sq_entity = moves.last().unwrap();
    let sq = sq_entity.sq;
    let piece = sq_entity.entity.kind;
    let translations = piece.get_translations()(&board, sq, board.turn_order, Some(piece));
    if !translations.is_empty() {
        let last = translations.last().unwrap();
        match board.checked_translate(sq, *last) {
            Ok(()) => (),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}
