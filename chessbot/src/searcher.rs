use crate::{board::Board, evaluate, singlemove::Move};
use rand::prelude::*;


pub fn search(board: &mut Board, depth: u32) -> (Move, i32){
    search_alpha_beta(board, i32::MIN, i32::MAX, depth, board.is_white())
}

fn search_alpha_beta(board: &mut Board, mut alfa: i32, mut beta: i32, depth: u32, maximizing_player: bool) -> (Move, i32){

    if depth == 0{
        if let Some(mv) = board.moves.last() {
            (*mv, evaluate::evaluate(board))
            
        }else {
            panic!("cant search start pos at depth 0")
        }
    } else if maximizing_player {

        let mut moves = board.get_possible_moves_turn();
        let mut rng = rand::thread_rng();

        moves.shuffle(&mut rng);
        let first = if let Some(mv) = moves.first() {
            mv.clone()
        }else { Move::null_move() };

        let mut value = (Move::null_move(), i32::MIN);
        for mv in moves{
            board.make_move(mv);
            let new_value = search_alpha_beta(board, alfa, beta, depth - 1, !maximizing_player);
            board.undo_last_move();
            if new_value.1 > value.1{
                value = (mv,new_value.1);
            }
            if value.1 > beta { break; }
            alfa = alfa.max(value.1)
        } 

        if value.0.is_null_move(){
            (first, value.1)
        }else {
            value
        }
    } else {
        let mut moves = board.get_possible_moves_turn();
        let mut rng = rand::thread_rng();

        moves.shuffle(&mut rng);

        let first = if let Some(mv) = moves.first() {
            mv.clone()
        }else { Move::null_move() };

        let mut value = (Move::null_move(), i32::MAX);
        for mv in moves{
            board.make_move(mv);
            let new_value = search_alpha_beta(board, alfa, beta, depth - 1, !maximizing_player);
            board.undo_last_move();
            if new_value.1 < value.1{
                value = (mv,new_value.1);
            }
            if value.1 < alfa { break; }
            beta = beta.min(value.1)
        } 
        if value.0.is_null_move(){
            (first, value.1)
        }else {
            value
        }
    }
}
