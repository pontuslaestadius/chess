extern crate colored;
use crate::board::Board;
use crate::colored::Colorize;
use crate::place::entity::Entity;
use crate::place::sq::Sq;
use crate::{Piece, Team};
use std::io::Error;

fn enum_to_label(entity: &Entity) -> String {
    let label: colored::ColoredString = match entity.kind {
        Piece::Bishop => color_team(&entity.team, "B"),
        Piece::King => color_team(&entity.team, "K"),
        Piece::Rook => color_team(&entity.team, "R"),
        Piece::Pawn => color_team(&entity.team, "P"),
        Piece::Queen => color_team(&entity.team, "Q"),
        Piece::Knight => color_team(&entity.team, "N"),
    };
    label.to_string()
}

pub fn present(board: &Board) {
    print!("{}[2J", 27 as char);
    for (x, row) in board.board.iter().rev().enumerate() {
        for (y, column) in row.iter().enumerate() {
            if y == 0 {
                print!(" {} ", row.len() - x);
            }
            let label: colored::ColoredString = match column {
                Some(piece) => enum_to_label(&piece).black(),
                None => " ".black(),
            };
            let mut label = format!(" {} ", label);
            if Sq::new(x, y).dark_square() {
                label = label.on_green().to_string();
            } else {
                label = label.on_purple().to_string();
            }
            print!("{}", label);
        }
        if x == 0 {
            print_player_to_move(&board.turn_order);
        } else {
            // let len = board.history.len();
            // if len >= x {
            //     let [w, b] = board.history.tuple(x - 1);
            //     print!("{}. {} {}", x, w.sq, b.sq);
            // }
        }
        println!();
    }
    println!("    A  B  C  D  E  F  G  H ");
}

fn print_player_to_move(team: &Team) {
    match team {
        Team::White => print!("{}", "        White to Move       ".black().on_white()),
        Team::Black => print!("{}", "        Black to Move       ".white().on_black()),
    };
}

pub fn print_error(err: Error) {
    let msg = format!("        {}        ", err);
    println!("{}", msg.black().on_red());
}

fn color_team(team: &Team, label: &str) -> colored::ColoredString {
    match team {
        Team::White => label.white(),
        Team::Black => label.black(),
    }
}
