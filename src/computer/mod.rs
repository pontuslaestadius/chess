pub mod player;

use crate::execute;
use crate::{Board, Team};
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Opening {
    pub name: String,
    pub reply: String,
    pub children: Box<Option<Opening>>,
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
            openings: read_openings_file("./src/computer/openings.json")
                .expect("Failed to read openings file"),
        }
    }
}

impl Playable for Computer {
    fn action(&self, mut board: &mut Board) -> io::Result<()> {
        // Check if there is a book move.

        match self.next_book_move(board) {
            Some(string) => match execute::execute(&mut board, string.chars()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            None => {
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
        // #[cfg(test)]
        println!("[computer/mod]: Checking for Known opening(s)");

        let mut suitable_openings: Vec<&Opening> = Vec::new();

        // initially add all openings as suitable.
        for (i, opening) in self.openings.iter().enumerate() {
            suitable_openings.push(&opening);
        }

        // Edge case if no history exist.

        let mut ptn = 0;
        let mut cur_team = Team::White;

        // Move pointer.
        while !suitable_openings.is_empty() {
            // Check if ptn from any opening matches History.
            match board.history.get(cur_team, ptn) {
                Some(mov) => {
                    match &mov.label {
                        Some(label) => {
                            println!("[computer/mod]: Checking if previous move was a book move");
                            // Check if any suitable_opening matches, remove those that do not match.
                            // for idx in &suitable_openings {
                            //     let candidate: &Opening = self.openings.get(*idx)?;
                            //     // if candidate.reply == label
                            //     // FIXME: History doesn't always have labels, this must be a requirement for this to work.
                            //     // if let Some(res) = candidate.moves.get(ptn) {
                            //     //     return Some(res.to_string());
                            //     // }
                            // }
                        }
                        None => return None,
                    }
                }
                None => {
                    println!("[computer/mod]: Up to date, look for next move to perform.");
                    // Check if suitable_opening has a follow-up move.
                    // for idx in &suitable_openings {
                    //     let candidate: &Opening = openings.get(*idx)?;
                    //     if let Some(res) = candidate.moves.get(ptn) {
                    //         return Some(res.to_string());
                    //     }
                    // }
                    // Return all (or just one?) suitable_openings.

                    println!(
                        "[computer/mod]: Selected reply: {}",
                        suitable_openings[0].reply
                    );
                    return Some(suitable_openings[0].reply.clone());

                    // return Some(openings.get(suitable_openings.pop()?)?);
                }
            }

            // Iterate ptn.
            ptn += 1;
            cur_team = cur_team.not();
        }

        None
    }
}
