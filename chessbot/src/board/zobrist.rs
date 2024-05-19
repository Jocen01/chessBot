use crate::constants;

use super::{color::Color, pice::{Pice, PiceType}, state::State};



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