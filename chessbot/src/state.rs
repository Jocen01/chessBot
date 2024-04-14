use crate::Color;

pub struct State{
    pub white_can_move: u64,
    pub black_can_move: u64,
    pub white_pices_bitboard: u64,
    pub black_pices_bitboard: u64,
    pub passant: u64,
    pub casle_rights: u8,
    pub white_king: u8,
    pub black_king: u8,

}

impl State {
    pub fn new(
            white_can_move: u64, 
            black_can_move: u64, 
            white_pices_bitboard: u64, 
            black_pices_bitboard: u64, 
            passant: u64, 
            casle_rights: u8, 
            white_king: u8, 
            black_king: u8
        ) -> State{
        State {
            white_can_move, 
            black_can_move, 
            white_pices_bitboard, 
            black_pices_bitboard, 
            passant,
            casle_rights,
            white_king,
            black_king            
        }
    }

    pub fn default() -> State{
        State { 
            white_can_move: 0, 
            black_can_move: 0, 
            white_pices_bitboard: 0, 
            black_pices_bitboard: 0 , 
            passant: 0, 
            casle_rights: 0b1111,
            white_king: 4,
            black_king: 60
        }
    }

    pub fn pice_at(&self, pos: u8) -> bool{
        (self.white_pices_bitboard | self.black_pices_bitboard) & (1<<pos) != 0
    }

    pub fn white_at(&self, pos: u8) -> bool{
        self.white_pices_bitboard & (1<<pos) != 0
    }

    pub fn black_at(&self, pos: u8) -> bool{
        self.black_pices_bitboard & (1<<pos) != 0
    }

    pub fn color_at(&self, pos: u8, color: Color) -> bool{
        match color {
            Color::White => self.white_at(pos),
            Color::Black => self.black_at(pos)
        }
    }

    pub fn opposite_color_at(&self, pos: u8, color: Color) -> bool{
        match color {
            Color::White => self.black_at(pos),
            Color::Black => self.white_at(pos)
        }
    }

    pub fn reset_pices_bitboard(&mut self){
        self.white_pices_bitboard = 0;
        self.black_pices_bitboard = 0;
    }

    pub fn reset_can_move(&mut self){
        self.white_can_move = 0;
        self.black_can_move = 0;
    }

    pub fn passant_at(&self, pos: u8) -> bool{
        self.passant & (1<<pos) != 0
    }
}