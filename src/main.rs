extern crate colored;
extern crate paw;

mod board;
mod computer;
mod display;
mod execute;
mod game_loop;
mod input;
mod place;

use crate::board::history::History;
use crate::board::piece::Piece;
use crate::board::team::Team;
use crate::board::{Board, KingStatus, SqStatus};
use crate::place::entity::Entity;
use crate::place::optsq::OptSq;
use crate::place::sq::Sq;

// With the "paw" feature enabled in structopt
#[derive(structopt::StructOpt)]
struct Args {
    /// Parses input as one or more PGN(s), and execute them.
    #[structopt(long = "pgn")]
    pgn: Option<String>,
    /// Who is playing as White
    #[structopt(short = "w", long = "white", default_value = "player")]
    white: String,
    /// Who is playing as Black
    #[structopt(short = "b", long = "black", default_value = "player")]
    black: String,
}

#[paw::main]
fn main(args: Args) -> std::io::Result<()> {
    match args.pgn {
        Some(pgn) => game_loop::automatic_game_loop(pgn),
        None => game_loop::manual_game_loop(args.white, args.black),
    }
}
