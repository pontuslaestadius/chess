use crate::{Board, OptSq, Piece, Sq, Team};

const RANKS: [isize; 8] = [2, 2, 1, -1, -2, -2, 1, -1];
const FILES: [isize; 8] = [-1, 1, 2, 2, -1, 1, -2, -2];

pub fn get_translations(board: &Board, from: Sq, team: Team, piece: Option<Piece>) -> Vec<Sq> {
    let mut vec = Vec::new();
    if board.find(from, Some(&team), piece).is_none() {
        return vec;
    }
    for idx in 0..8 {
        let target = match from.mutate(RANKS[idx], FILES[idx]) {
            Some(sq) => sq,
            None => continue,
        };
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

pub fn locate(board: &Board, to: Sq, from: OptSq, team: Team, piece: Piece) -> Option<Sq> {
    for idx in 0..8 {
        let check_rank: isize = match from.digit {
            Some(x) => x as isize,
            None => to.digit as isize + RANKS[idx],
        };
        let check_file: isize = match from.letter {
            Some(x) => x as isize,
            None => to.letter as isize + FILES[idx],
        };
        if !Sq::valid_idx(check_rank, check_file) {
            continue;
        }

        let target = Sq::new(check_rank as usize, check_file as usize);
        if let Some(sq) = board.legal_target(target, to, team, piece) {
            return Some(sq);
        };

        // if let Some(target) = to.mutate(check_rank, check_file) {
        //     if let Some(sq) = board.legal_target(target, to, team, piece) {
        //         return Some(sq);
        //     };
        // }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute::*;
    use crate::run;
    use crate::Entity;
    use std::io::Result;
    #[test]
    fn test_find_knight() -> Result<()> {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::notation("b1")?;
        for team in [Team::White, Team::Black].iter() {
            board.place(sq, Entity::new(Piece::Knight, *team));
            board.turn_order = *team;
            let to = Sq::notation("a3")?;
            assert_eq!(
                locate(&board, to, OptSq::new(), *team, Piece::Knight),
                Some(sq),
                "Knight at {} should be able to move to {}",
                sq,
                to
            );
        }
        Ok(())
    }
    #[allow(non_snake_case)]
    #[test]
    fn test_Nf6() -> Result<()> {
        let mut board = Board::new();
        board.turn_order = Team::Black;
        let trans = get_translations(
            &board,
            Sq::notation("g8")?,
            board.turn_order,
            Some(Piece::Knight),
        );
        let target = Sq::notation("f6")?;
        assert!(trans.contains(&target), "translations: {:?}", trans);

        Ok(())
    }
    #[test]
    fn test_knight_macro_initial_moves() {
        let mut board = Board::new();
        run!(board, "Nf3", "Nf6", "Nc3", "Nc6", "Nd4", "Nxd4");
    }
    #[test]
    fn test_knight_need_file() {
        let mut board = Board::new();
        run!(board, "Nf3", "Nf6", "Nc3", "Nc6", "Nd4", "Nd5", "Ncb5", "Ndb4");
    }
    #[test]
    fn test_knight_needs_rank() {
        let mut board = Board::new();
        run!(board, "g4", "Nf6", "g5", "Nd5", "c4", "Nb4", "a3", "N4c6", "b4");
    }
    #[test]
    fn test_knight_pinned_move() {
        let mut board = Board::new();
        run!(
            board, "e4", "c6", "d4", "d5", "e5", "Bf5", "Bd3", "Bg6", "f4", "e6", "Nc3", "Nd7",
            "Nge2", "Qh4+", "Ng3", "Ne7", "Be3", "Bxd3", "Qxd3", "Ng6", "Ne2"
        );
    }
}
