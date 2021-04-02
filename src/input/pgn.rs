use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct Turn {
    // White doesn't need to be Option<T> since T will always be set, but the data type consistancy is nice.
    pub white: Option<String>,
    pub black: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct Instruction {
    pub white: String,
    pub black: Option<String>,
    pub turns: Vec<Turn>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Pgn {
    turns: Vec<Turn>,
    meta: Vec<String>,
}

// impl Pgn {
//     #[allow(dead_code)]
//     pub fn new() -> Self {
//         Pgn {
//             instructions: Vec::new(),
//             meta: Vec::new(),
//         }
//     }
// }

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let black = match &self.black {
            Some(s) => s,
            None => "",
        };
        write!(f, "{} {}", self.white, black)
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let black = match &self.black {
            Some(s) => s,
            None => "",
        };
        write!(f, "{} {}", self.white, black)
    }
}

pub fn parse<T: ToString>(game: T) -> Vec<Turn> {
    let mut vec: Vec<Turn> = Vec::new();
    // println!("game: {}", game);
    let string = game.to_string();
    let mut split: Vec<&str> = string.split_whitespace().into_iter().collect();
    // println!("split: {:?}", split);
    while split.len() > 3 {
        // We want to split it into groups of three because the format to decode is: "1. e4 e5"
        let drain: Vec<String> = split
            .drain(0..3)
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        // println!("drain: {:?}", drain);
        match drain[0].contains('{') {
            true => break,
            false => (),
        };
        // let nr = split.pop().unwrap().to_string();
        // println!("nr: {}", nr);
        let mut will_break = false;
        // If the annotation contains the meta data character '{}'
        // Then it is at the end of a game, e.g. 34. Qf4# {black wins by checkmate}
        let black = match drain[2].contains('{') {
            true => {
                will_break = true;
                None
            }
            false => Some(drain[2].clone()),
        };
        vec.push(Turn {
            white: Some(drain[1].clone()),
            black,
        });
        if will_break {
            break;
        }
    }
    vec
}
