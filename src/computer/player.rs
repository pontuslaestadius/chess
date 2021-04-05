use crate::computer::Playable;
use crate::execute;
use crate::input;
use crate::Board;
use std::io;

pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Player {}
    }
}

impl Playable for Player {
    fn action(&self, mut board: &mut Board) -> io::Result<()> {
        let input = input::read()?;
        match execute::execute(&mut board, input.chars()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
