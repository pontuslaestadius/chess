use crate::place::entity::{Entity, SqEntity};
use crate::place::sq::Sq;
use crate::SIZE;
use crate::{History, Piece, Team};
use state::*;
use std::io::{Error, ErrorKind, Result};

use std::convert::TryFrom;

pub mod castling;
pub mod history;
pub mod king_status;
pub mod piece;
pub mod state;
pub mod team;

pub enum SqStatus {
    None,
    Blocked,
    Some(Sq),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub state: BoardState,
    pub turn_order: Team,
    pub history: History,
    pub board: [[Option<Entity>; SIZE]; SIZE],
    pub en_passant_target_square: Option<Sq>,
    pub castling: castling::Castling,
    pub fullmove: usize,
    pub halfmove: usize,
}

/// Using FEN
/// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
impl TryFrom<String> for Board {
    type Error = Error;

    fn try_from(item: String) -> Result<Self> {
        let mut board = Board::new();
        board.board = [[None; SIZE]; SIZE];
        let item: Vec<&str> = item.split_whitespace().into_iter().collect();

        let expected_len = 6;
        if item.len() != expected_len {
            let msg = format!(
                "Wrong number of space seperated arguments to decode FEN, expected {}, got {}",
                expected_len,
                item.len()
            );
            return Err(Error::new(ErrorKind::Other, msg));
        }

        // Piece placement.
        let ranks = item[0].split('/');

        for (i, rank) in ranks.enumerate() {
            let mut file: usize = 0;
            println!("[board/mod]: FEN rank: {}", rank);
            for ch in rank.chars() {
                match ch {
                    'a'..='z' | 'A'..='Z' => {
                        // Creat entity and populate on board.
                        let entity: Entity = Entity::from(ch);
                        let target: Sq = Sq::new(7 - i, file);
                        board.place(target, entity);
                        println!("[board/mod]: FEN Wants to place {:?} at {}", entity, target);
                        file += 1;
                    }
                    '1'..='8' => match ch.to_digit(10) {
                        Some(dig) => {
                            // Skip 'dig' files.
                            file += dig as usize;
                        }
                        None => panic!("invalid FEN indicator {}", ch),
                    },
                    _ => panic!("invalid FEN indicator {}", ch),
                }
            }
        }
        // Active color.
        board.turn_order = item[1].into();

        // Castling availability.
        board.castling = item[2].into();

        // En passant target square.
        board.en_passant_target_square = match Sq::notation(item[3]) {
            Ok(sq) => Some(sq),
            Err(_) => None,
        };

        // Halfmove clock.
        // item[4]

        // Fullmove number.
        // item[5]

        Ok(board)
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = [[None; SIZE]; SIZE];
        let b = Team::Black;
        let w = Team::White;

        for (team, rank) in &[(Team::White, 0usize), (Team::Black, (SIZE - 1) as usize)] {
            board[*rank][0] = Some(Entity::new(Piece::Rook, *team));
            board[*rank][1] = Some(Entity::new(Piece::Knight, *team));
            board[*rank][2] = Some(Entity::new(Piece::Bishop, *team));
            board[*rank][3] = Some(Entity::new(Piece::Queen, *team));
            board[*rank][4] = Some(Entity::new(Piece::King, *team));
            board[*rank][5] = Some(Entity::new(Piece::Bishop, *team));
            board[*rank][6] = Some(Entity::new(Piece::Knight, *team));
            board[*rank][7] = Some(Entity::new(Piece::Rook, *team));
        }

        for n in 0..SIZE {
            board[1][n] = Some(Entity::new(Piece::Pawn, w));
            board[6][n] = Some(Entity::new(Piece::Pawn, b));
        }
        Board {
            state: BoardState::new(),
            halfmove: 0,
            fullmove: 1,
            en_passant_target_square: None,
            castling: castling::Castling::new(),
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
        self.board = [[None; SIZE]; SIZE];
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

        // Move was alright, register it.
        // FIXME: should probably be a move, self = other.
        self.board = other.board;
        self.turn_order = other.turn();
        self.castling = other.castling;
        self.en_passant_target_square = other.en_passant_target_square;
        self.halfmove = other.halfmove;

        #[cfg(test)]
        println!("[board/mod]: {} -> {}", from, to);

        Ok(())
    }
    pub fn can_translate(&self, from: Sq, to: Sq) -> bool {
        let mut other = self.clone();
        let turn = other.turn();
        let res = other.translate(from, to);
        if res.is_err() {
            #[cfg(test)]
            println!("translate failed because {:?}", res);
            return false;
        };
        !other.in_check(turn)
    }

    pub fn in_check(&self, team: Team) -> bool {
        let entities = self.find_by_team(team.not());
        for sq_entity in entities {
            let sq = sq_entity.sq;
            let piece = sq_entity.entity.kind;
            let translations = piece.get_translations()(&self, sq, team.not(), piece);
            for t in translations {
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
            let translations = piece.get_translations()(&other, sq, team, piece);
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
        for rank in 0..SIZE {
            for file in 0..SIZE {
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
    /// Generates a FEN for the given board position.
    /// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    pub fn fen(&self) -> String {
        let mut res = String::new();

        for rank in (0..SIZE).rev() {
            let mut cur_empty_square: u8 = 0;
            for file in 0..SIZE {
                let sq = Sq::new(rank, file);
                if let Some(entity) = self.find(sq, None, None) {
                    if cur_empty_square > 0 {
                        res.push_str(&format!("{}", cur_empty_square));
                        cur_empty_square = 0;
                    }
                    let team_label: &str = entity.kind.into();
                    match entity.team {
                        Team::White => res.push_str(team_label),
                        Team::Black => res.push_str(&team_label.to_lowercase()),
                    };
                } else {
                    cur_empty_square += 1;
                }
            }
            if cur_empty_square > 0 {
                res.push_str(&format!("{}", cur_empty_square));
            }
            // Last row does not need a delimitor.
            if rank != 0 {
                res.push('/');
            }
        }
        res.push_str(&format!(
            " {} {} ",
            self.turn().abrev().to_lowercase(),
            self.castling.fen()
        ));

        match self.en_passant_target_square {
            None => res.push('-'),
            Some(sq) => res.push_str(&format!("{}", sq)),
        }

        res.push_str(&format!(" {} {}", self.halfmove, self.fullmove));
        res
    }

    pub fn stalemate(&self, team: Team) -> bool {
        // Overwrite existing turn order to mimick if the player could move.
        let mut other = self.clone();
        other.turn_order = team;

        self.find_by_team_closure(team, &|sq_entity: SqEntity| -> bool {
            let sq = sq_entity.sq;
            let piece = sq_entity.entity.kind;
            let translations = piece.get_translations()(&other, sq, team, piece);
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
                if ent.team != self.turn() {
                    let msg = format!("illegal move, (piece: {}, turn: {})", ent.team, self.turn());
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
                if from_entity.kind == Piece::Pawn {
                    label.push(from.get_file_char());
                }
                label.push_str("x");
                self.halfmove = 0;
                self.board[to.digit][to.letter] = self.board[from.digit][from.letter];
                self.board[from.digit][from.letter] = None;
            }
            None => {
                self.halfmove += 1;
                // Check if en passant
                // If it was a capture move but didn't land on an occupied square.
                #[cfg(test)]
                println!(
                    "[board/mod]: en passant square: {:?}",
                    self.en_passant_target_square
                );
                if from_entity.kind == Piece::Pawn
                    && self.en_passant_target_square.is_some()
                    && from.letter != to.letter
                {
                    let diff = from.digit as isize - to.digit as isize;
                    let clean_up_sq = Sq::new((to.digit as isize + diff) as usize, to.letter);
                    match self.find(clean_up_sq, Some(&self.not_turn()), Some(Piece::Pawn)) {
                        Some(_) => {
                            if from_entity.kind == Piece::Pawn {
                                label.push(from.get_file_char());
                            }
                            label.push_str("x");
                            self.board[clean_up_sq.digit][clean_up_sq.letter] = None
                        }
                        None => {
                            let msg = format!("illegal en passant, no Pawn at {}", clean_up_sq);
                            return Err(Error::new(ErrorKind::Other, msg));
                        }
                    }
                }
                self.board[to.digit][to.letter] = self.board[from.digit][from.letter];
                self.board[from.digit][from.letter] = None;
            }
        };
        // Increment fullmove
        if self.turn_order == Team::Black {
            self.fullmove += 1;
        }
        self.turn_order = match self.turn_order {
            Team::White => Team::Black,
            Team::Black => Team::White,
        };
        label.push_str(format!("{}", to).as_ref());

        // Only pawn moves that moved 2 squares.

        if from_entity.kind == Piece::Pawn {
            self.halfmove = 0;
            let two_or_not_to_two = isize::abs(from.digit as isize - to.digit as isize);
            #[cfg(test)]
            println!(
                "[board/mod]: from_entity: {:?} from {} to {}, two: {}",
                from_entity, from, to, two_or_not_to_two
            );

            if two_or_not_to_two == 2 {
                #[cfg(test)]
                println!("[board/mod]: en passant allowed");
                self.en_passant_target_square =
                    Some(Sq::new((to.digit + from.digit) / 2, to.letter));
            } else {
                #[cfg(test)]
                println!("[board/mod]: en passant disallowed");
                self.en_passant_target_square = None;
            }
        }

        Ok(label)
    }

    pub fn legal_target(&self, from: Sq, to: Sq, team: Team, piece: Piece) -> Option<Sq> {
        if let Some(_entity) = self.find(from, Some(&team), Some(piece)) {
            let translations = piece.get_translations()(&self, from, team, piece);
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
    use crate::display;
    use crate::execute::*;
    use crate::run;
    use std::convert::TryInto;
    #[test]
    fn test_new_board() {
        Board::new();
    }
    #[test]
    fn test_en_passant_square() {
        let mut board = Board::new();
        run!(board, "e4");
        assert_eq!(
            board.en_passant_target_square,
            Some(Sq::notation("e3").unwrap())
        );
    }
    #[test]
    fn test_halfclock() {
        let mut board = Board::new();
        assert_eq!(board.halfmove, 0);
        run!(board, "e4");
        assert_eq!(board.halfmove, 0); // Pawn move resets
        run!(board, "Nc6");
        assert_eq!(board.halfmove, 1); // Knight move increments
        run!(board, "e5");
        assert_eq!(board.halfmove, 0); // Pawn move resets
        run!(board, "f6");
        assert_eq!(board.halfmove, 0); // Pawn move resets
        run!(board, "exf6");
        assert_eq!(board.halfmove, 0); // Pawn move resets
        run!(board, "Nf6");
        assert_eq!(board.halfmove, 0); // Capture resets
    }
    #[test]
    fn test_evaluation() {
        let board = Board::new();
        assert_eq!(board.evaluation(), 0, "Initial evaluation is awlays 0");
    }
    #[test]
    fn test_initial_fen() {
        let board = Board::new();
        assert_eq!(
            board.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        );
    }
    #[test]
    fn test_initial_from_fen() {
        let board: Board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
            .to_string()
            .try_into()
            .unwrap();
        let expected_board = Board::new();
        if board != expected_board {
            display::present(&board);
            assert_eq!(
                board.castling, expected_board.castling,
                "FEN: castling differed"
            );
            assert_eq!(
                board.halfmove, expected_board.halfmove,
                "FEN: halfmove differed"
            );
            assert_eq!(
                board.en_passant_target_square, expected_board.en_passant_target_square,
                "FEN: en_passant_target_square differed"
            );
            assert_eq!(board.board, expected_board.board, "FEN: Board differed");
        }
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
