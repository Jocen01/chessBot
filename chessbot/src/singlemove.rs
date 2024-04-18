#[derive(Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct Move{
    value: u16, // FFFFfffffftttttt = Flag, from, to
    captured: Option<usize>
}

impl Move {
        pub fn new(from: u8, to: u8, move_type: MoveType) -> Move {
        Move {value: (to as u16)  | (from as u16) << 6 | (move_type as u16) << 12, captured: None}
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
        match self.value >> 12 {
            i if i == MoveType::Normal as u16 => MoveType::Normal,
            i if i == MoveType::Castle as u16 => MoveType::Castle,
            i if i == MoveType::Pessant as u16 => MoveType::Pessant,
            i if i == MoveType::PromotionQueen as u16 => MoveType::PromotionQueen,
            i if i == MoveType::PromotionRook as u16 => MoveType::PromotionRook,
            i if i == MoveType::PromotionBishop as u16 => MoveType::PromotionBishop,
            i if i == MoveType::PromotionKnight as u16 => MoveType::PromotionKnight,
            i if i == MoveType::Pawndubblemove as u16 => MoveType::Pawndubblemove,
            i => panic!("{} is not a valid move type flag", i)
        }
    }
}
