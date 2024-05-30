use crate::board::{color::Color, Board};

use super::singlemove::{Move, MoveType};



pub struct MoveOrder{
    killer_moves: [[u32; 2]; 64],
    history: [[[usize; 64];64];2]
}

impl MoveOrder {
    pub fn default() -> MoveOrder{
        MoveOrder { killer_moves: [[0;2];64], history: [[[0;64];64];2] }
    }

    pub fn add_killer(&mut self, mv: &Move, ply: usize){
        if ply < 64{
            self.killer_moves[ply][1] = self.killer_moves[ply][0];
            self.killer_moves[ply][0] = mv.get_hash();
        }
    }

    pub fn add_history(&mut self, mv: &Move, depth: usize, white: bool){
        let i = if white { 0 } else { 1 };
        self.history[i][mv.from() as usize][mv.to() as usize] += depth;
        if self.history[i][mv.from() as usize][mv.to() as usize] > 90_000{
            self.history[i][mv.from() as usize][mv.to() as usize] /= 2;
        }
    }

    pub fn sort_moves(&self, moves: &mut Vec<Move>, board: &Board, ply: usize){
        moves.sort_by_cached_key(|mv| {
            let i = if board.get_turn() == Color::White { 0 } else { 1 };
            let mut weight = 1;
            if let Some(captured) = board.get_pice_pos(mv.to()) {
                if let Some(pice) = board.get_pice_pos(mv.from()) {
                    weight += ((captured.pice_type() as usize) * 100 - pice.pice_type() as usize) * 1_000_000
                }
            }else if board.state.passant_at(mv.to()) {
                if let Some(pice) = board.get_pice_pos(mv.from()) {
                    weight += (100 - pice.pice_type() as usize) * 1_000_000
                }              
            }else if ply < 64 && (self.killer_moves[ply][0] == mv.get_hash() || self.killer_moves[ply][1] == mv.get_hash()){
                weight += 100_000;
            }
            weight += self.history[i][mv.from() as usize][mv.to() as usize];
            match mv.move_type() {
                MoveType::PromotionQueen => weight *= 7,
                MoveType::PromotionKnight =>weight *= 6,
                MoveType::Castle => weight *= 5,
                MoveType::PromotionRook => weight *= 4,
                MoveType::PromotionBishop => weight *= 3,
                _ => {}
            }
            weight
        });
        moves.reverse();
    }

    pub fn clear(&mut self){
        for i in 0..64{
            self.killer_moves[i][0] = 0;
            self.killer_moves[i][1] = 0;
        }
        for i in 0..2{
            for j in 0..64{
                for k in 0..64{
                    self.history[i][j][k] = 0;
                }
            }
        }
    }
}