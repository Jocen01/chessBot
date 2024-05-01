use std::fmt;

use crate::state::CastleRights;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MoveType {
    Normal = 0,
    Castle = 1,
    Pessant = 2,
    PromotionQueen = 3,
    PromotionRook = 4,
    PromotionBishop = 5,
    PromotionKnight = 6,
    Pawndubblemove = 7,
}

impl MoveType {
    pub fn iter_promotions() -> Vec<MoveType>{
        vec![MoveType::PromotionQueen, MoveType::PromotionRook, MoveType::PromotionBishop, MoveType::PromotionKnight]
    }
}

#[derive(Clone, Copy)]
pub struct Move{
    value: u32, // FFFFCCCCfffffftttttt = Flag, Castle rights removed, from, to
    captured: Option<usize>
}

impl Move {
    pub fn new(from: u8, to: u8, move_type: MoveType) -> Move {
        Move {value: (to as u32)  | (from as u32) << 6 | (move_type as u32) << 16, captured: None}
    }

    pub fn null_move() -> Move{
        Move { value: 0, captured: None }
    }

    pub fn is_null_move(&self) -> bool{
        self.value == 0
    }

    pub fn long_algebraic_notation(&self) -> String{
        let pos: Vec<String> = vec![self.from(), self.to()].iter().map(|pos| {
            Move::square_to_coordinates(*pos)
        }).collect();
        let mut res = pos.concat();
        match self.move_type() {
            MoveType::PromotionQueen => {
                res.push('q')
            },
            MoveType::PromotionRook => {
                res.push('r')
            },
            MoveType::PromotionBishop => {
                res.push('b')
            },
            MoveType::PromotionKnight => {
                res.push('n')
            },
            _ => {

            }
        }
        res
    }

    fn square_to_coordinates(square: u8) -> String {
        if square < 64 {
            let rank = (square / 8) + b'1';
            let file = (square % 8) + b'a';
            let mut result = String::new();
            result.push(file as char);
            result.push(rank as char);
            result
        } else {
            panic!("not a valid square");
        }
    }

    pub fn capture(&mut self, pice: usize){
        self.captured = Some(pice);
    }

    pub fn from(&self) -> u8{
        ((self.value >> 6) & 0b111111) as u8
    }

    pub fn to(&self) -> u8{
        (self.value & 0b111111) as u8
    }

    pub fn get_captured(&self) -> Option<usize>{
        self.captured
    }

    pub fn move_type(&self) -> MoveType{
        match self.value >> 16 {
            i if i == MoveType::Normal as u32 => MoveType::Normal,
            i if i == MoveType::Castle as u32 => MoveType::Castle,
            i if i == MoveType::Pessant as u32 => MoveType::Pessant,
            i if i == MoveType::PromotionQueen as u32 => MoveType::PromotionQueen,
            i if i == MoveType::PromotionRook as u32 => MoveType::PromotionRook,
            i if i == MoveType::PromotionBishop as u32 => MoveType::PromotionBishop,
            i if i == MoveType::PromotionKnight as u32 => MoveType::PromotionKnight,
            i if i == MoveType::Pawndubblemove as u32 => MoveType::Pawndubblemove,
            i => panic!("{} is not a valid move type flag", i)
        }
    }

    pub fn remove_casle_right(&mut self, castle_rights: CastleRights){
        self.value |= 1<<(12 + (castle_rights as u8));
    }

    pub fn get_removed_castlerights(&self) -> Option<Vec<CastleRights>>{
        if (self.value >> 12) & 0b1111 != 0{
            let mut res = vec![];
            for castle_right in CastleRights::iter(){
                if self.value & 1<<(12 + (castle_right as u8)) != 0{
                    res.push(castle_right);
                }
            }
            Some(res)
        }else {
            None
        }
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Move: {{ move_type: {:?}, from: {}, to: {} }}", self.move_type(), self.from(), self.to())
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Move: {{ move_type: {:?}, from: {}, to: {} }}", self.move_type(), self.from(), self.to())
    }
}