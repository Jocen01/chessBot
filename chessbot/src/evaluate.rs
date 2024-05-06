use crate::{board::Board, constants, state::{PiceBoards, State}, Color};

#[allow(dead_code)]
pub fn evaluate_white_old(board: &Board) -> i32{
    let white = sum_pice_values(&board.state.white);
    let black = sum_pice_values(&board.state.black);
    white - black
}

pub fn evaluate_turn(board: &Board) -> i32{
    match board.get_turn() {
        Color::White => evaluate_white(board),
        Color::Black => -evaluate_white(board)
    }
}

#[allow(dead_code)]
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

pub fn evaluate_white(board: &Board) -> i32{
    let mut eval = evaluate_pice_pos(board);
    eval += eval_past_pawns(&board.state);
    eval
}

pub fn evaluate_pice_pos(board: &Board) -> i32{
    let mut mg: [i32;2] = [0,0];
    let mut eg: [i32;2] = [0,0];
    let mut game_phase = 0;

    /* evaluate each piece */
    for sq in 0..64 {
        if let Some(pice) = board.get_pice_pos(sq) {
            let color = pice.color().to_0_1();
            let p = (pice.pice_type() as usize) - 1; 
            mg[color] += constants::MG_TABLE[p*2+color][sq as usize];
            eg[color] += constants::EG_TABLE[p*2+color][sq as usize];
            game_phase += constants::GAMEPHASE_INC[p*2 + color];
        }
    }

    /* tapered eval */
    let mg_score = mg[0] - mg[1];
    let eg_score = eg[0] - eg[1];
    let mut mg_phase = game_phase;
    if mg_phase > 24 {mg_phase = 24;}; /* in case of early promotion */
    let eg_phase = 24 - mg_phase;
    return (mg_score * mg_phase + eg_score * eg_phase) / 24;
}

fn get_set_bits(pos: &u64) -> Vec<u8>{
    if *pos == ((1 as u64)<<63){
        vec![63]
    }else {
        let mut i = pos.clone();
        let mut res = vec![];
        let mut idx = 0;
        while i!= 0 {
            let t = i.trailing_zeros() as u8;
            res.push(idx + t);
            idx += t + 1;
            i >>= t+1
        }
        res
    } 
}

fn eval_past_pawns(state: &State) -> i32{
    let white_pawns = get_set_bits(&state.white.pawns);
    let value_past_pawns_white: i32 = white_pawns.iter().filter(|&pos| {
        constants::PASTPAWN_WHITE_MASK[*pos as usize] & state.black.pawns == 0
    }).map(|pos| {
        constants::PASSED_PAWNS_VALUE[(pos>>3) as usize]
    }).sum();

    let black_pawns = get_set_bits(&state.black.pawns);
    let value_past_pawns_black: i32 = black_pawns.iter().filter(|&pos| {
        constants::PASTPAWN_BLACK_MASK[*pos as usize] & state.white.pawns == 0
    }).map(|pos| {
        constants::PASSED_PAWNS_VALUE[(8 - (pos>>3)) as usize]
    }).sum();

    value_past_pawns_white - value_past_pawns_black

}

fn mobility_score(state: &State) -> i32{
    0
}

fn rooks_on_open_files(state: &State) -> i32{
    0
}