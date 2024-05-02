use crate::{constants, pice::Pice, Color, PiceType};
// use strum::IntoEnumIterator; // 0.17.1
// use strum_macros::EnumIter; // 0.17.1

// can capture bitmap       2x
// king bitmap / pos        2x
// sliding pices bitmap     2x
// pawns bitmap             2x
// pices bitmap             2x
// passant u8 / bitmap
// castle rights

#[derive(Debug, Clone, Copy)]
pub enum CastleRights {
    WhiteQueenside = 0,
    WhiteKingside = 1,
    BlackQueenside = 2,
    BlackKingside = 3,
}

impl CastleRights {
    pub fn iter() -> Vec<CastleRights>{
        vec![CastleRights::WhiteQueenside, CastleRights::WhiteKingside, CastleRights::BlackQueenside, CastleRights::BlackKingside]
    }

    pub fn str_to_casle_rights(s: &str) -> u8{
        let itr =  vec![
                            ('K',CastleRights::WhiteKingside),
                            ('Q',CastleRights::WhiteQueenside),
                            ('k',CastleRights::BlackKingside),
                            ('q',CastleRights::BlackQueenside)
        ];
        let mut rights = 0;
        for (c, castle_right) in itr{
            if s.contains(c){
                rights |= 1<<(castle_right as u8);
            }
        }
        rights
    }
}

#[derive(Debug)]
pub struct PiceBoards{
    pub capture: u64,
    pub king: u64,
    pub diagonal_sliders: u64,
    pub orthoganal_sliders: u64,
    pub pawns: u64,
    pub knights: u64
}

impl PiceBoards {
    #[allow(dead_code)]
    pub fn new(capture: u64, king: u64, diagonal_sliders: u64, orthoganal_sliders: u64, pawns: u64, horses: u64) -> PiceBoards{
        PiceBoards {capture, king, diagonal_sliders, orthoganal_sliders, pawns, knights: horses}
    }

    #[allow(dead_code)]
    pub fn default(color: Color) -> PiceBoards{
        match color {
            Color::White => {
                PiceBoards{
                    capture: (0b11111111)<<16,
                    king: 0b10000,
                    diagonal_sliders: 0b101100,
                    orthoganal_sliders:0b10000001,
                    pawns: (0b11111111)<<8,
                    knights:0b1000010
                }
            },
            Color::Black => {
                PiceBoards{
                    capture: (0b11111111)<<40,
                    king: 0b10000<<56,
                    diagonal_sliders: 0b101100<<56,
                    orthoganal_sliders:0b10000001<<56,
                    pawns: (0b11111111)<<48,
                    knights:0b1000010<<56
                }
            }
        }
    }

    pub fn empty() -> PiceBoards{
        PiceBoards { capture: 0, king: 0, diagonal_sliders: 0, orthoganal_sliders: 0, pawns: 0, knights: 0 }
    }

    pub fn from_pices(pices: &Vec<&Pice>) -> PiceBoards{
        let mut board = PiceBoards::empty();
        pices.iter().for_each(|pice| {
            match pice.pice_type() {
                PiceType::King => board.king = 1<<pice.pos,
                PiceType::Queen => {
                    board.diagonal_sliders |= 1<<pice.pos; 
                    board.orthoganal_sliders|=1<<pice.pos
                },
                PiceType::Rook => board.orthoganal_sliders|=1<<pice.pos,
                PiceType::Bishop => board.diagonal_sliders |= 1<<pice.pos,
                PiceType::Knight => board.knights |= 1<<pice.pos,
                PiceType::Pawn => board.pawns |= 1<< pice.pos
            }
        });
        assert!(board.pawns.count_ones() <= 8);
        board
    }

    pub fn pice_at(&self, pos: u8) -> bool{
        (self.king | self.diagonal_sliders | self.orthoganal_sliders | self.pawns | self.knights) & (1<<pos) != 0
    }

    pub fn bitmap_all(&self) -> u64{
        self.king | self.diagonal_sliders | self.orthoganal_sliders | self.pawns | self.knights
    }

    pub fn move_pice(&mut self, from: u8, to: u8, pice: &Pice){
        match pice.pice_type() {
            PiceType::King => self.king = 1<<to,
            PiceType::Queen => {
                self.diagonal_sliders ^= 1<<from; 
                self.diagonal_sliders |= 1<<to; 
                self.orthoganal_sliders^=1<<from;
                self.orthoganal_sliders|=1<<to;
            },
            PiceType::Rook => {
                self.orthoganal_sliders^=1<<from;
                self.orthoganal_sliders|=1<<to;
            },
            PiceType::Bishop => {
                self.diagonal_sliders^=1<<from;
                self.diagonal_sliders|=1<<to;
            },
            PiceType::Knight => {
                self.knights^=1<<from;
                self.knights|=1<<to;
            },
            PiceType::Pawn => {
                self.pawns^=1<<from;
                self.pawns|=1<<to;
            }
        }
    }

    pub fn remove_pice(&mut self, pos: u8, pice: &Pice){
        match pice.pice_type() {
            PiceType::King => panic!("king can't be removed"),
            PiceType::Queen => {
                self.diagonal_sliders ^= 1<<pos; 
                self.orthoganal_sliders^=1<<pos;
            },
            PiceType::Rook => {
                self.orthoganal_sliders^=1<<pos;
            },
            PiceType::Bishop => {
                self.diagonal_sliders^=1<<pos;
            },
            PiceType::Knight => {
                self.knights^=1<<pos;
            },
            PiceType::Pawn => {
                self.pawns^=1<<pos;
            }
        }
    }

    pub fn reinstate_pice(&mut self, pos: u8, pice: &Pice){
        match pice.pice_type() {
            PiceType::King => panic!("king can't be reinstated since it cant be removed'"),
            PiceType::Queen => {
                self.diagonal_sliders |= 1<<pos; 
                self.orthoganal_sliders|=1<<pos;
            },
            PiceType::Rook => {
                self.orthoganal_sliders|=1<<pos;
            },
            PiceType::Bishop => {
                self.diagonal_sliders|=1<<pos;
            },
            PiceType::Knight => {
                self.knights|=1<<pos;
            },
            PiceType::Pawn => {
                self.pawns|=1<<pos;
            }
        }
    }

}

#[derive(Debug)]
pub struct Zobrist(u64);

impl Zobrist {
    pub fn from_pices(pices: &Vec<Pice>, state: &State, turn: Color) -> Zobrist{
        let mut zob = 0;
        zob ^= state.passant;
        zob ^= constants::ZOBRIST_CASTLE_RIGHTS[state.castle_rights as usize];
        pices.iter().filter(|pice| !pice.is_captured()).for_each(|pice| {
            zob ^= Zobrist::rand_value(&pice);
        });
        match turn {
            Color::White => {
                zob ^= constants::ZOBRIST_TURN_COLOR[0];
            },
            Color::Black => {
                zob ^= constants::ZOBRIST_TURN_COLOR[1];
            }
        }

        Zobrist(zob)
    }

    #[allow(dead_code)]
    pub fn default() -> Zobrist{
        let mut zob = 0;
        for i in 8..16{
            zob ^= constants::ZOBRIST_WHITE_PAWN[i];
        }
        for i in 48..56{
            zob ^= constants::ZOBRIST_BLACK_PAWN[i];
        }
        zob ^= constants::ZOBRIST_WHITE_ROOK[0];
        zob ^= constants::ZOBRIST_WHITE_KNIGHT[1];
        zob ^= constants::ZOBRIST_WHITE_BISHOP[2];
        zob ^= constants::ZOBRIST_WHITE_QUEEN[3];
        zob ^= constants::ZOBRIST_WHITE_KING[4];
        zob ^= constants::ZOBRIST_WHITE_BISHOP[5];
        zob ^= constants::ZOBRIST_WHITE_KNIGHT[6];
        zob ^= constants::ZOBRIST_WHITE_ROOK[7];
        zob ^= constants::ZOBRIST_BLACK_ROOK[56];
        zob ^= constants::ZOBRIST_BLACK_KNIGHT[57];
        zob ^= constants::ZOBRIST_BLACK_BISHOP[58];
        zob ^= constants::ZOBRIST_BLACK_QUEEN[59];
        zob ^= constants::ZOBRIST_BLACK_KING[60];
        zob ^= constants::ZOBRIST_BLACK_BISHOP[61];
        zob ^= constants::ZOBRIST_BLACK_KNIGHT[62];
        zob ^= constants::ZOBRIST_BLACK_ROOK[63];
        
        Zobrist(zob)
    }

    fn rand_value(pice: &Pice) -> u64{
        match pice.color() {
            Color::White => {
                match pice.pice_type() {
                    PiceType::Pawn => constants::ZOBRIST_WHITE_PAWN[pice.pos as usize],
                    PiceType::Knight => constants::ZOBRIST_WHITE_KNIGHT[pice.pos as usize],
                    PiceType::Bishop => constants::ZOBRIST_WHITE_BISHOP[pice.pos as usize],
                    PiceType::Rook => constants::ZOBRIST_WHITE_ROOK[pice.pos as usize],
                    PiceType::Queen => constants::ZOBRIST_WHITE_QUEEN[pice.pos as usize],
                    PiceType::King => constants::ZOBRIST_WHITE_KING[pice.pos as usize],
                }
            },
            Color::Black => {
                match pice.pice_type() {
                    PiceType::Pawn => constants::ZOBRIST_BLACK_PAWN[pice.pos as usize],
                    PiceType::Knight => constants::ZOBRIST_BLACK_KNIGHT[pice.pos as usize],
                    PiceType::Bishop => constants::ZOBRIST_BLACK_BISHOP[pice.pos as usize],
                    PiceType::Rook => constants::ZOBRIST_BLACK_ROOK[pice.pos as usize],
                    PiceType::Queen => constants::ZOBRIST_BLACK_QUEEN[pice.pos as usize],
                    PiceType::King => constants::ZOBRIST_BLACK_KING[pice.pos as usize],
                }
            }
        }
    }

    pub fn get(&self) -> u64{
        self.0
    }
}

#[derive(Debug)]
pub struct State{
    pub white: PiceBoards,
    pub black: PiceBoards,
    pub passant: u64,
    pub castle_rights: u8,
}

impl State {
    pub fn new(
            white: PiceBoards,
            black: PiceBoards, 
            passant: u64, 
            casle_rights: u8,
        ) -> State{
        State {
            white,
            black,
            passant,
            castle_rights: casle_rights,
        }
    }

    #[allow(dead_code)]
    pub fn default() -> State{
        State { 
            white: PiceBoards::default(Color::White), 
            black: PiceBoards::default(Color::Black), 
            passant: 0, 
            castle_rights: 0b1111,
        }
    }

    pub fn from_pices(pices: &Vec<Pice>, passant: u64, casle_rights: u8) -> State{
        let white: Vec<&Pice> = pices.iter().filter(|pice| pice.color() == Color::White).collect();
        let black: Vec<&Pice> = pices.iter().filter(|pice| pice.color() == Color::Black).collect();
        State::new(
            PiceBoards::from_pices(&white), 
            PiceBoards::from_pices(&black), 
            passant, 
            casle_rights,
        )
    }

    pub fn pice_at(&self, pos: u8) -> bool{
        self.white.pice_at(pos) || self.black.pice_at(pos)
    }

    pub fn white_at(&self, pos: u8) -> bool{
        self.white.pice_at(pos)
    }

    pub fn black_at(&self, pos: u8) -> bool{
        self.black.pice_at(pos)
    }

    pub fn opposite_color_at(&self, pos: u8, color: Color) -> bool{
        match color {
            Color::White => self.black_at(pos),
            Color::Black => self.white_at(pos)
        }
    }

    pub fn reset_can_capture(&mut self, color: Color, pices: &Vec<Pice>){
        match color {
            Color::White => {
                self.white.capture = 0;
                pices.iter().filter(|pice| pice.color() == color && !pice.is_captured()).for_each(|pice| {
                    self.white.capture |= pice.moves;
                })
            },
            Color::Black => {
                self.black.capture = 0;
                pices.iter().filter(|pice| pice.color() == color && !pice.is_captured()).for_each(|pice| {
                    self.black.capture |= pice.moves;
                })
            }
        }
    }

    pub fn passant_at(&self, pos: u8) -> bool{
        self.passant & (1<<pos) != 0
    }

    pub fn in_check(&self, color: Color) -> bool{
        match color {
            Color::White => self.white.king & self.black.capture != 0,
            Color::Black => self.black.king & self.white.capture != 0
        }
    }

    pub fn piceboards(&self, color: Color) -> &PiceBoards{
        match color {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }

    pub fn casle_right(&self, side: CastleRights) -> bool{
        self.castle_rights & (1 << (side as u8)) != 0
    }

    pub fn remove_casle_right(&mut self, side: CastleRights) -> bool{
        let mask = 1 << (side as u8);
        if self.castle_rights & mask != 0{
            self.castle_rights &= !mask;
            true
        }else {
            false
        }
    }

    pub fn reinstate_casle_right(&mut self, side: CastleRights){
        self.castle_rights |= 1<< (side as u8);
    }

    pub fn move_pice(&mut self, from: u8, to: u8, pice: &Pice){
        match pice.color() {
            Color::White => self.white.move_pice(from, to, pice),
            Color::Black => self.black.move_pice(from, to, pice),
        }
    }

    pub fn remove_pice(&mut self, pos: u8, pice: &Pice){
        match pice.color() {
            Color::White => self.white.remove_pice(pos, pice),
            Color::Black => self.black.remove_pice(pos, pice),
        }
    }

    pub fn reinstate_pice(&mut self, pos: u8, pice: &Pice){
        match pice.color() {
            Color::White => self.white.reinstate_pice(pos, pice),
            Color::Black => self.black.reinstate_pice(pos, pice),
        }
    }
}