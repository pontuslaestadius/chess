use crate::display;
use crate::execute;
use crate::input;
use crate::input::pgn::Turn;
use crate::{Board, Team};
use std::fs;
use std::io;
use std::str::FromStr;
use std::{thread, time};

mod opponent;
use crate::computer;
use opponent::Opponent;

fn opponent_to_action(opponent: Opponent) -> &'static dyn Fn(&mut Board) -> io::Result<()> {
    match opponent {
        Opponent::Player => &computer::player::action,
        Opponent::Computer => &computer::dummy::action,
    }
}

pub fn manual_game_loop(white: String, black: String) -> io::Result<()> {
    let white_action = opponent_to_action(Opponent::from_str(&white).unwrap());
    let black_action = opponent_to_action(Opponent::from_str(&black).unwrap());

    let short_dur = time::Duration::from_millis(150);

    let mut board = Board::new();
    display::present(&board);
    loop {
        let result = match board.turn_order {
            Team::White => white_action(&mut board),
            Team::Black => black_action(&mut board),
        };
        match result {
            Ok(_) => (),
            Err(e) => {
                display::print_error(e);
                continue;
            }
        };
        display::present(&board);

        thread::sleep(short_dur);
    }
}

pub fn automatic_game_loop(pgn: String) -> io::Result<()> {
    let short_dur = time::Duration::from_millis(20);
    // let long_dur = time::Duration::from_millis(2000);
    let data = fs::read_to_string(pgn).unwrap();
    let lines = data.lines();
    let failed_commands: Vec<String> = Vec::new();
    let mut failed_count: usize = 0;
    let mut successful_count: usize = 0;
    let mut game_count: usize = 0;
    let mut total_count: usize = 0;
    let debug = false;

    for line in lines {
        #[cfg(debug)]
        meta.push(line);
        // filter meta data.
        if line.starts_with('[') {
            // #[cfg(debug)]
            if debug && line.starts_with("[FICSGames") {
                // meta = Vec::new();
                thread::sleep(short_dur);
                println!("{}. {}", game_count, line);
            }
            continue;
        }
        // seperator
        if line == "" {
            continue;
        }

        // For testing purposes.
        let mut board = Board::new();
        let turns: Vec<Turn> = input::pgn::parse(line);
        let len = turns.len();
        game_count += 1;
        total_count += len;
        // println!("--- Game --- {} moves", game.len());

        for (n, turn) in turns.iter().enumerate() {
            if debug {
                println!("{}. {:?}", n + 1, turn);
                thread::sleep(short_dur);
            }

            for action in [turn.white.as_ref(), turn.black.as_ref()].iter() {
                match action {
                    Some(string) => match execute::execute(&mut board, string.chars()) {
                        Ok(eresult) => match eresult {
                            execute::EResult::Ok => successful_count += 1,
                            execute::EResult::Stalemate | execute::EResult::Checkmate => {
                                if n + 1 != len {
                                    failed_count += 1;
                                    // if !failed_commands.contains(&string.clone()) {
                                    //     failed_commands.push(string.clone());
                                    // }
                                }
                            }
                        },
                        Err(_) => {
                            failed_count += 1;
                            // #[cfg(debug)]
                            // {
                            //     // for m in meta.clone().iter() {
                            //     //     println!("{}", m);
                            //     // }
                            //     display::present(&board);
                            //     println!("{}. {} failed: {}", n + 1, turn.white, e);
                            //     println!("{:?}", turns);
                            // }
                            // if !failed_commands.contains(&string) {
                            //     failed_commands.push(string.clone());
                            // }
                            #[cfg(debug)]
                            thread::sleep(long_dur);
                            break;
                        }
                    },
                    None => continue,
                }
            }
        }
    }

    println!(
        "{} games, {} moves (+{}/-{})",
        game_count, total_count, successful_count, failed_count
    );

    if !failed_commands.is_empty() {
        println!("unique list of failed commands encountered:");
        println!("{:?}", failed_commands);
    }
    Ok(())
}
