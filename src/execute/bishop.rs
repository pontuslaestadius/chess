use crate::{Board, OptSq, Piece, Sq, SqStatus, Team};

const PATTERN: [[isize; 2]; 4] = [[1, 1], [-1, 1], [-1, -1], [1, -1]];

pub fn get_translations(board: &Board, from: Sq, team: Team, piece: Option<Piece>) -> Vec<Sq> {
    let mut vec = Vec::new();

    let team = match board.find(from, Some(&team), piece) {
        Some(entity) => entity.team,
        None => return vec,
    };

    let mut lambda = |rank: isize, file: isize| -> bool {
        let target = match from.mutate(rank, file) {
            Some(sq) => sq,
            None => return false,
        };
        match board.find(target, None, None) {
            None => {
                vec.push(target);
                true
            }
            Some(entity) => {
                if entity.team != team {
                    vec.push(target);
                }
                false
            }
        }
    };

    for [a, b] in &PATTERN {
        for c in 1..8 {
            if !lambda(c * a, c * b) {
                break;
            }
        }
    }

    vec
}

pub fn locate(board: &Board, to: Sq, from: OptSq, team: Team, piece: Piece) -> Option<Sq> {
    let lambda = |rank: isize, file: isize| -> SqStatus {
        if let Some(target) = to.mutate(rank, file) {
            let target = target.union(from);
            if let Some(sq) = board.legal_target(target, to, team, piece) {
                return SqStatus::Some(sq);
            }
        }
        SqStatus::None
    };

    for [a, b] in &PATTERN {
        for c in 1..8 {
            match lambda(c * a, c * b) {
                SqStatus::Some(sq) => return Some(sq),
                SqStatus::Blocked => break,
                SqStatus::None => (),
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute::*;
    use crate::run;
    use crate::Entity;
    #[test]
    fn test_find_bishop() {
        let mut board = Board::new();
        board.clear();
        let sq = Sq::new(4, 4);
        for team in [Team::White, Team::Black].iter() {
            board.place(sq, Entity::new(Piece::Bishop, *team));
            board.turn_order = *team;
            for x in 0..8 {
                if x == 4 {
                    continue;
                }
                let to = Sq::new(x, x);
                assert_eq!(
                    locate(&board, to, OptSq::new(), board.turn_order, Piece::Bishop),
                    Some(sq),
                    "Bishop at {} should be able to move to {}",
                    sq,
                    to
                );
            }
        }
    }
    #[test]
    fn test_no_find_bishop() {
        let mut board = Board::new();
        board.clear();
        board.place(Sq::new(4, 4), Entity::new(Piece::Bishop, Team::White));
        let res = locate(
            &board,
            Sq::new(1, 0),
            OptSq::new(),
            board.turn_order,
            Piece::Bishop,
        );
        assert_eq!(res, None);
    }
    #[test]
    fn test_bishop_move_zero() {
        let mut board = Board::new();
        board.clear();
        board.place(Sq::new(4, 4), Entity::new(Piece::Bishop, Team::White));
        assert_eq!(
            locate(
                &board,
                Sq::new(4, 4),
                OptSq::new(),
                board.turn_order,
                Piece::Bishop
            ),
            None
        );
    }
    #[test]
    fn test_bishop_bd2_bd7() {
        let mut board = Board::new();
        run!(board, "d4", "d5", "Bd2", "Bd7");
    }
    #[test]
    fn test_bishop_bb5_bb4() {
        let mut board = Board::new();
        run!(board, "e4", "e5", "Bb5", "Bb4");
    }
}
