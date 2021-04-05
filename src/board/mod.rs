use crate::place::entity::{Entity, SqEntity};
use crate::place::sq::Sq;
use crate::{History, Piece, Team};
use std::io::{Error, ErrorKind, Result};

pub mod history;
pub mod piece;
pub mod team;

pub enum SqStatus {
    None,
    Blocked,
    Some(Sq),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KingStatus {
    Safe,
    Check,
    Mate,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub turn_order: Team,
    pub history: History,
    pub board: [[Option<Entity>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut board = [[None; 8]; 8];
        let b = Team::Black;
        let w = Team::White;

        board[7][0] = Some(Entity::new(Piece::Rook, b));
        board[7][1] = Some(Entity::new(Piece::Knight, b));
        board[7][2] = Some(Entity::new(Piece::Bishop, b));
        board[7][3] = Some(Entity::new(Piece::Queen, b));
        board[7][4] = Some(Entity::new(Piece::King, b));
        board[7][5] = Some(Entity::new(Piece::Bishop, b));
        board[7][6] = Some(Entity::new(Piece::Knight, b));
        board[7][7] = Some(Entity::new(Piece::Rook, b));
        board[0][0] = Some(Entity::new(Piece::Rook, w));
        board[0][1] = Some(Entity::new(Piece::Knight, w));
        board[0][2] = Some(Entity::new(Piece::Bishop, w));
        board[0][3] = Some(Entity::new(Piece::Queen, w));
        board[0][4] = Some(Entity::new(Piece::King, w));
        board[0][5] = Some(Entity::new(Piece::Bishop, w));
        board[0][6] = Some(Entity::new(Piece::Knight, w));
        board[0][7] = Some(Entity::new(Piece::Rook, w));

        for n in 0..8 {
            board[1][n] = Some(Entity::new(Piece::Pawn, w));
            board[6][n] = Some(Entity::new(Piece::Pawn, b));
        }
        Board {
            turn_order: Team::White,
            history: History::new(),
            board,
        }
    }
    pub fn place(&mut self, sq: Sq, entity: Entity) {
        self.board[sq.digit][sq.letter] = Some(entity);
    }
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.board = [[None; 8]; 8];
    }
    pub fn not_turn(&self) -> Team {
        self.turn_order.not()
    }
    #[allow(dead_code)]
    pub fn turn(&self) -> Team {
        self.turn_order
    }
    pub fn checked_translate(&mut self, from: Sq, to: Sq) -> Result<()> {
        let mut other = self.clone();
        let turn = other.turn_order;

        let label = other.translate(from, to)?;
        if other.in_check(turn) {
            let msg = format!("illegal move for {}, places yourself in check.", turn);
            return Err(Error::new(ErrorKind::Other, msg));
        }

        match self.find(from, Some(&turn), None) {
            None => (), // Panic?
            Some(entity) => self.history.push(turn, entity.kind, from, to, Some(label)),
        }

        self.board = other.board;
        self.turn_order = other.turn_order;

        // Move was alright, register it.
        // FIXME: should probably be a move, self = other.
        // self.translate(from, to)?;

        #[cfg(test)]
        println!("[board/mod]: {} -> {}", from, to);

        Ok(())
    }
    pub fn can_translate(&self, from: Sq, to: Sq) -> bool {
        let mut other = self.clone();
        let turn = other.turn_order;
        let res = other.translate(from, to);
        if res.is_err() {
            #[cfg(test)]
            println!("translate failed because {:?}", res);
            return false;
        };
        !other.in_check(turn)
    }

    pub fn in_check(&self, team: Team) -> bool {
        #[cfg(test)]
        println!("checking if {:?} is in check.", team);

        let opposite_team = match team {
            Team::White => Team::Black,
            Team::Black => Team::White,
        };

        let entities = self.find_by_team(opposite_team);
        for sq_entity in entities {
            let sq = sq_entity.sq;
            let piece = sq_entity.entity.kind;
            let translations = piece.get_translations()(&self, sq, opposite_team, Some(piece));

            // #[cfg(test)]
            // println!("{} {:?}: {:?}", sq, piece, translations);
            for t in translations {
                // #[cfg(test)]
                // println!("Searching for {}'s King at {} ", team, t);
                if let Some(_ent) = self.find(t, Some(&team), Some(Piece::King)) {
                    #[cfg(test)]
                    println!(
                        "{:?} is covered by {:?} at {}, {:?} is in check",
                        t, piece, sq, team
                    );
                    return true;
                }
            }
        }
        #[cfg(test)]
        println!("{:?} is not in check.", team);
        false
    }

    // Assumes Team is already in check.
    pub fn check_mate(&self, team: Team) -> bool {
        // Overwrite existing turn order to mimick if the player could move.
        let mut other = self.clone();
        other.turn_order = team;

        self.find_by_team_closure(team, &|sq_entity: SqEntity| -> bool {
            let sq = sq_entity.sq;
            let piece = sq_entity.entity.kind;
            let translations = piece.get_translations()(&other, sq, team, Some(piece));
            for t in translations {
                if other.can_translate(sq, t) {
                    return false;
                }
            }
            true
        })
    }

    pub fn evaluation(&self) -> isize {
        // Get piece values of all entities for each team.
        let mut result: isize = 0;
        for rank in 0..8 {
            for file in 0..8 {
                let sq = Sq::new(rank, file);
                if let Some(entity) = self.find(sq, None, None) {
                    match entity.team {
                        Team::White => result += entity.kind.value() as isize,
                        Team::Black => result -= entity.kind.value() as isize,
                    }
                }
            }
        }
        result
    }

    pub fn stalemate(&self, team: Team) -> bool {
        // Overwrite existing turn order to mimick if the player could move.
        let mut other = self.clone();
        other.turn_order = team;

        self.find_by_team_closure(team, &|sq_entity: SqEntity| -> bool {
            let sq = sq_entity.sq;
            let piece = sq_entity.entity.kind;
            let translations = piece.get_translations()(&other, sq, team, Some(piece));
            for t in translations {
                if other.can_translate(sq, t) {
                    #[cfg(test)]
                    println!(
                        "[board/mod]: {} not in stalemate because {:?} can move to {:?}",
                        team, piece, t
                    );
                    return false;
                }
            }
            true
        })
    }
    pub fn translate(&mut self, from: Sq, to: Sq) -> Result<String> {
        let mut label: String = String::new();

        let from_entity = match self.get(from) {
            Some(ent) => {
                if ent.team != self.turn_order {
                    let msg = format!(
                        "illegal move, (piece: {}, turn: {})",
                        ent.team, self.turn_order
                    );
                    return Err(Error::new(ErrorKind::Other, msg));
                }
                ent
            }
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "illegal move, nothing at from",
                ));
            }
        };

        label.push_str(from_entity.kind.to_str());

        match self.get(to) {
            Some(to_entity) => {
                if from_entity.team == to_entity.team {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "illegal move, target is friendly",
                    ));
                }
                if to_entity.kind == Piece::King {
                    return Err(Error::new(ErrorKind::Other, "illegal move, target is King"));
                }
                label.push_str("x");
                self.board[to.digit][to.letter] = self.board[from.digit][from.letter];
                self.board[from.digit][from.letter] = None;
            }
            None => {
                // Check if en passant
                if from_entity.kind == Piece::Pawn {
                    // If it was a capture move but didn't land on an occupied square.
                    if from.letter != to.letter {
                        let diff = from.digit as isize - to.digit as isize;
                        let clean_up_sq = Sq::new((to.digit as isize + diff) as usize, to.letter);
                        match self.find(clean_up_sq, Some(&self.not_turn()), Some(Piece::Pawn)) {
                            Some(_) => {
                                label.push_str("x");
                                self.board[clean_up_sq.digit][clean_up_sq.letter] = None
                            }
                            None => {
                                let msg = format!("illegal en passant, no Pawn at {}", clean_up_sq);
                                return Err(Error::new(ErrorKind::Other, msg));
                            }
                        }
                    }
                }
                self.board[to.digit][to.letter] = self.board[from.digit][from.letter];
                self.board[from.digit][from.letter] = None;
            }
        };
        self.turn_order = match self.turn_order {
            Team::White => Team::Black,
            Team::Black => Team::White,
        };
        label.push_str(format!("{}", to).as_ref());

        Ok(label)
    }

    pub fn legal_target(&self, from: Sq, to: Sq, team: Team, piece: Piece) -> Option<Sq> {
        if let Some(_entity) = self.find(from, Some(&team), Some(piece)) {
            let translations = piece.get_translations()(&self, from, team, Some(piece));
            if translations.contains(&to) && self.can_translate(from, to) {
                return Some(from);
            }
        }
        None
    }

    pub fn get(&self, sq: Sq) -> Option<Entity> {
        if sq.digit > 7 || sq.letter > 7 {
            return None;
        }
        self.board[sq.digit][sq.letter]
    }
    pub fn find(&self, sq: Sq, team: Option<&Team>, piece: Option<Piece>) -> Option<Entity> {
        let mut res = false;
        if let Some(entity) = self.get(sq) {
            // #[cfg(test)]
            // println!(
            //     "Find: sq: {:?}, team: {:?}, piece: {:?} -> {:?}",
            //     sq, team, piece, entity
            // );
            if team.is_none() && piece.is_none() {
                return Some(entity);
            }
            if let Some(t) = team {
                if *t == entity.team {
                    res = true;
                } else {
                    return None;
                }
            }
            if let Some(p) = piece {
                if p == entity.kind {
                    res = true;
                } else {
                    return None;
                }
            }
        }
        match res {
            true => self.get(sq),
            false => None,
        }
    }
    pub fn find_by_team(&self, team: Team) -> Vec<SqEntity> {
        // for performance, look at black from the top, and white from the bottom.
        let [outer, inner] = match team {
            Team::White => [[0, 1, 2, 3, 4, 5, 6, 7], [0, 1, 2, 3, 4, 5, 6, 7]],
            Team::Black => [[7, 6, 5, 4, 3, 2, 1, 0], [7, 6, 5, 4, 3, 2, 1, 0]],
        };
        let mut vec = Vec::new();
        for rank in outer.iter() {
            for file in inner.clone().iter() {
                let sq = Sq::new(*rank, *file);
                if let Some(entity) = self.find(sq, Some(&team), None) {
                    vec.push(SqEntity { entity, sq });
                }
            }
        }
        vec
    }
    pub fn find_by_team_closure(&self, team: Team, closure: &dyn Fn(SqEntity) -> bool) -> bool {
        // for performance, look at black from the top, and white from the bottom.
        let [outer, inner] = match team {
            Team::White => [[0, 1, 2, 3, 4, 5, 6, 7], [0, 1, 2, 3, 4, 5, 6, 7]],
            Team::Black => [[7, 6, 5, 4, 3, 2, 1, 0], [7, 6, 5, 4, 3, 2, 1, 0]],
        };
        for rank in outer.iter() {
            for file in inner.clone().iter() {
                let sq = Sq::new(*rank, *file);
                if let Some(entity) = self.find(sq, Some(&team), None) {
                    if !closure(SqEntity { entity, sq }) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute::*;
    use crate::run;
    #[test]
    fn test_new_board() {
        Board::new();
    }
    #[test]
    fn test_evaluation() {
        let board = Board::new();
        assert_eq!(board.evaluation(), 0, "Initial evaluation is awlays 0");
    }
    #[test]
    fn test_get_rooked() {
        let board = Board::new();
        assert_eq!(
            board.get(Sq::new(0, 0)),
            Some(Entity::new(Piece::Rook, Team::White))
        );
        assert_eq!(
            board.get(Sq::new(7, 7)),
            Some(Entity::new(Piece::Rook, Team::Black))
        );
    }
    #[test]
    fn test_find_rook() {
        let board = Board::new();
        let res = board.find(Sq::new(0, 0), None, Some(Piece::Rook));
        assert!(res.is_some());
    }
    #[test]
    fn test_find_rook_invalid_team() {
        let board = Board::new();
        let res = board.find(Sq::new(0, 0), Some(&Team::Black), Some(Piece::Rook));
        assert!(res.is_none());
    }
    #[test]
    fn test_find_rook_invalid_piece() {
        let board = Board::new();
        let res = board.find(Sq::new(0, 0), Some(&Team::White), Some(Piece::Pawn));
        assert!(res.is_none());
    }
    #[test]
    fn test_find_empty() {
        let board = Board::new();
        let res = board.find(Sq::new(4, 4), Some(&Team::White), Some(Piece::King));
        assert!(res.is_none());
    }
    #[test]
    fn sq_add_1() {
        let sq = Sq::new(0, 0) + Sq::new(0, 0);
        assert_eq!(sq, Sq::new(0, 0));
    }
    #[test]
    fn sq_add_2() {
        let sq = Sq::new(3, 4) + Sq::new(1, 0);
        assert_eq!(sq, Sq::new(4, 4));
    }
    #[test]
    fn sq_add_overflow() {
        let sq = Sq::new(4, 7) + Sq::new(6, 5);
        assert_eq!(sq, Sq::new(7, 7));
    }
    #[test]
    fn sq_add_underflow() {
        let sq = Sq::new(1, 1) - Sq::new(2, 2);
        assert_eq!(sq, Sq::new(0, 0));
    }
    #[test]
    fn test_init_check_mate() {
        let board = Board::new();
        assert_eq!(board.check_mate(Team::Black), false);
        assert_eq!(board.check_mate(Team::White), false)
    }
    #[test]
    fn test_stalemate() {
        let mut board = Board::new();
        run!(
            board, "d4", "c5", "dxc5", "Nf6", "c3", "g6", "b4", "Bg7", "Bd2", "O-O", "Qc1", "Nc6",
            "Nf3", "Ne4", "Bh6", "d6", "Bxg7", "Kxg7", "e3", "dxc5", "Bd3", "Qxd3", "bxc5", "Rd8",
            "Nd4", "Bg4", "f3", "Nxd4", "cxd4", "e5", "fxg4", "exd4", "exd4", "Rxd4", "a4", "Rad8",
            "Ra2", "Qd1+", "Qxd1", "Rxd1+", "Ke2", "Rxh1", "Rb2", "Nxc5", "Nc3", "Rxh2", "Kf2",
            "Rc8", "Nb5", "Nd3+", "Kg3", "Nxb2", "Kxh2", "a6", "Nd6", "Rc5", "Nxb7", "Nxa4", "Nd6",
            "Nb6", "Kg3", "Rc2", "Kf3", "a5", "Nb5", "Nc4", "g5", "a4", "Ke4", "a3", "Nxa3",
            "Nxa3", "g4", "Rc6", "Ke5", "Re6+", "Kf4", "Re1", "Kf3", "Nb5", "Kf4", "Nd4", "Kg3",
            "Re2", "Kh3", "Re5", "Kh4"
        );
        assert_eq!(
            board.stalemate(Team::White),
            false,
            "White should not be in stalemate"
        );
        assert_eq!(
            board.stalemate(Team::Black),
            false,
            "Black should not be in stalemate"
        );
        run!(board, "Re3");
        assert_eq!(
            board.stalemate(Team::White),
            true,
            "White should be in stalemate"
        );
        assert_eq!(
            board.stalemate(Team::Black),
            false,
            "Black should not be in stalemate"
        );
    }
    #[test]
    fn test_find_by_team_count() {
        let board = Board::new();
        let white_count = board.find_by_team(Team::White);
        let black_count = board.find_by_team(Team::Black);
        assert_eq!(white_count.len(), 16, "White should own 16 pieces");
        assert_eq!(black_count.len(), 16, "Black should own 16 pieces");
    }
}
