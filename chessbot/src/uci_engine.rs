use rand::prelude::*;

use crate::{board::{self, Board}, singlemove::{Move, MoveType}};

#[derive(Clone)]
pub enum UciMessage {
    Uci,
    Debug(bool),
    IsReady,
    SetOption{
        name: String,
        value: String
    },
    Register{
        later: bool,
        name: String,
        code: String
    },
    UciNewGame,
    Position{
        fen: Option<String>,
        moves: Vec<Move>
    },
    Go{
        search_moves: Option<Vec<Move>>,
        ponder: bool,
        wtime: u64,
        btime: u64,
        winc: u64,
        binc: u64,
        depth: u8,
        nodes: u64,
        mate: u8,
        move_time: u64,
        infinite: bool
    },
    Stop,
    Ponderhit,
    Quit,

    // Engine to GUI

    Id{
        name: String,
        author: String
    },
    UciOk,
    ReadyOk,
    BestMove{
        best_move: Move,
        ponder: Option<Move>
    },
    CopyProtection,
    Registration,
    Info{
        depth: Option<u8>,
        seldepth: Option<u8>,
        time: u64,
        nodes: u64,
        pv: Option<Vec<Move>>,
        multipv: Option<Vec<Vec<Move>>>,
        // todo needs more fields

    },
    Option,
}

impl UciMessage {
    pub fn parse(s: String) -> UciMessage{
        if s == "uci"{
            UciMessage::Uci
        } else if s.starts_with("debug") {
            UciMessage::Debug(true)
        } else if s == "isready" {
            UciMessage::IsReady
        } else if s.starts_with("setoption") {
            let name_pos: usize = s.find("name").unwrap();
            let value_pos: usize = s.find("value").unwrap();

            // Extract the substring between "name" and "value"
            let name_value = s[name_pos + 4..value_pos].trim();

            // Extract the substring after "value"
            let value_value = s[value_pos + 5..].trim();
            UciMessage::SetOption { name: name_value.into(), value: value_value.into() }
        } else if s.starts_with("register") {
            panic!("not implemented")
        } else if s == "ucinewgame" {
            UciMessage::UciNewGame
        } else if s.starts_with("position") {
            let moves_pos = s.find("moves").unwrap();
            let fen: Option<String> = if s.contains("startpos") { None } else {
                Some(s[8..moves_pos].trim().into())
            };
            let mut board = if let Some(fenn) = fen.clone() {
                Board::from_fen(&fenn)
            }else{
                Board::default()
            };
            let mut moves_res = vec![];
            s[moves_pos + 5..].split_whitespace().into_iter().for_each(|move_str| {
                let moves = board.get_possible_moves_turn();
                let (from_square, to_square) = parse_algebraic_notation(move_str);
                moves.iter().filter(|&mv| {
                    mv.from() == from_square && mv.to() == to_square
                }).filter(|&mv|{
                    if move_str.len() == 5{
                        match move_str.chars().nth(4).unwrap() {
                            'q' => mv.move_type() == MoveType::PromotionQueen,
                            'r' => mv.move_type() == MoveType::PromotionRook,
                            'b' => mv.move_type() == MoveType::PromotionBishop,
                            'k' => mv.move_type() == MoveType::PromotionKnight,
                            _ => true
                        }
                    }else {
                        true
                    }
                }).for_each(|mv| {
                    moves_res.push(*mv);
                    board.make_move(*mv);

                });
            });
            UciMessage::Position { fen: fen, moves: moves_res }
        }else if s.starts_with("go") {
            UciMessage::Go { search_moves: None, ponder: false, wtime: 10000, btime: 10000, winc: 10000, binc: 10000, depth: 4, nodes: 10000000, mate: 100, move_time: 1000, infinite: false }
        } else if s == "stop" {
            UciMessage::Stop
        } else if s == "ponderhit" {
            UciMessage::Ponderhit
        } else if s == "quit" {
            UciMessage::Quit
        } else {
            panic!("not a valid uci message: {}", s);
        }
    }

    pub fn serialize(&self) -> String{
        match self {
            UciMessage::Uci | UciMessage::Debug(..) | UciMessage::IsReady | UciMessage::Register { .. } | UciMessage::UciNewGame | UciMessage::Position { .. } |UciMessage::Go { .. } | UciMessage::Stop | UciMessage::Ponderhit | UciMessage::Quit | UciMessage::SetOption { .. }=> {
                panic!("not serializable");
            },
            UciMessage::Id { name, author } => {
                format!("id name {}\nid author {}",name, author)
            },
            UciMessage::UciOk => {
                "uciok".into()
            },
            UciMessage::ReadyOk => {
                "readyok".into()
            },
            UciMessage::BestMove { best_move, ponder } => {
                if let Some(ponder_best) = ponder {
                    format!("bestmove {} ponder {}", best_move, ponder_best)
                }else {
                    format!("bestmove {}", best_move.long_algebraic_notation())
                }
            },
            UciMessage::CopyProtection => {
                panic!("not impl")
            },
            UciMessage::Registration => {
                panic!("not impl")
            },
            UciMessage::Info { .. } => {
                panic!("not impl")
            },
            UciMessage::Option => {
                panic!("not impl")
            }
        }
    }
}

fn parse_algebraic_notation(alg_notation: &str) -> (u8, u8) {

    let from_file = alg_notation.chars().nth(0).unwrap();
    let from_rank = alg_notation.chars().nth(1).unwrap();
    let to_file = alg_notation.chars().nth(2).unwrap();
    let to_rank = alg_notation.chars().nth(3).unwrap();

    let from_square = map_to_square(from_file, from_rank).unwrap();
    let to_square = map_to_square(to_file, to_rank).unwrap();

    (from_square, to_square)
}

fn map_to_square(file: char, rank: char) -> Option<u8> {
    let file_index = match file {
        'a'..='h' => file as u8 - b'a',
        _ => return None, // Invalid file
    };
    let rank_index = match rank {
        '1'..='8' => rank as u8 - b'1',
        _ => return None, // Invalid rank
    };
    Some(rank_index * 8 + file_index)
}

pub struct UciEngine {
    board: Board,
    settings: String,
    debug: bool
}

impl UciEngine {
    
    pub fn new() -> UciEngine{
        UciEngine { board: Board::default(), settings: "".into(), debug: false }
    }

    pub fn execute(&mut self, message: UciMessage) -> Option<Vec<UciMessage>>{
        match message {
            UciMessage::Uci => {
                let mut res = vec![];
                res.push(UciEngine::uci_id());
                self.options().iter().for_each(|option| res.push(option.clone()));
                res.push(UciMessage::UciOk);
                Some(res)
            },
            UciMessage::Debug(on) => {
                self.debug = on;
                None
            },
            UciMessage::IsReady => {
                Some(vec![UciMessage::ReadyOk])
            },
            UciMessage::SetOption { name, value } => {
                panic!("options not implemented")
            },
            UciMessage::Register { later, name, code } => {
                todo!()
            },
            UciMessage::UciNewGame => {
                self.board = Board::default();
                None
            },
            UciMessage::Position { fen, moves } => {
                self.board = if let Some(fe) = fen { Board::from_fen(&fe) } else { Board::default() };
                for mv in moves{
                    self.board.make_move(mv);
                }
                None
            },
            UciMessage::Go { .. } => {
                let mv = self.best_move();
                let res = UciMessage::BestMove { best_move: mv, ponder: None };
                Some(vec![res])
            },
            UciMessage::Ponderhit => {
                panic!("not implemented")
            },
            UciMessage::Stop => {
                let mv = self.best_move();
                let res = UciMessage::BestMove { best_move: mv, ponder: None };
                Some(vec![res])
            },
            UciMessage::Quit => {
                panic!("quit is not for the engine")
            }
            _ => {
                panic!()
            }
        }
    }

    fn options(&self) -> Vec<UciMessage>{
        vec![] //no options currently implemented
    }

    fn uci_id() -> UciMessage{
        UciMessage::Id { name: UciEngine::name().into(), author: UciEngine::author().into() }
    }

    fn author() -> &'static str{
        "Jonathan Cederlund"
    }

    fn name() -> &'static str {
        "alfa-beta-bot"
    }

    fn best_move(&mut self) -> Move{
        let mut moves = self.board.get_possible_moves_turn();
        let mut rng = rand::thread_rng();

        moves.shuffle(&mut rng);
        if let Some(mv) = moves.first() {
            *mv
        }else {
            panic!("no leagal moves")
        }
    }
}
