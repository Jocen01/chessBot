use crate::{board::Board, constants, singlemove::Move, state::{PiceBoards, State}, Color};

// cant use i32::MIN cause if negetet it overflows
pub const NEGATIVE_INF: i32 = i32::MIN + 10000;
pub const POSETIVE_INF: i32 = i32::MAX - 10000;

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

fn evaluate_white(board: &Board) -> i32{
    let mut eval = evaluate_pice_pos(board);
    eval += eval_past_pawns(&board.state);
    eval += mobility_score(&board.state);
    eval += rooks_on_open_files(&board.state);
    eval += doubled_pawns(&board.state);
    eval
}

fn evaluate_pice_pos(board: &Board) -> i32{
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
    ((state.white.capture.count_ones() as i32) - (state.black.capture.count_ones() as i32)) * 5
}

fn rooks_on_open_files(state: &State) -> i32{
    fn help(own: &PiceBoards, opponent: &PiceBoards) -> i32{
        let rooks = get_set_bits(&(own.orthoganal_sliders & (own.orthoganal_sliders ^ own.diagonal_sliders)));
        rooks.iter().map(|pos| {
            let mask = constants::FILES_MASK[*pos as usize & 0b111];
            if mask & own.pawns == 0{
                if mask & opponent.pawns == 0{
                    20
                }else {
                    8
                }
            }else {
                0
            }
        }).sum()
    }
    help(&state.white, &state.black) - help(&state.black, &state.white)
}

fn doubled_pawns(state: &State) -> i32{
    let mut eval = 0;
    for mask in constants::FILES_MASK{
        if (state.white.pawns & mask).count_ones() > 1{
            eval -= 50;
        }
        if (state.black.pawns & mask).count_ones() > 1{
            eval += 50;
        }
    }
    eval
}

#[allow(dead_code)]
fn bishops_on_open_diagonals(state: &State) -> i32{
    fn help(own: &PiceBoards, opponent: &PiceBoards) -> i32{
        let bishops = get_set_bits(&(own.diagonal_sliders & (own.orthoganal_sliders ^ own.diagonal_sliders)));
        bishops.iter().map(|pos| {
            let mask = constants::FILES_MASK[*pos as usize & 0b111];
            if mask & own.pawns == 0{
                if mask & opponent.pawns == 0{
                    20
                }else {
                    8
                }
            }else {
                0
            }
        }).sum()
    }
    help(&state.white, &state.black) - help(&state.black, &state.white)
}

pub fn draw_by_repetition() -> i32{
    -50
}

pub fn mate_ajusted_score(ply: usize) -> i32{
    NEGATIVE_INF + 10 + (ply as i32)
}

pub fn is_mate_score(score: i32) -> bool{
    score.abs() > 100000
}

pub fn to_mate(score: i32) -> i32{
    let sign = if score < 0 { -1 } else { 1 };
    let sign_adjusted = if score < 0 { score } else { -score };
    ((sign_adjusted - NEGATIVE_INF - 10) * sign) / 2
}

pub fn sort_moves(moves: &mut Vec<Move>, board: &Board){
    moves.sort_by_cached_key(|mv| {
        if let Some(captured) = board.get_pice_pos(mv.to()) {
            if let Some(pice) = board.get_pice_pos(mv.from()) {
                (captured.pice_type() as usize) * 100 - pice.pice_type() as usize
            }else {
                0
            }
        }else {
            0
        }
    });
    moves.reverse();
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, evaluate::evaluate_white};

    #[test]
    fn same_eval_both_sides(){
        let fen1 = "r1bqk2r/2p1bppp/p1np1n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 w kq - 0 8";
        let fen2 = "rnbqr1k1/pppp1ppp/1b3n2/4p3/1P2P3/P1NP1N2/2P1BPPP/R1BQK2R b kq - 0 8";
        let board1 = Board::from_fen(&fen1);
        let board2 = Board::from_fen(&fen2);
        assert_eq!(evaluate_white(&board1), -evaluate_white(&board2));
    }
}