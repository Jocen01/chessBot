use crate::{board::Board, pice::Pice};

mod pice;
mod board;
mod singlemove;
mod constants;


#[derive(Debug,PartialEq)]
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

#[derive(Debug,PartialEq)]
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
            panic!("not a valid color")
        }
    }
}

fn main() {
    println!("Hello, world!");
    let pawn = Pice::new(PiceType::Pawn, Color::White, 8);
    println!("first pice {:?}", pawn);
    let b = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
    println!("{}", b);
    let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", b);
}



