use std::error;

use uci_engine::UciEngine;


mod pice;
mod board;
mod singlemove;
mod constants;
mod state;
mod uci_engine;
mod uci_message;
mod evaluate;
mod searcher;
mod transposition_table;
mod uciio;
mod movegenerator;
mod openingbook;


#[derive(Debug,PartialEq, Clone, Copy)]
pub enum PiceType {
    King = 6,
    Queen = 5,
    Rook = 4,
    Bishop = 3,
    Knight = 2,
    Pawn = 1
}


impl PiceType {
    pub fn char<'a>(i: u8) -> &'a str{
        match i & 7 {
            1 => "p",
            2 => "n",
            3 => "b",
            4 => "r",
            5 => "q",
            6 => "k",
            _ => "X"
        }
    }

    pub fn from_char(c: char) -> PiceType{
        match c.to_ascii_lowercase() {
            'k' => PiceType::King,
            'q' => PiceType::Queen,
            'r' => PiceType::Rook,
            'b' => PiceType::Bishop,
            'n' => PiceType::Knight,
            'p' => PiceType::Pawn,
            _ => panic!("{} is not a pice type", c)
        }
    }

    pub fn _type(i: u8) -> PiceType{
        match i & 7 {
            1 => PiceType::Pawn,
            2 => PiceType::Knight,
            3 => PiceType::Bishop,
            4 => PiceType::Rook,
            5 => PiceType::Queen,
            6 => PiceType::King,
            _ => panic!("Not a valid pice type")
        }
    }
}

#[derive(Debug,PartialEq, Clone, Copy)]
pub enum Color {
    White = 8,
    Black = 16
}

impl Color {
    pub fn from_char(c: char) -> Color{
        if c.is_uppercase() || c == 'w'{
            Color::White
        }else {
            Color::Black
        }
    }

    pub fn from_int(i: u8) -> Color{
        if i & 8 == 8 {
            Color::White
        }else if i & 16 == 16 {
            Color::Black
        }else {
            panic!("{} not a valid color", i)
        }
    }

    pub fn other(&self) -> Color{
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn to_0_1(&self) -> usize{
        match self {
            Color::White => 0,
            Color::Black => 1
        }
    }
}

#[allow(dead_code)]
fn vec_pos_to_bitmap(pos: Vec<u8>) -> u64{
    let mut res = 0;
    for i in pos{
        res |= 1<<i;
    }
    res
}

// macro_rules! read_str {
//     ($out:ident) => {
//         #[allow(unused_mut)]
//         let mut inner = String::new();
//         std::io::stdin().read_line(&mut inner).expect("A String");
//         let $out = inner.trim();
//     };
// }

fn main() -> Result<(), Box<dyn error::Error>> {
    let (thread_in,rx) = uciio::new_uci_in_tread();
    let (thread_out, tx) = uciio::new_uci_out_tread();
    let mut engine = UciEngine::new(tx, rx);
    engine.run()?;
    thread_in.join().expect("something went wrong");
    thread_out.join().expect("something went wrong");
    Ok(())
}




