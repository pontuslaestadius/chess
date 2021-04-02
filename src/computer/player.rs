use crate::execute;
use crate::input;
use crate::Board;
use std::io;

pub fn action(mut board: &mut Board) -> io::Result<()> {
    let input = input::read()?;
    match execute::execute(&mut board, input.chars()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
