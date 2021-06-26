use crate::display;
use crate::execute;
use crate::input;
use crate::input::pgn::Turn;
use crate::{Board, Team};
use std::convert::TryInto;
use std::fs;
use std::io;
use std::str::FromStr;
use std::{thread, time};

mod opponent;
use crate::Args;
use opponent::Opponent;

pub fn manual_game_loop(args: Args) -> io::Result<()> {
    let white = Opponent::from_str(&args.white).unwrap().init();
    let black = Opponent::from_str(&args.black).unwrap().init();
    let short_dur = time::Duration::from_millis(120);

    let mut board = match args.fen {
        Some(fen) => fen.try_into()?,
        None => Board::new(),
    };

    display::present(&board);
    loop {
        thread::sleep(short_dur);
        let result = match board.turn_order {
            Team::White => white.action(&mut board),
            Team::Black => black.action(&mut board),
        };
        match result {
            Ok(_) => (),
            Err(e) => {
                display::print_error(e);
                continue;
            }
        };
        thread::sleep(short_dur);
        display::present(&board);
        thread::sleep(short_dur);
    }
}

pub fn automatic_game_loop(pgn: String) -> io::Result<()> {
    let data = fs::read_to_string(pgn)?;
    let mut pgns = Vec::new();

    for (n, line) in data.lines().enumerate() {
        if line != "" && !line.starts_with('[') {
            pgns.push(input::pgn::parse(line));
        }
    }
    println!("{} game(s) parsed", pgns.len());
    let mut failed_count: usize = 0;
    let mut successful_count: usize = 0;

    for pgn in pgns {
        let mut board = Board::new();
        let len = pgn.len();
        for (n, turn) in pgn.iter().enumerate() {
            for action in [turn.white.as_ref(), turn.black.as_ref()].iter() {
                match action {
                    Some(string) => match execute::execute(&mut board, string.chars()) {
                        Ok(eresult) => match eresult {
                            execute::EResult::Ok => successful_count += 1,
                            execute::EResult::Stalemate | execute::EResult::Checkmate => {
                                if n + 1 != len {
                                    failed_count += 1;
                                }
                            }
                        },
                        Err(_) => {
                            failed_count += 1;
                            break;
                        }
                    },
                    None => continue,
                }
            }
        }
    }

    println!("+{}/-{}", successful_count, failed_count);

    Ok(())
}

pub fn legacy_automatic_game_loop(pgn: String) -> io::Result<()> {
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

    for (n, line) in lines.enumerate() {
        // print!("{}[2J", 27 as char);
        #[cfg(debug)]
        meta.push(line);
        // filter meta data.
        if line.starts_with('[') {
            // #[cfg(debug)]
            if debug && line.starts_with("[FICSGames") {
                // meta = Vec::new();
                // thread::sleep(short_dur);
                println!("{}. {}", n + 1, line);
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
    // print!("{}[2J", 27 as char);

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
