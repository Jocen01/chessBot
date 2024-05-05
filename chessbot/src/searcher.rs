use crate::{board::Board, evaluate, singlemove::Move, transposition_table::{TranspositionsFlag, TranspositionsTable}};
use rand::prelude::*;
use std::time::{Duration, Instant};

// cant use i32::MIN cause if negetet it overflows
const NEGATIVE_INF: i32 = i32::MIN + 1;
const POSETIVE_INF: i32 = i32::MAX - 1;

pub struct Searcher{
    traspos_table: TranspositionsTable,
    pub searches: u64,
    start_time: Instant,
    duration: Duration
}

impl Searcher {
    pub fn new(traspos_table_size: usize) -> Searcher{
        Searcher{
            traspos_table: TranspositionsTable::new(traspos_table_size),
            searches: 0,
            start_time: Instant::now(),
            duration: Duration::from_millis(200)
        }
    }

    pub fn search(&mut self, board: &mut Board, depth: usize) -> (Move, i32){
        let val = self.search_alpha_beta(board, NEGATIVE_INF, POSETIVE_INF, depth, 0);
        // let mv = self.traspos_table.get_best_move(board.get_zobrist_hash());
        // if let Some(mv) = mv {
        
        //     (mv, val)
        // }else {
        //     if let Some(mv) = board.get_possible_moves_turn().first() {
        //         (*mv, val)
        //     }else {
        //         panic!("no leagal moves, board \n{}", board);
        //     }
        // }
        val
    }

    pub fn iterative_deepening(&mut self, board: &mut Board) -> (Move, i32){
        // dont know if table needs clearing
        self.traspos_table.clear();
        self.searches = 0;

        self.start_time = Instant::now();
        let mut best_move = Move::null_move();
        let mut val = 0;
        for depth in 1..255{
            let (mv, val_depth) = self.search(board, depth);
            
            // return if searchtime has elapsed
            if self.start_time.elapsed() >= self.duration {
                break;
            }else {
                best_move = mv;
                val = val_depth;
            }
        }

        (best_move, val)
    }

    fn search_alpha_beta(&mut self, board: &mut Board, mut alpha: i32, beta: i32, depth: usize, ply: usize) -> (Move, i32){
        let mut flag = TranspositionsFlag::UpperBound;
        let zobrist = board.get_zobrist_hash();
        self.searches += 1;
        // lookup the position if it exists in the table
        if let Some(val) = self.traspos_table.lookup_eval(zobrist, depth, alpha, beta){
            if let Some(best) = self.traspos_table.get_best_move(zobrist) {
                return (best, val);
            }
            return (Move::null_move(),val);
        }
        if depth == 0{
            
            // let mut val = evaluate::evaluate_turn(board);

            let val = self.search_stable_pos(board, alpha, beta);
            self.traspos_table.record_entry(zobrist, depth, val, TranspositionsFlag::Exact, None);
            return (Move::null_move(),val);
        }
        let mut moves = board.get_possible_moves_turn();

        //random ordering for moves before ordering is implemented
        let mut rng = rand::thread_rng();
        moves.shuffle(&mut rng);

        // a first best move
        let mut best_move = if let Some(mv) = moves.first() {
            *mv
        }else{ Move::null_move() };
       

        for mv in moves{
            board.make_move(mv);
            let (_,mut val) = self.search_alpha_beta(board, -beta, -alpha, depth - 1, ply + 1);
            val = -val;
            board.undo_last_move();

            //branch can be pruned
            if val >= beta{
                self.traspos_table.record_entry(zobrist, depth, val, TranspositionsFlag::LowerBound, Some(mv));
                return (mv, beta);
            }

            // found a new best move
            if val > alpha{
                flag = TranspositionsFlag::Exact;
                alpha = val;
                best_move = mv;
            }
            // return if searchtime has elapsed
            if self.start_time.elapsed() >= self.duration {
                return (best_move, alpha);
            }
        }       
        //  record the position and the best move found
        self.traspos_table.record_entry(zobrist, depth, alpha, flag, Some(best_move));
        (best_move, alpha)
    }

    fn search_stable_pos(&mut self, board: &mut Board, mut alpha: i32, beta: i32) ->  i32{
        self.searches += 1;
            
        let mut val = evaluate::evaluate_turn(board);

        if val >= beta{
            return beta;
        }
        // alpha = val;
        alpha = alpha.max(val);
        //get captures only
        let mut moves = board.get_possible_captures_turn();

        //random ordering for moves before ordering is implemented
        let mut rng = rand::thread_rng();
        moves.shuffle(&mut rng);

        for mv in moves{
            board.make_move(mv);
            val = -self.search_stable_pos(board, -beta, -alpha);
            board.undo_last_move();

            //branch can be pruned
            if val >= beta{
                return beta;
            }

            // found a new best move
            alpha = alpha.max(val);

            // return if searchtime has elapsed
            if self.start_time.elapsed() >= self.duration {
                break;
            }
        }       
        alpha
    }

    #[allow(dead_code)]
    pub fn search_2(&mut self, board: &mut Board, depth: u32) -> (Move, i32){
        self.searches = 0;
        self.search_alpha_beta_2(board, i32::MIN, i32::MAX, depth, board.is_white())
    }
    
    #[allow(dead_code)]
    fn search_alpha_beta_2(&mut self, board: &mut Board, mut alfa: i32, mut beta: i32, depth: u32, maximizing_player: bool) -> (Move, i32){
        self.searches += 1;
        if depth == 0{
            if let Some(mv) = board.moves.last() {
                (*mv, evaluate::evaluate_white(board))
                
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
                let new_value = self.search_alpha_beta_2(board, alfa, beta, depth - 1, !maximizing_player);
                board.undo_last_move();
                if new_value.1 > value.1{
                    value = (mv,new_value.1);
                }
                if value.1 >= beta { break; }
                alfa = alfa.max(value.1);
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
                let new_value = self.search_alpha_beta_2(board, alfa, beta, depth - 1, !maximizing_player);
                board.undo_last_move();
                if new_value.1 < value.1{
                    value = (mv,new_value.1);
                }
                if value.1 <= alfa { break; }
                beta = beta.min(value.1)
            } 
            if value.0.is_null_move(){
                (first, value.1)
            }else {
                value
            }
        }
    }
}

