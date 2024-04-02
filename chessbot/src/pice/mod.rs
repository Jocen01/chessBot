use crate::{board::Board, constants, Color, PiceType};

#[derive(Debug)]
pub struct Pice{
    pub typ: u8,
    pub pos: u8,
    pub moves: u64,
    pinned: bool,
}

impl Pice {
    pub fn new(pice: PiceType, color: Color, pos: u8) -> Pice{
        Pice { typ: pice as u8 | color as u8, pos: pos, moves: 0, pinned: false }
    }

    pub fn from_char(c: char, pos: u8) -> Pice{
        let color = Color::from_char(c);
        let pice = PiceType::from_char(c);
        Pice::new(pice, color, pos)
    }

    pub fn char(&self) -> String{
        let mut c: String = PiceType::char(self.typ).into();
        if self.typ & 8 == 8{
            c = c.to_uppercase();
        }
        c
    }

    pub fn pice_type(&self) -> PiceType{
        PiceType::_type(self.typ)
    }

    pub fn color(&self) -> Color{
        Color::from_int(self.typ)
    }

    pub fn update_moves(&mut self, board: &Board ) {
        match PiceType::_type(self.typ) {
            PiceType::King => self.update_moves_king(&board),
            PiceType::Queen => self.update_moves_queen(&board),
            PiceType::Rook => self.update_moves_rook(&board),
            PiceType::Bishop => self.update_moves_bishop(&board),
            PiceType::Knight => self.update_moves_knight(),
            PiceType::Pawn => self.update_moves_pawn(&board),
        }
    }

    fn update_moves_king(&mut self, board: &Board ) {
        todo!()
    }

    fn update_moves_queen(&mut self, board: &Board ) {
        todo!()
    }

    fn update_moves_rook(&mut self, board: &Board ) {
        todo!()
    }

    fn update_moves_bishop(&mut self, board: &Board ) {
        todo!()
    }

    fn update_moves_knight(&mut self) {
        self.moves = constants::HORSE_BIT_MOVES[self.pos as usize];
    }

    fn update_moves_pawn(&mut self, board: &Board ) {
        todo!()
    }
}