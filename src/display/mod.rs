extern crate colored;
use crate::board::Board;
use crate::colored::Colorize;
use crate::place::sq::Sq;
use crate::Team;
use std::io::Error;

pub fn present(board: &Board) {
    print!("{}[2J", 27 as char);
    for (x, row) in board.board.iter().rev().enumerate() {
        for (y, column) in row.iter().enumerate() {
            if y == 0 {
                print!(" {} ", row.len() - x);
            }
            let label: colored::ColoredString = match column {
                Some(ent) => color_team(&ent.team, ent.kind.into()).to_string().black(),

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
            let len = board.history.len(Team::White);
            if len > 0 && len >= x {
                let [w, b] = board.history.tuple(x - 1);
                let w = match w {
                    Some(x) => x.label.clone().unwrap(),
                    None => String::new(),
                };
                let b = match b {
                    Some(x) => x.label.clone().unwrap(),
                    None => String::new(),
                };
                print!("   {}. {} {}", x, pad_string(w, 7), pad_string(b, 7));
            }
        }
        if x == 7 {
            print!("  Eval: {}   ", board.evaluation());
        }
        println!();
    }
    println!("    A  B  C  D  E  F  G  H ");
}

fn print_player_to_move(team: &Team) {
    print!(
        "{}",
        color_team_background(
            team,
            &color_team(team, &format!("    {} to Move    ", team))
        )
    );
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

#[allow(dead_code)]
fn color_team_rev(team: &Team, label: &str) -> colored::ColoredString {
    match team {
        Team::White => label.black(),
        Team::Black => label.white(),
    }
}

fn color_team_background(team: &Team, label: &str) -> colored::ColoredString {
    match team {
        Team::White => label.on_white(),
        Team::Black => label.on_black(),
    }
}

#[allow(dead_code)]
fn color_team_background_rev(team: &Team, label: &str) -> colored::ColoredString {
    match team {
        Team::White => label.on_black(),
        Team::Black => label.on_white(),
    }
}

fn pad_string(mut string: String, len: usize) -> String {
    while string.len() < len {
        string.push_str(" ");
    }
    string
}
