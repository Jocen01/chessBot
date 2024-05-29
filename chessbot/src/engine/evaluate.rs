use crate::{board::{color::Color, state::{PiceBoards, State}, Board}, constants, movegeneration::singlemove::{Move, MoveType}};

// cant use i32::MIN cause if negetet it overflows
pub const NEGATIVE_INF: i32 = i32::MIN + 10000;
pub const POSETIVE_INF: i32 = i32::MAX - 10000;

pub fn evaluate_turn(board: &Board) -> i32{
    match board.get_turn() {
        Color::White => evaluate_white(board),
        Color::Black => -evaluate_white(board)
    }
}

fn evaluate_white(board: &Board) -> i32{
    let (mut eval, _mg_phase, _eg_phase) = evaluate_pice_pos(board);
    eval += eval_past_pawns(&board.state);
    eval += isolated_pawns(&board.state);
    // eval += mobility_score(&board.state);
    eval += rooks_on_open_files(&board.state);
    eval += doubled_pawns(&board.state);
    // eval += king_endgame(&board.state, eval, _eg_phase);
    eval
}

fn evaluate_pice_pos(board: &Board) -> (i32, i32, i32){
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
    return ((mg_score * mg_phase + eg_score * eg_phase) / 24, mg_phase, eg_phase);
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
        let mut v = constants::PASSED_PAWNS_VALUE[(pos>>3) as usize];
        if constants::FILES_MASK[(pos & 0b111) as usize] & ((1<<pos)-1) & state.white.orthoganal_sliders != 0{
            v *= 7;
            v /= 5;
        }
        v
    }).sum();

    let black_pawns = get_set_bits(&state.black.pawns);
    let value_past_pawns_black: i32 = black_pawns.iter().filter(|&pos| {
        constants::PASTPAWN_BLACK_MASK[*pos as usize] & state.white.pawns == 0
    }).map(|pos| {
        let mut v = constants::PASSED_PAWNS_VALUE[(8 - (pos>>3)) as usize];
        if constants::FILES_MASK[(pos & 0b111) as usize] & (!((1<<pos)-1)) & state.black.orthoganal_sliders != 0{
            v *= 7;
            v /= 5;
        }
        v
    }).sum();

    value_past_pawns_white - value_past_pawns_black

}

#[allow(dead_code)]
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
                    30
                }else {
                    10
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

fn isolated_pawns(state: &State) -> i32{
    fn help(pawns: u64) -> i32{
        get_set_bits(&pawns).iter()
        .map(|pos| {
            if pos & 0b111 == 0 {
                constants::FILES_MASK[1]
            }else if pos & 0b111 == 0b111 {
                constants::FILES_MASK[6]
            }else{
                constants::FILES_MASK[((pos - 1) & 0b111) as usize] | constants::FILES_MASK[((pos + 1) & 0b111) as usize]
            }
        }).filter(|mask| mask & pawns == 0)
        .map(|_| -5).sum()
    }

    help(state.white.pawns) - help(state.black.pawns)
}

#[allow(dead_code)]
fn king_endgame(state: &State, eval: i32, eg_phase: i32) -> i32{
    fn help(own: &PiceBoards, opponent: &PiceBoards, eg_phase: i32) -> i32{
        let pos = own.king.trailing_zeros();
        let mut eval = 0;
        if pos < 56 && (own.king << 8) & opponent.pawns != 0{
            eval += 14 * eg_phase / 24;
        }
        eval
    }
    let king_distance = ((state.white.king & 0b111) + (state.black.king & 0b111)/2).max((state.white.king & 0b111000) + (state.black.king & 0b111000)/2) as i32;
    let distance_eval = eval * king_distance * (eg_phase - 12) / (150 * 24);
    help(&state.white, &state.black, eg_phase) - help(&state.black, &state.white, eg_phase) + distance_eval
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
    50
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
                ((captured.pice_type() as usize) * 100 - pice.pice_type() as usize) * 10_000
            }else {
                0
            }
        } else if mv.move_type() != MoveType::Normal {
            match mv.move_type() {
                MoveType::PromotionQueen => 9_000,
                MoveType::PromotionKnight => 8_000,
                MoveType::Castle => 7_000,
                MoveType::PromotionRook => 6_000,
                MoveType::PromotionBishop => 5_000,
                MoveType::Pessant => 4_000,
                MoveType::Pawndubblemove | MoveType::Normal => 0,
            }
        }else {
            0
        }
    });
    moves.reverse();
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, engine::evaluate::evaluate_white, movegeneration::singlemove::{Move, MoveType}};

    use super::evaluate_turn;

    #[test]
    fn same_eval_both_sides(){
        let fen1 = "r1bqk2r/2p1bppp/p1np1n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 w kq - 0 8";
        let fen2 = "rnbqr1k1/pppp1ppp/1b3n2/4p3/1P2P3/P1NP1N2/2P1BPPP/R1BQK2R b kq - 0 8";
        let board1 = Board::from_fen(&fen1);
        let board2 = Board::from_fen(&fen2);
        assert_eq!(evaluate_white(&board1), -evaluate_white(&board2));
    }

    #[test]
    fn rasonable_eval(){
        let fen = "2r1r1k1/4Bpp1/p1b5/5Q2/1p1pp3/1P5P/2Pq1PP1/2R1R1K1 w - - 0 35";
        let mut board = Board::from_fen(&fen);
        board.make_move(Move::new(4, 3, MoveType::Normal));
        board.make_move(Move::new(11, 18, MoveType::Normal));
        board.make_move(Move::new(52, 25, MoveType::Normal));
        board.make_move(Move::new(18, 25, MoveType::Normal));
        board.make_move(Move::new(2, 0, MoveType::Normal));
        assert!(evaluate_turn(&board) > 100);
    }

}