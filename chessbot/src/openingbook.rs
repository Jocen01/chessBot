use crate::{board::Board, singlemove::Move};
use rand::prelude::*;

const SPECIFIC_OPENING: Option<&str> = None;

struct BookEntry{
    name: String,
    moves: Vec<Move>
}

impl BookEntry {
    fn new(name: &str, moves: &str) -> BookEntry{
        let mut board = Board::default();
        let move_strings: Vec<String> =  moves.split_ascii_whitespace().map(|s| s.to_string()).collect();
        let moves: Vec<Move> = move_strings.iter()
        .map_while(|s| {
            let res = board.move_from_long_algebraic_notation(s.to_string());
            if let Some(mv) = res{
                board.make_move(mv);
            }
            res
        })
        .collect();
        BookEntry { name: name.into(), moves  }
    }

    fn get_next(&self) -> Option<&Move>{
        self.moves.first()
    }
}

pub struct Book{
    entries: Vec<BookEntry>
}

impl Book {
    pub fn new() -> Book{
        let res: Vec<BookEntry> = ENTRIES.iter().map(|v|{
            BookEntry::new(v[0], v[1])
        }).collect();
        Book{ entries: res }
    }

    pub fn play_move(&mut self, game_move: &Move){
        self.entries.retain_mut(|book_entry|{
            if let Some(opening) = SPECIFIC_OPENING {
                if book_entry.name != opening.to_string() { 
                    return false; 
                }
            }
            if let Some(mv) = book_entry.moves.first(){
                if mv.from_to_mask() == game_move.from_to_mask() && book_entry.moves.len() >= 2{
                    book_entry.moves.remove(0);
                    return true;
                }
            }
            false
        });
    }

    pub fn get_random_move(&mut self) -> Option<(Move, &String)>{   
        let mut rng = rand::thread_rng();
        self.entries.retain(|book_entry| {
            if let Some(opening) = SPECIFIC_OPENING {
                if book_entry.name != opening.to_string() { 
                    return false; 
                }
            }
            true
        });
        if let Some(be) = self.entries.choose(&mut rng){
            if let Some(mv) = be.get_next() {
                return Some((*mv, &be.name));
            }
        }
        None
    }
}


































const ENTRIES: [[&str;2];248] = [["Bird","f2f4 e7e5 f4e5 d7d6 e5d6 f8d6 g1f3 g7g5 g2g3 g5g4 f3h4 g8e7 d2d4"],
["Bird","f2f4 d7d5 g1f3 g8f6 e2e3 g7g6 b2b3 f8g7 c1b2 e8g8 f1e2 c7c5 e1g1 b8c6 f3e5 d8c7"],
["Bird","f2f4 d7d5 g1f3 g8f6 e2e3 c8g4 h2h3 g4f3 d1f3 b8d7 b1c3 c7c6 g2g4"],
["Bird","f2f4 d7d5 g1f3 g8f6 e2e3 c8g4 f1e2 e7e6 e1g1 f8d6 b2b3 e8g8 c1b2 c7c5"],
["Bird","f2f4 d7d5 g1f3 c7c5 e2e3 b8c6 f1b5 c8d7 b2b3 g8f6 c1b2 d8b6 b5c6 d7c6 e1g1"],
["Bird","f2f4 d7d5 g1f3 c7c5 e2e3 b8c6 f1b5 c8d7 b2b3 g8f6 b5c6 d7c6 c1b2 e7e6 e1g1 f8e7"],
["Bird","f2f4 d7d5 g1f3 g8f6 e2e3 g7g6 f1e2 f8g7 e1g1 e8g8 d2d3 c7c5 d1e1 b8c6 b1c3 d5d4"],
["Bird","f2f4 d7d5 g1f3 g8f6 g2g3 c8g4 f1g2 g4f3 g2f3 c7c6 e1g1 b8d7"],
["Bird","f2f4 d7d5 g1f3 g8f6 e2e3 g7g6 b2b3 f8g7 c1b2 e8g8 f1e2 c7c5 e1g1 b8c6 f3e5 c8d7"],
["Bird","f2f4 d7d5 g1f3 g8f6 e2e3 e7e6 b2b3 c7c5 c1b2 b8c6 f1b5 f8e7 b5c6 b7c6 f3e5 c8b7"],
["Bird","f2f4 d7d5 b2b3 g8f6 e2e3 d5d4 f1d3 d4e3 d2e3"],
["Bird","f2f4 d7d5 b2b3 g8f6 e2e3 c7c5 g1f3 b8c6 f1b5 c8d7 c1b2 e7e6 e1g1 f8e7 d2d3"],
["Bird","f2f4 g7g6 g1f3 f8g7 g2g3 g8f6 f1g2 e8g8 e1g1 c7c5 d2d3 d7d5 d1e1 d5d4 b1a3 b8c6"],
["Bird","f2f4 c7c5 b2b3 b8c6 c1b2 b7b6 e2e3 c8b7 g1f3 g8f6 b1c3 d7d6 f1b5 a7a6 b5c6 b7c6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6 f1e2 e7e5 d4b3 f8e7 c1e3 e8g8"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e6 f1e2 a7a6 c1e3 f8e7 e1g1 b8c6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 f2f3 g7g6 c2c4 f8g7 c1e3 e8g8 d1d2 b8c6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6 f1e2 e7e5 d4b3 f8e7 c1g5 c8e6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 f1b5 c8d7 b5d7 b8d7 e1g1 g8f6 f1e1 d8c7 c2c3 e7e6 d2d4 c5d4"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 f1b5 c8d7 b5d7 d8d7 e1g1 b8c6 c2c3 g8f6 f1e1 e7e6 d2d4 c5d4"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 f1b5 c8d7 b5d7 d8d7 e1g1 b8c6 c2c3 g8f6 d2d4 f6e4 d4d5 c6e5"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 f1b5 c8d7 b5d7 d8d7 c2c4 b8c6 b1c3 g8f6 d2d4 c5d4 f3d4 g7g6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 f1b5 c8d7 b5d7 b8d7 e1g1 g8f6 d1e2 e7e6 b2b3 f8e7 c1b2 e8g8"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 d1d4 b8c6 f1b5 c8d7 b5c6 d7c6 b1c3 g8f6 c1g5 e7e6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 d1d4 b8c6 f1b5 c8d7 b5c6 d7c6 b1c3 g8f6 c1g5 e7e5"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 d1d4 g8f6 e4e5 b8c6 f1b5 d8a5 b1c3 a5b5 c3b5 c6d4"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6 c1g5 e7e6 f2f4 f8e7 d1f3 d8c7"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6 f1e2 e7e5 d4b3 f8e7 e1g1 e8g8"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6 c1e3 e7e5 d4b3 c8e6 f2f3 f8e7"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 g7g6 c1e3 f8g7 f2f3 e8g8 d1d2 b8c6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e6 f1e2 f8e7 e1g1 e8g8 f2f4 b8c6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e6 g2g4 h7h6 h2h4 b8c6 h1g1 h6h5"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e6 g2g4 h7h6 g4g5 h6g5 c1g5 b8c6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e6 g2g4 h7h6 h2h4 f8e7 h1g1 d6d5"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e6 c1e3 f8e7 f2f3 a7a6 d1d2 b8c6"],
["Sicilian","e2e4 c7c5 g1f3 e7e6 d2d4 c5d4 f3d4 b8c6 b1c3 a7a6 g2g3 g8e7 d4b3 d7d6 a2a4 b7b6"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6 f1e2 e7e6 e1g1 f8e7 f2f4 d8c7"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6 f1e2 e7e6 e1g1 f8e7 f2f4 e8g8"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 g7g6 c1e3 f8g7 f1e2 b8c6 e1g1 e8g8"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e6 f1c4 a7a6 a2a3 f8e7 c4a2 e8g8"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e6 g2g4 h7h6 f1g2 b8c6 h2h3 c8d7"],
["Sicilian","e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e6 f2f4 b8c6 c1e3 e6e5 d4f3 f6g4"],
["Sicilian","e2e4 c7c5 g1f3 b8c6 d2d4 c5d4 f3d4 e7e5 d4b5 d7d6 c2c4 f8e7 b1c3 a7a6 b5a3 h7h6"],
["Sicilian","e2e4 c7c5 g1f3 b8c6 d2d4 c5d4 f3d4 g8f6 b1c3 e7e5 d4b5 d7d6 c1g5 a7a6 b5a3 b7b5"],
["Sicilian","e2e4 c7c5 g1f3 b8c6 d2d4 c5d4 f3d4 g8f6 b1c3 d7d6 c1g5 e7e6 d1d2 a7a6 e1c1 c8d7"],
["Sicilian","e2e4 c7c5 g1f3 b8c6 d2d4 c5d4 f3d4 g8f6 b1c3 d7d6 c1g5 e7e6 d1d2 a7a6 e1c1 h7h6"],
["Sicilian","e2e4 c7c5 g1f3 b8c6 d2d4 c5d4 f3d4 g8f6 b1c3 d7d6 f1c4 e7e6 c1e3 f8e7 d1e2 a7a6"],
["Sicilian","e2e4 c7c5 g1f3 b8c6 d2d4 c5d4 f3d4 g8f6 b1c3 d7d6 f1c4 d8b6 d4b3 e7e6 e1g1 f8e7"],
["Sicilian","e2e4 c7c5 g1f3 b8c6 f1b5 e7e6 e1g1 g8e7 c2c3 d7d5 e4d5 d8d5 f1e1 a7a6 b5c6 e7c6"],
["Sicilian","e2e4 c7c5 g1f3 e7e6 d2d4 c5d4 f3d4 g8f6 b1c3 d7d6 g2g4 h7h6 h2h4 b8c6 h1g1 d6d5"],
["Sicilian","e2e4 c7c5 g1f3 e7e6 d2d4 c5d4 f3d4 g8f6 b1c3 d7d6 g2g4 h7h6 h2h4 b8c6 h1g1 h6h5"],
["Sicilian","e2e4 c7c5 g1f3 e7e6 d2d4 c5d4 f3d4 g8f6 b1c3 d7d6 f1e2 f8e7 e1g1 e8g8 c1e3 b8c6"],
["Sicilian","e2e4 c7c5 g1f3 e7e6 d2d4 c5d4 f3d4 g8f6 b1c3 d7d6 c1e3 a7a6 f2f3 b7b5 d1d2 b8d7"],
["Sicilian","e2e4 c7c5 g1f3 e7e6 d2d4 c5d4 f3d4 a7a6 b1c3 d8c7 g2g3 f8b4"],
["Sicilian","e2e4 c7c5 g1f3 e7e6 d2d4 c5d4 f3d4 b8c6 b1c3 d8c7 c1e3 a7a6 d1d2 g8f6 e1c1 f8b4"],
["Sicilian","e2e4 c7c5 b1c3 b8c6 g2g3 g7g6 f1g2 f8g7 d2d3 d7d6 c1e3 e7e6 d1d2 a8b8 g1e2 c6d4"],
["Sicilian","e2e4 c7c5 b1c3 b8c6 g2g3 g7g6 f1g2 f8g7 d2d3 d7d6 f2f4 e7e6 g1f3 g8e7 e1g1 e8g8"],
["Sicilian","e2e4 c7c5 b1c3 b8c6 f2f4 g7g6 g1f3 f8g7 f1c4 e7e6 f4f5 g8e7 f5e6 d7e6"],
["Sicilian","e2e4 c7c5 b1c3 b8c6 f2f4 g7g6 g1f3 f8g7 f1c4 e7e6 e1g1 g8e7 d2d3 e8g8 d1e1 d7d5"],
["Sicilian","e2e4 c7c5 b1c3 b8c6 f2f4 g7g6 g1f3 f8g7 f1b5 c6d4 e1g1 a7a6 b5d3 d7d6 f3d4 c5d4"],
["Sicilian","e2e4 c7c5 b1c3 b8c6 g1f3 g7g6 d2d4 c5d4 f3d4 f8g7 c1e3 g8f6 f1c4 e8g8 c4b3 d7d6"],
["Sicilian","e2e4 c7c5 c2c3 d7d5 e4d5 d8d5 d2d4 g8f6 g1f3 c8g4 f1e2 e7e6 h2h3 g4h5 e1g1 b8c6"],
["Sicilian","e2e4 c7c5 c2c3 d7d5 e4d5 d8d5 d2d4 g8f6 g1f3 c8g4 f1e2 e7e6 e1g1 b8c6 c1e3 c5d4"],
["Sicilian","e2e4 c7c5 d2d4 c5d4 c2c3 d4c3 b1c3 b8c6 g1f3 e7e6 f1c4 a7a6 e1g1 g8e7 c1g5 f7f6"],
["Sicilian","e2e4 c7c5 f2f4 e7e6 g1f3 b8c6 b1c3 a7a6 g2g3 d7d5 f1g2 d5d4"],
["Sicilian","e2e4 c7c5 f2f4 e7e6 g1f3 b8c6 b1c3 a7a6 g2g3 d7d5 d2d3 d5d4 c3e2 g8f6"],
["Sicilian","e2e4 c7c5 f2f4 e7e6 g1f3 b8c6 b1c3 a7a6 g2g3 d7d5 e4e5 g8e7 f1g2 e7f5 e1g1 h7h5"],
["Sicilian","e2e4 c7c5 f2f4 e7e6 g1f3 d7d5 f1b5 c8d7 b5d7 b8d7 d2d3 f8d6 e1g1 g8e7 c2c4 e8g8"],
["Sicilian","e2e4 c7c5 f2f4 b8c6 g1f3 g7g6 f1b5 f8g7 b5c6 b7c6 d2d3 d7d6 e1g1 g8f6 b1c3 e8g8"],
["Sicilian","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 d7d6 d4d5"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 d7d6 c1e3 f8e7 b1d2 e8g8 d4e5 c6e5 f3e5 d6e5"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 d7d5 f1b5 f6e4 f3e5 c8d7 d1b3 c6e5 b3d5 d8e7"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 e5d4 e4e5 f6d5 f1b5 a7a6 b5c6 d7c6 f3d4 f8e7"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 f6e4 d4d5 c6b8 f1d3 e4c5 f3e5 c5d3 e5d3 d7d6"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 f6e4 d4d5 c6e7 f3e5 e7g6 f1d3 g6e5 d3e4 f8c5"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 f6e4 d4d5 c6b8 f3e5 f8c5 d1g4 e8g8 g4e4 d7d6"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 f6e4 d4d5 c6e7 f3e5 e7g6 e5g6 h7g6 f1d3 e4f6"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 f6e4 d4d5 c6b8 f1d3 e4c5 f3e5 c5d3 e5d3 f8e7"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 d7d5 d1a4 f7f6 f1b5"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 d7d5 d1a4 c8d7 e4d5 c6d4 a4d1 d4f3 d1f3 g8f6 f1c4 e5e4"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 d7d5 d1a4 d8d6 f1b5 c8d7 e4d5 d6d5 e1g1 e8c8 b5c4 d5d6"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 d7d5 d1a4 d8d6 e4d5 d6d5 f1b5"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 e5d4 e4e5 f6d5 c3d4 f8b4 c1d2 b4d2 d1d2 e8g8"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 e5d4 e4e5 f6d5 c3d4 d7d6 f1b5 f8e7 b1c3 c8e6"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 e5d4 e4e5 f6e4 c3d4 d7d5 b1c3 f8b4 d1b3 e8g8"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 e5d4 e4e5 f6e4 d1e2 f7f5 e5f6 d7d5 b1d2 d8f6"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 f6e4 d4d5 c6e7 f3e5 e7g6 d1d4 e4d6 e5f3 f8e7"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 d7d5 f1b5 e5d4 e4e5 f6e4 f3d4 c8d7 b5c6 b7c6"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 d7d5 f1b5 f6e4 f3e5 c8d7 e5d7 d8d7 e1g1"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 d7d5 f1b5 f6e4 f3e5 c8d7 b5c6 d7c6 e1g1 f8e7"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 d7d5 f1b5 e5d4 e4e5 f6e4 c3d4 f8b4 b1d2 e8g8"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 g8f6 d2d4 d7d5 f1b5 f6e4 f3e5 c8d7 b5c6 d7c6 d1f3 d8f6"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 d7d5 d1a4 f7f6 d2d3 c8e6 c1e3 d8d7 b1d2"],
["Ponziani","e2e4 e7e5 g1f3 b8c6 c2c3 d7d5 d1a4 f7f6 f1b5 g8e7 e4d5 d8d5 d2d4"],
["Ponziani","e2e4 e7e5 g1f3 g8f6 f3e5 d7d6 e5f3 f6e4 d1e2 d8e7 d2d3 e4f6 c1g5 e7e2 f1e2 f8e7"],
["Petroff","e2e4 e7e5 g1f3 g8f6 f3e5 d7d6 e5f3 f6e4 d1e2 d8e7 d2d3 e4f6 b1c3 e7e2 f1e2 f8e7"],
["Petroff","e2e4 e7e5 g1f3 g8f6 f3e5 d7d6 e5f3 f6e4 d2d4 d6d5 f1d3 b8c6 e1g1 c8g4 c2c3 f8e7"],
["Petroff","e2e4 e7e5 g1f3 g8f6 f3e5 f6e4 d1e2 d8e7 e2e4 d7d6 d2d4 d6e5 d4e5 b8c6 f2f4 f7f6"],
["Petroff","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 g1f3 g8f6 h2h3 c8f5 b2b4 a5b6 a1b1 e7e6"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 d2d4 g8f6 g1f3 c7c6 f1c4 c8f5 c1d2 e7e6 d1e2 f8b4"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 d2d4 g8f6 g1f3 c7c6 h2h3 c8f5 f1d3 f5d3 d1d3 e7e6"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 d2d4 g8f6 g1f3 c8g4 h2h3 g4f3 d1f3 c7c6 c1d2 b8d7"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 d2d4 g8f6 g1f3 c8f5 f1c4 e7e6 c1d2 c7c6 d1e2 f8b4"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 d2d4 g8f6 g1f3 c8g4 h2h3 g4h5 g2g4 h5g6 f3e5 e7e6"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 d2d4 g8f6 f1c4 c7c6 g1f3 c8f5 c1d2 e7e6 c3d5 a5d8"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 g1f3 g8f6 h2h3 c8f5 f1c4 e7e6 d2d3 c7c6 a2a3 b8d7"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 d2d4 g8f6 g1f3 c8f5 f1c4 e7e6 e1g1 c7c6 f1e1 f8b4"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 d2d4 g8f6 g1f3 c8f5 f1c4 e7e6 e1g1 c7c6 f3e5 b8d7"],
["Scandinavian","e2e4 d7d5 e4d5 d8d5 b1c3 d5a5 g1f3 g8f6 h2h3 c8f5 f1c4 e7e6 e1g1 c7c6 d2d3 b8d7"],
["Scandinavian","e2e4 d7d5 e4d5 g8f6 d2d4 f6d5 c2c4 d5b6 g1f3 c8g4 c4c5 b6d7 f1c4 e7e6 h2h3 g4f3"],
["Scandinavian","e2e4 d7d5 e4d5 g8f6 d2d4 f6d5 c2c4 d5b6 g1f3 c8g4 f1e2 e7e6 e1g1 b8c6 b2b3 f8e7"],
["Scandinavian","e2e4 d7d5 e4d5 g8f6 d2d4 f6d5 c2c4 d5b6 b1c3 e7e5 d4e5 d8d1 c3d1 b8c6 f2f4 c8e6"],
["Scandinavian","e2e4 d7d5 e4d5 g8f6 d2d4 f6d5 c2c4 d5b6 g1f3 g7g6 b1c3 f8g7 c4c5 b6d5 f1c4 c7c6"],
["Scandinavian","e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 g1f3 c8g4 f1e2 e7e6 e1g1 f8e7 c2c4 d5b6 b1c3 e8g8"],
["Alekhine","e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 g1f3 d6e5 f3e5 g7g6 f1c4 c7c6 e1g1 f8g7 f1e1 e8g8"],
["Alekhine","e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 g1f3 g7g6 f1c4 c7c6 e1g1 f8g7 e5d6 d8d6 f1e1 e8g8"],
["Alekhine","e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 g1f3 c8g4 f1e2 e7e6 e1g1 f8e7 c2c4 d5b6 h2h3 g4h5"],
["Alekhine","e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 c2c4 d5b6 f2f4 d6e5 f4e5 b8c6 c1e3 c8f5 b1c3 e7e6"],
["Alekhine","e2e4 g8f6 e4e5 f6d5 c2c4 d5b6 d2d4 d7d6 e5d6 c7d6 b1c3 g7g6 c1e3 f8g7 a1c1 e8g8"],
["Alekhine","e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 g1f3 g7g6 f1c4 d5b6 c4b3 f8g7 f3g5 e7e6 f2f4 d6e5"],
["Alekhine","e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 g1f3 g7g6 f1c4 d5b6 c4b3 f8g7 f3g5 d6d5 f2f4 f7f6"],
["Alekhine","e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 g1f3 g7g6 f1c4 d5b6 c4b3 f8g7 d1e2 b8c6 e1g1 e8g8"],
["Alekhine","e2e4 e7e6 d2d4 d7d5 b1c3 f8b4 e4e5 c7c5 a2a3 b4c3 b2c3 g8e7 d1g4 d8c7 g4g7 h8g8"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 f8b4 e4e5 c7c5 a2a3 b4c3 b2c3 g8e7 d1g4 e8g8 f1d3 b8c6"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 f8b4 e4e5 c7c5 a2a3 b4c3 b2c3 g8e7 d1g4 e8g8 f1d3 f7f5"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 g8f6 c1g5 f8e7 e4e5 f6d7 g5e7 d8e7 f2f4 a7a6 g1f3 c7c5"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 g8f6 c1g5 f8e7 e4e5 f6d7 g5e7 d8e7 f2f4 e8g8 g1f3 c7c5"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 g8f6 c1g5 f8e7 e4e5 f6d7 h2h4 a7a6 d1g4 e7g5 h4g5 c7c5"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 g8f6 c1g5 f8e7 e4e5 f6d7 h2h4 e7g5 h4g5 d8g5 g1h3 g5e7"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 g8f6 c1g5 d5e4 c3e4 f8e7 g5f6 e7f6 g1f3 b8d7 d1d2 e8g8"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 g8f6 c1g5 d5e4 c3e4 b8d7 e4f6 d7f6 g1f3 f8e7 f1d3 e8g8"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 g8f6 e4e5 f6d7 f2f4 c7c5 g1f3 b8c6 c1e3 c5d4 f3d4 f8c5"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 f8b4 e4e5 g8e7 a2a3 b4c3 b2c3 b7b6 d1g4 e7g6 h2h4 h7h5"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 f8b4 e4e5 g8e7 a2a3 b4c3 b2c3 c7c5 d1g4 b8c6 g1f3 e8g8"],
["French","e2e4 e7e6 d2d4 d7d5 b1c3 f8b4 e4e5 g8e7 a2a3 b4c3 b2c3 c7c5 d1g4 e8g8 g1f3 c5c4"],
["French","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 f2f4 f8g7 g1f3 c7c5 f1b5 c8d7 e4e5 f6g4"],
["Pirc","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 f2f4 f8g7 g1f3 e8g8 f1d3 b8c6 e1g1 c8g4 e4e5 d6e5"],
["Pirc","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 f2f4 f8g7 g1f3 e8g8 f1d3 b8c6 e4e5 d6e5 f4e5 f6h5"],
["Pirc","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 f2f4 f8g7 g1f3 e8g8 f1d3 b8c6 e1g1 e7e5 f4e5 d6e5"],
["Pirc","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 f2f4 f8g7 g1f3 c7c5 d4c5 d8a5 f1d3 a5c5 d1e2 e8g8"],
["Pirc","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 f2f4 f8g7 g1f3 c7c5 f1b5 c8d7 b5d7"],
["Pirc","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 g1f3 f8g7 f1e2 e8g8 e1g1 c7c6 a2a4 b8d7 h2h3 e7e5"],
["Pirc","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 g1f3 f8g7 f1e2 e8g8 e1g1 c7c6 a2a4 a7a5 h2h3 b8a6"],
["Pirc","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 g1f3 f8g7 f1e2 e8g8 e1g1 c7c6 f1e1 b8d7 c1f4 d8a5 f3d2 a5c7"],
["Pirc","e2e4 d7d6 d2d4 g8f6 b1c3 g7g6 g1f3 f8g7 f1e2 e8g8 e1g1 c7c6 f1e1 d8c7 c1f4 f6h5"],
["Pirc","e2e4 d7d6 d2d4 g7g6 b1c3 f8g7 c1e3 a7a6 d1d2 b7b5 f2f3 b8d7 h2h4 h7h5 g1h3 c8b7"],
["Pirc","e2e4 d7d6 d2d4 g7g6 b1c3 f8g7 c1e3 a7a6 d1d2 b8d7 f2f3 b7b5 g2g4 c8b7 g1e2 c7c5"],
["Pirc","e2e4 d7d6 d2d4 g7g6 b1c3 f8g7 c1e3 a7a6 d1d2 b8d7 f2f3 b7b5 h2h4 h7h5 g1h3 c8b7"],
["Pirc","e2e4 d7d6 d2d4 g7g6 b1c3 f8g7 c1e3 c7c6 d1d2 b7b5 f1d3 b8d7 g1f3 d8c7 e1g1 g8f6"],
["Pirc","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 c8f5 e4g3 f5g6 h2h4 h7h6 g1f3 b8d7 h4h5 g6h7"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 c8f5 e4g3 f5g6 g1f3 b8d7 f1d3 e7e6 e1g1 g8f6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 c8f5 e4g3 f5g6 g1f3 b8d7 f1d3 g6d3 d1d3 g8f6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 b8d7 e4g5 g8f6 f1d3 e7e6 g1f3 f8d6 d1e2 h7h6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 b8d7 g1f3 g8f6 e4g3 e7e6 f1d3 c6c5 e1g1 c5d4"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 b8d7 g1f3 g8f6 e4f6 d7f6 f1c4 c8f5 d1e2 e7e6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 b8d7 g1f3 g8f6 e4f6 d7f6 f3e5 c8e6 f1e2 g7g6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 g7f6 c2c3 d8d5 c1e3 h8g8 d1b3 d5b3"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 g7f6 c2c3 c8f5 g1e2 b8d7 e2g3 f5g6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 g7f6 c2c3 c8f5 g1f3 e7e6 f1d3 f8d6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 g7f6 g1f3 c8f5 f1d3"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 g7f6 g1f3 c8g4 f1e2 d8c7 c1e3 b8d7"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 g7f6 g1f3 c8g4 f1e2 e7e6 e1g1 d8c7"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 g7f6 g1f3 c8g4 f1e2 e7e6 c1f4 f8d6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 e7f6 c2c3 f8d6 f1d3 e8g8 d1c2 f8e8"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 e7f6 c2c3 f8d6 f1d3 e8g8 g1e2 f8e8"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 e7f6 f1c4 f8d6 d1e2 d8e7 e2e7 e8e7"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 e7f6 f1c4 f8d6 d1h5 e8g8 g1e2 b8d7"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 e7f6 f1c4 d8e7 d1e2 e7e2 g1e2 c8e6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 g8f6 e4f6 e7f6 g1f3 f8d6 f1d3 e8g8 e1g1 c8g4"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 b8d7 f1c4 g8f6 e4g5 e7e6 d1e2 d7b6 c4d3 h7h6"],
["Caro Kann","e2e4 c7c6 d2d4 d7d5 b1c3 d5e4 c3e4 b8d7 f1c4 g8f6 e4g5 e7e6 d1e2 d7b6 c4b3 h7h6"],
["Caro Kann","e2e4 e7e5 d2d4 e5d4 d1d4 b8c6 d4e3 g8f6 a2a3 g7g6"],
["Centre Game","e2e4 e7e5 d2d4 e5d4 d1d4 b8c6 d4e3 g8f6 f1d3 f8b4 c1d2 e8g8 a2a3 b4d2 b1d2 d7d5"],
["Centre Game","e2e4 e7e5 d2d4 e5d4 d1d4 b8c6 d4e3 g8f6 c1d2 f8b4 a2a3 b4d2 b1d2 e8g8 e1c1 f8e8"],
["Centre Game","e2e4 e7e5 d2d4 e5d4 d1d4 b8c6 d4a4 g8f6 g1f3 d7d6 c1g5 f8e7 b1c3 e8g8 e1c1 c8d7"],
["Centre Game","e2e4 e7e5 d2d4 e5d4 d1d4 b8c6 d4a4 g8f6 c1g5 f8e7 b1c3 e8g8 e1c1 f8e8"],
["Centre Game","e2e4 e7e5 d2d4 e5d4 d1d4 b8c6 d4a4 g8f6 b1c3 f8c5 f2f3 e8g8 c1g5 c5e7"],
["Centre Game","e2e4 e7e5 d2d4 e5d4 d1d4 b8c6 d4a4 f8c5 g1f3 g8e7"],
["Centre Game","e2e4 e7e5 g1f3 d7d6 d2d4 g8f6 d4e5 f6e4 d1d5 e4c5 c1g5 f8e7 e5d6 d8d6 b1c3 c8e6"],
["Robatsch","e2e4 g7g6 d2d4 f8g7 c2c3 d7d6 f2f4 g8f6 f1d3 e7e5 g1f3 c8g4 e1g1 e8g8"],
["Robatsch","e2e4 g7g6 d2d4 f8g7 g1f3 d7d6 f1c4"],
["Robatsch","e2e4 g7g6 d2d4 f8g7 c2c3 d7d6 g1f3 g8f6 f1d3 e8g8 h2h3 b8c6"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 b1c3 g8f6 c1g5 f8e7 e2e3 e8g8 g1f3 h7h6 g5h4 f6e4 h4e7 d8e7"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 b1c3 g8f6 c1g5 f8e7 e2e3 e8g8 g1f3 h7h6 g5f6 e7f6 a1c1 c7c6"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 b1c3 g8f6 c1g5 f8e7 e2e3 e8g8 g1f3 b7b6 c4d5 e6d5 f1d3 c8b7"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 b1c3 g8f6 c1g5 f8e7 c4d5 e6d5 e2e3 b8d7 f1d3 c7c6 g1e2 d7b6"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 b1c3 g8f6 c1g5 f8e7 e2e3 e8g8 g1f3 f6e4 g5e7 d8e7 d1c2 e4c3"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 b1c3 g8f6 g1f3 f8e7 c1f4 e8g8 e2e3 c7c5 d4c5 e7c5 d1c2 b8c6"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 b1c3 g8f6 c4d5 e6d5 c1g5 c7c6 e2e3 f8e7 f1d3 f6e4 g5e7 d8e7"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 b1c3 c7c6 g1f3 g8f6 e2e3 b8d7 f1d3 d5c4 d3c4 b7b5 c4d3 c8b7"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 b1c3 g8f6 c4d5 e6d5 c1g5 f8e7 e2e3 e8g8 f1d3 b8d7"],
["Queens Gambit Declined","d2d4 d7d5 c2c4 e7e6 g1f3 g8f6 g2g3 f8e7 f1g2 e8g8 e1g1 d5c4 d1c2 a7a6 c2c4 b7b5"],
["Trompowsky","d2d4 d7d5 c1g5 h7h6 g5h4 c7c6 e2e3 d8b6 f1d3 b8d7"],
["Trompowsky","d2d4 d7d5 c1g5 c7c6 e2e3 d8b6"],
["Trompowsky","d2d4 d7d5 c1g5 g8f6 g5f6 e7f6 e2e3 f8d6 c2c4 d5c4 f1c4 e8g8 b1c3 b8d7"],
["Queens Pawn","d2d4 d7d5 e2e3 g8f6 b1d2 c7c5 c2c3 e7e6 f1d3 b8d7 f2f4 d8c7 g1f3 b7b6 f3e5 f8e7"],
["Queens Pawn","d2d4 d7d5 e2e3 g8f6 f1d3 e7e6 b1d2 c7c5 c2c3 b8d7 f2f4 d8c7 h2h3 f8e7 g1f3 e8g8"],
["Queens Pawn","d2d4 d7d5 e2e3 g8f6 c2c4 c7c6 b1c3 e7e6 c4d5 e6d5 f1d3 f8d6 g1f3 b8d7 e1g1 e8g8"],
["English","c2c4 e7e5 b1c3 g8f6 g1f3 b8c6 g2g3 d7d5 c4d5 f6d5 f1g2 d5b6 e1g1 f8e7 d2d3 e8g8"],
["English","c2c4 e7e5 b1c3 g8f6 g1f3 b8c6 g2g3 d7d5 c4d5 f6d5 f1g2 d5b6 e1g1 f8e7 a2a3 e8g8"],
["English","c2c4 e7e5 b1c3 g8f6 g2g3 f8b4 f1g2 e8g8 g1f3 f8e8 e1g1 e5e4 f3d4 b4c3 b2c3 d7d6"],
["English","c2c4 e7e5 b1c3 g8f6 g2g3 f8b4 f1g2 e8g8 g1f3 b4c3 b2c3 e5e4 f3d4 c7c5 d4c2 d7d5"],
["English","c2c4 e7e5 b1c3 g8f6 g1f3 b8c6 e2e3 f8b4 d1c2 b4c3 c2c3 d8e7 a2a3 d7d5 d2d4 e5d4"],
["English","c2c4 e7e5 b1c3 g8f6 g1f3 b8c6 d2d4 e5d4 f3d4 f8b4 c1g5 h7h6 g5h4 b4c3 b2c3 c6e5"],
["English","c2c4 e7e5 b1c3 g8f6 g1f3 b8c6 g2g3 d7d5 c4d5 f6d5 f1g2 d5b6 e1g1 f8e7 a2a3 c8e6"],
["English","c2c4 e7e5 b1c3 b8c6 g2g3 g7g6 f1g2 f8g7 e2e4 g8e7 g1e2 e8g8 d2d3 d7d6 e1g1 f7f5"],
["English","c2c4 g8f6 b1c3 g7g6 g2g3 f8g7 f1g2 e8g8 g1f3 d7d6 e1g1 e7e5 d2d3 b8c6 a1b1 a7a5"],
["English","c2c4 g8f6 b1c3 c7c5 g1f3 e7e6 g2g3 b7b6 f1g2 c8b7 b2b3 f8e7 c1b2 e8g8"],
["English","c2c4 g8f6 b1c3 e7e5 d2d3 d7d5 c4d5 f6d5 g1f3 b8c6 a2a3 c8e6 g2g3 h7h6"],
["English","c2c4 g8f6 b1c3 e7e6 d2d4 f8b4 d1c2 e8g8 a2a3 b4c3 c2c3 b7b6 g2g3 c8b7 g1f3 d7d5"],
["English","c2c4 g8f6 b1c3 e7e6 g1f3 f8b4 g2g3 b4c3 b2c3 b7b6 f1g2 c8b7 e1g1 e8g8 d2d3"],
["English","c2c4 g8f6 g2g3 c7c6 f1g2 e7e5 d2d4 e5d4 d1d4 d7d5 g1f3 f8e7"],
["English","c2c4 e7e6 g2g3 d7d5 f1g2 d5c4 d1a4 b8d7 a4c4 c7c5 d2d3"],
["English","c2c4 g7g6 d2d4 f8g7 e2e4 d7d6 b1c3 e7e5 d4d5 f7f5 e4f5 c8f5 f1d3 g8e7"],
["English","c2c4 g7g6 g2g3 c7c5 f1g2 f8g7 b1c3 b8c6 a2a3 d7d6 g1f3 e7e5 e1g1 c8e6 d2d3 g8e7"],
["English","c2c4 c7c5 g2g3 g7g6 f1g2 f8g7 g1f3 b8c6 b1c3 d7d6 e1g1 g8h6 d2d4"],
["English","c2c4 c7c5 g1f3 g8f6 b1c3 e7e6 g2g3 b7b6 f1g2 c8b7 e1g1 f8e7 d2d4 c5d4 d1d4 d7d6"],
["English","c2c4 f7f5 d2d4 g8f6 g2g3 g7g6 f1g2 f8g7 g1f3 e8g8 b1c3 d7d6 e1g1 d8e8 b2b3 b8a6"],
["1. Nf3","g1f3 g8f6 c2c4 g7g6 b1c3 f8g7 d2d4 e8g8 c1g5 d7d6 e2e3 b8d7 f1e2 c7c6 e1g1 h7h6"],
["1. Nf3","g1f3 g8f6 c2c4 g7g6 b1c3 f8g7 g2g3 e8g8 f1g2 d7d6 e1g1 e7e5 d2d3 b8c6 a1b1 a7a5"],
["1. Nf3","g1f3 g8f6 c2c4 g7g6 b1c3 d7d5 c4d5 f6d5 g2g3 f8g7 f1g2 e7e5 d1a4 c8d7 a4b3 d5b6"],
["1. Nf3","g1f3 g8f6 g2g3 g7g6 f1g2 f8g7 e1g1 e8g8 d2d4 d7d5 c2c4 d5c4 b1a3 c4c3 b2c3 c7c5"],
["1. Nf3","g1f3 g8f6 g2g3 d7d5 f1g2 c7c6 e1g1 c8g4 d2d3 b8d7 b1d2 e7e5 e2e4 d5e4 d3e4 f8c5"],
["1. Nf3","g1f3 g8f6 c2c4 g7g6 b1c3 f8g7 e2e4 d7d6 d2d4 e8g8 f1e2 e7e5 e1g1 e5d4 f3d4 f8e8"],
["1. Nf3","g1f3 g8f6 d2d4 d7d5 c1g5 e7e6 b1d2 f8e7 e2e3 b8d7 f1d3 b7b6 e1g1 c8b7 f3e5 d7e5"],
["1. Nf3","g1f3 g8f6 d2d4 d7d5 c1g5 g7g6 b1c3 f8g7 g5f6 e7f6 e2e4 c7c6 e4d5 c6d5"],
["Larsen","b2b3 e7e5 c1b2 b8c6 e2e3 g8f6 f1b5 d7d6 g1e2 c8d7 e1g1 a7a6 b5c6 d7c6 d2d4 d8e7"],
["Larsen","b2b3 e7e5 c1b2 b8c6 e2e3 g8f6 f1b5 d7d6 g1f3 a7a6 b5c6 b7c6 e1g1 e5e4"],
["Larsen","b2b3 e7e5 c1b2 b8c6 e2e3 d7d5 f1b5 f8d6 b5c6 b7c6 g1f3 d8e7"],
["Larsen","b2b3 e7e5 c1b2 b8c6 e2e3 d7d5 f1b5 f7f6 g1e2"],
["Larsen","b2b3 e7e5 c1b2 b8c6 e2e3 g7g6 f2f4 f8g7 g1f3"],
["Dutch","d2d4 f7f5 g2g3 g8f6 f1g2 g7g6 g1f3 f8g7 e1g1 e8g8 c2c4 d7d6 b1c3 c7c6 d4d5 e7e5"],
["Dutch","d2d4 f7f5 c2c4 g8f6 b1c3 g7g6 f2f3 d7d6 e2e4 f8g7 e4e5 d6e5 d4e5 d8d1 e1d1 f6d7"],
["Dutch","d2d4 f7f5 c2c4 g8f6 g2g3 e7e6 f1g2 f8e7 g1f3 e8g8 e1g1 d7d5 b1c3 c7c6"],
["Dutch","d2d4 f7f5 g1f3 g8f6 c1g5 e7e6 b1d2 h7h6 g5f6 d8f6"],
["Dutch","d2d4 f7f5 b1c3 g8f6 c1g5 d7d5 g5f6 e7f6 e2e3 c8e6 f1d3 b8c6 g1e2 d8d7 a2a3 e8c8"],
["Dutch","d2d4 f7f5 g2g3 g8f6 f1g2 c7c6 g1f3 g7g6 e1g1 f8g7 b2b3 e8g8 c2c4 d7d6 b1c3 b8a6"],
["Dutch","d2d4 f7f5 g2g4 f5g4 e2e4 d7d6 h2h3 g4g3"],
["Dutch","d2d4 f7f5 g2g4 f5g4 e2e4 d7d6 h2h3 g8f6 h3g4 c8g4 f2f3 g4c8 b1c3 e7e5 c1e3 b8c6"],
["Dutch","d2d4 f7f5 g2g4 f5g4 e2e4 d7d6 h2h3 g8f6 b1c3"],
["Dutch","d2d4 f7f5 g2g4 f5g4 c1f4 g8f6 b1c3"],
["Dutch","d2d4 f7f5 g2g4 f5g4 h2h3 g4g3 f2g3 c7c5"],
["Dutch","d2d4 f7f5 e2e3 g8f6 c2c4 e7e6 g1f3 b7b6 f1e2 c8b7"],
["Polish","b2b4 e7e5 c1b2 f8b4 b2e5 g8f6 e2e3 b8c6 e5b2 e8g8 g1f3 d7d5 f1e2 f8e8 e1g1 c8g4"],
["Polish","b2b4 e7e5 c1b2 f8b4 b2e5 f7f6"],
["Polish","b2b4 e7e5 a2a3 d7d5 c1b2 f8d6 g1f3 d8e7"],
["Barcza","g2g3 d7d5 f1g2 g8f6 d2d3 c7c6 f2f4 g7g6 g1f3 f8g7"],
["Barcza","g2g3 d7d5 f1g2 g8f6 g1f3 c7c6 e1g1 c8g4 d2d3 b8d7"]];