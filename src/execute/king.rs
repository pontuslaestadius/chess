use crate::{Board, OptSq, Piece, Sq, Team};

const RANKS: [isize; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const FILES: [isize; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

pub fn get_translations(board: &Board, from: Sq, team: Team, piece: Option<Piece>) -> Vec<Sq> {
    let mut vec = Vec::new();
    if board.find(from, Some(&team), piece).is_none() {
        return vec;
    }

    for idx in 0..8 {
        let check_rank: isize = from.digit as isize + RANKS[idx];
        let check_file: isize = from.letter as isize + FILES[idx];
        if !Sq::valid_idx(check_rank, check_file) {
            continue;
        }

        let target = Sq::new(check_rank as usize, check_file as usize);
        match board.find(target, None, None) {
            None => vec.push(target),
            Some(entity) => {
                if entity.team != team {
                    vec.push(target);
                }
            }
        }
    }
    vec
}

// Kings cannot have a 'from' notation as there is only 1 king per side.
pub fn locate(board: &Board, to: Sq, _from: OptSq, team: Team, piece: Piece) -> Option<Sq> {
    for idx in 0..8 {
        let check_rank: isize = to.digit as isize + RANKS[idx];
        let check_file: isize = to.letter as isize + FILES[idx];
        if !Sq::valid_idx(check_rank, check_file) {
            continue;
        }

        let target = Sq::new(check_rank as usize, check_file as usize);
        if let Some(sq) = board.legal_target(target, to, team, piece) {
            return Some(sq);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::execute::*;
    use crate::run;
    #[test]
    fn test_bongcloud() {
        let mut board = Board::new();
        run!(board, "e4", "e5", "Ke2", "Ke7");
    }
}
