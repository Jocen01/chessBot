use crate::{board::Board, constants, state::PiceBoards};

pub fn evaluate(board: &Board) -> i32{
    let white = sum_pice_values(&board.state.white);
    let black = sum_pice_values(&board.state.black);
    white - black
}

fn sum_pice_values(pice_board: &PiceBoards) -> i32{
    let mut value = 0;
    value += constants::PAWN_VALUE * pice_board.pawns.count_ones() as i32;
    value += constants::KNIGHT_VALUE * pice_board.knights.count_ones() as i32;
    let queens = pice_board.diagonal_sliders & pice_board.orthoganal_sliders;
    let rooks = queens ^ pice_board.orthoganal_sliders;
    let bishops = queens ^ pice_board.diagonal_sliders;
    value += constants::BISHOP_VALUE * bishops.count_ones() as i32;
    value += constants::ROOK_VALUE * rooks.count_ones() as i32;
    value += constants::QUEEN_VALUE * queens.count_ones() as i32;
    value
}