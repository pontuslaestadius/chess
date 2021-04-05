pub mod player;

use crate::execute;
use crate::{Board, Team};
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Opening {
    pub name: String,
    pub reply: String,
    #[allow(clippy::box_vec)]
    pub children: Option<Box<Vec<Opening>>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CDCOpening {
    /// Unique identifier for the Opening
    pub id: usize,
    /// Opening Tag
    pub c: String,
    /// Name
    pub n: String,
    /// FEN
    pub f: String,
    /// Name with different format?
    pub u: String,
    /// List of moves included in opening
    pub m: String,
}

pub fn read_openings_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Opening>> {
    let data = fs::read_to_string(path)?;
    let res: Vec<Opening> = serde_json::from_str(&data)?;
    Ok(res)
}

pub trait Playable {
    // This is called when you are expected to reply to a turn.
    fn action(&self, board: &mut Board) -> io::Result<()>;
}

pub struct Computer {
    openings: Vec<Opening>,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            openings: read_openings_file("./data/openings.json")
                .expect("Failed to read openings file"),
        }
    }
}

impl Playable for Computer {
    fn action(&self, mut board: &mut Board) -> io::Result<()> {
        match self.next_book_move(board) {
            Some(string) => match execute::execute(&mut board, string.chars()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            None => {
                #[cfg(test)]
                println!("[computer/mod]: No book move available, thinking on my own!");
                let moves = board.find_by_team(board.turn_order);
                let sq_entity = moves.last().unwrap();
                let sq = sq_entity.sq;
                let piece = sq_entity.entity.kind;
                let translations =
                    piece.get_translations()(&board, sq, board.turn_order, Some(piece));
                if !translations.is_empty() {
                    let last = translations.last().unwrap();
                    match board.checked_translate(sq, *last) {
                        Ok(()) => (),
                        Err(e) => return Err(e),
                    };
                }
                Ok(())
            }
        }
    }
}

impl Computer {
    fn next_book_move(&self, board: &Board) -> Option<String> {
        // One variation of an opening can have several children, like a graph, or a tree structure.
        // TODO: make it able to return several openings.
        #[cfg(test)]
        println!("[computer/mod]: Checking for Known opening(s)");

        let mut suitable_openings: Vec<Opening> = Vec::new();

        // initially add all openings as suitable.
        for opening in self.openings.iter() {
            suitable_openings.push(opening.clone());
        }

        // Edge case if no history exist.

        let mut ptn = 0;
        let mut cur_team = Team::White;

        // Move pointer.
        while !suitable_openings.is_empty() {
            // Check if ptn from any opening matches History.
            #[cfg(test)]
            println!(
                "[computer/mod]: Getting History for {} on idx {}",
                cur_team, ptn
            );

            match board.history.get(cur_team, ptn) {
                Some(mov) => {
                    match &mov.label {
                        Some(label) => {
                            #[cfg(test)]
                            println!("[computer/mod]: Checking if previous move was a book move");
                            // We don't want to append to the same vec we are removing items from,
                            // This will cause indexing issues, so we make a temporary vec.
                            let mut tmp_suitable_openings = Vec::new();

                            // Remove all items that do not match the given book move.
                            for opening in suitable_openings.clone().iter() {
                                if opening.reply == *label {
                                    #[cfg(test)]
                                    println!("[computer/mod]: {} was a book move", label);
                                    if let Some(mut boxed_openings) = opening.children.clone() {
                                        tmp_suitable_openings.append(&mut (*boxed_openings));
                                        #[cfg(test)]
                                        println!("[computer/mod]: {:?}", tmp_suitable_openings);
                                    }
                                }
                            }
                            suitable_openings = tmp_suitable_openings;

                            // Since the ptn is the same for White and Black, we only iterate every other.
                            if cur_team == Team::Black {
                                ptn += 1;
                            }
                            cur_team = cur_team.not();
                        }
                        None => return None,
                    }
                }
                None => {
                    #[cfg(test)]
                    println!("[computer/mod]: Up to date, look for next move to perform.");
                    #[cfg(test)]
                    println!(
                        "[computer/mod]: Selected reply: {}",
                        suitable_openings[0].reply
                    );

                    let selected_opening = suitable_openings.choose(&mut rand::thread_rng())?;

                    return Some(selected_opening.reply.clone());
                }
            }
        }

        None
    }
}
