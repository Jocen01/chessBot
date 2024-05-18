use crate::{board::Board, evaluate::{self, NEGATIVE_INF, POSETIVE_INF}, singlemove::Move, transposition_table::{TranspositionsFlag, TranspositionsTable}, uci_message::UciMessage};
use rand::prelude::*;
use std::{collections::HashSet, time::{Duration, Instant}};
use std::sync::mpsc::Sender;


const MAX_EXTENTIONS: usize = 2;
const NULL_MOVE_REDUCTION: usize = 2;
const WINDOW: i32 = 50;

pub struct Searcher{
    traspos_table: TranspositionsTable,
    pub searches: u64,
    start_time: Instant,
    pub duration: Duration,
    tx: Sender<UciMessage>,
    search_moves: Option<Vec<String>>,
}

impl Searcher {
    pub fn new(traspos_table_size: usize, tx: Sender<UciMessage>) -> Searcher{
        Searcher{
            traspos_table: TranspositionsTable::new(traspos_table_size),
            searches: 0,
            start_time: Instant::now(),
            duration: Duration::from_millis(3000),
            tx,
            search_moves: None,
        }
    }

    pub fn iterative_deepening(&mut self, board: &mut Board) -> (Move, i32){
        // dont know if table needs clearing
        self.traspos_table.clear();
        self.searches = 0;
        let mut info = UciMessage::new_empty_info();
        let mut best_move = Move::null_move();
        let mut eval = 0;
        let mut alpha = NEGATIVE_INF;
        let mut beta = POSETIVE_INF;
        let mut depth = 1;
        
        self.start_time = Instant::now();
        loop {
            let (mv, val_depth) = self.search_alpha_beta(board, alpha, beta, depth, 0, 0, true);
            // return if searchtime has elapsed
            if self.start_time.elapsed() >= self.duration {
                break;
            }

            // We fell outside the window, so try again with a
            // full-width window (and the same depth).
            if (val_depth <= alpha) || (val_depth >= beta) {
                // send info about faild window
                {
                    info.info_add_string(format!("window search failed, nodes waisted {}", self.searches));
                    self.searches = 0;
                    self.tx.send(info).err();
                    info = UciMessage::new_empty_info();
                }

                alpha = NEGATIVE_INF;    
                beta = POSETIVE_INF;      
                continue;
            }

            // Set up the window for the next iteration.
            alpha = val_depth - WINDOW; 
            beta = val_depth + WINDOW;
            
            // update best move
            if !mv.is_null_move(){
                best_move = mv;
                eval = val_depth; 
            }
            
            // send info
            {
                info.info_add_depth(depth as u8);
                info.info_add_nodes(self.searches);
                self.searches = 0;
                if evaluate::is_mate_score(eval){
                    info.info_add_score_mate(evaluate::to_mate(eval));
                }else {
                    info.info_add_score_cp(eval);
                }
                info.info_add_hashfull(self.traspos_table.get_permill_fill());
                let best_line = self.get_current_best_line(board);
                if !best_line.is_empty(){
                    info.info_add_pv(best_line);
                }
                self.tx.send(info).err();
                info = UciMessage::new_empty_info();
            }

            // early exit if mate found
            if evaluate::is_mate_score(val_depth){ 
                break;
            }

            depth += 1;
        }
        (best_move, eval)
    }


    fn search_alpha_beta(&mut self, board: &mut Board, mut alpha: i32, beta: i32, depth: usize, ply: usize, extentions: usize, prev_nullmove: bool) -> (Move, i32){
        let mut flag = TranspositionsFlag::UpperBound;
        let zobrist = board.get_zobrist_hash();
        self.searches += 1;

        // draw by repetition
        if ply != 0 && board.game_history_contains(zobrist){
            return (Move::null_move(), evaluate::draw_by_repetition());
        }

        // lookup the position if it exists in the table
        if let Some(val) = self.traspos_table.lookup_eval(zobrist, depth, alpha, beta){
            if let Some(best) = self.traspos_table.get_best_move(zobrist) {
                return (best, val);
            }
            return (Move::null_move(),val);
        }
        
        // full depth is reached return nullmove and evaluation
        if depth == 0{
            let val = self.search_stable_pos(board, alpha, beta);
            self.traspos_table.record_entry(zobrist, depth, val, TranspositionsFlag::Exact, None);
            return (Move::null_move(),val);
        }

        // init bet move
        let mut best_move = Move::null_move();    

        let mut moves = board.get_possible_moves_turn();
        let check = board.in_check();    
        let extend = if check && extentions < MAX_EXTENTIONS { 1 } else { 0 };
        // start with the previous best move in the position
        if let Some(mv) = self.traspos_table.get_best_move(zobrist) {
            board.make_move(mv);
            let (_,mut val) = self.search_alpha_beta(board, -beta, -alpha, depth - 1 + extend, ply + 1, extentions + extend, false);
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

        // return 0 if stalemate else -Inf checkmate
        if moves.is_empty(){
            if board.in_check(){
                return (Move::null_move(), evaluate::mate_ajusted_score(ply));
            }else {
                return (Move::null_move(),0);
            }
        }

        // remove best found move if already seached
        if let Some(bm) = self.traspos_table.get_best_move(zobrist) {
            moves.retain(|mv| mv.from_to_mask() != bm.from_to_mask());
        }

        // check if specified seachmoves from gui
        if ply == 0{
            if let Some(search_moves) = &self.search_moves{
                moves.retain(|mv| {
                    search_moves.contains(&mv.long_algebraic_notation())
                });
            }
        }

        //random ordering for moves before ordering is implemented
        let mut rng = rand::thread_rng();
        moves.shuffle(&mut rng);

        // set a random best move to avoid nullmove being returned
        if best_move.is_null_move(){
            if let Some(mv) = moves.first() {
                best_move = *mv;
            }
        }

        // nullmove reduction
        if !prev_nullmove && depth >= 3 && !check && (board.state.white.bitmap_all() | board.state.black.bitmap_all()).count_ones() > 10{
            board.make_null_move();
            let (_, mut val) = self.search_alpha_beta(board, -beta, -beta + 1, depth - 1 - NULL_MOVE_REDUCTION, ply + 1, extentions, true);
            val = -val;
            board.undo_null_move();
            if val >= beta{
                return (Move::null_move(),beta);
            }
        }
        
        evaluate::sort_moves(&mut moves, board);

        for mv in moves{
            board.make_move(mv);
            let (_,mut val) = self.search_alpha_beta(board, -beta, -alpha, depth - 1 + extend, ply + 1, extentions + extend, false);
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

        evaluate::sort_moves(&mut moves, board);

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
                (*mv, evaluate::evaluate_white_old(board))
                
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

    fn get_current_best_line(&mut self, board: &mut Board) -> Vec<Move>{
        let mut res = vec![];
        let mut been: HashSet<u64> = HashSet::new();
        let mut zob = board.get_zobrist_hash();
        while let Some(mv) = self.traspos_table.get_best_move(zob) {
            been.insert(zob);
            res.push(mv);
            board.make_move(mv);
            zob = board.get_zobrist_hash();
            if been.contains(&zob){ break; }
        }
        res.iter().for_each(|_| {
            board.undo_last_move();
        });
        res
    }

    pub fn set_search_moves(&mut self,moves: Option<Vec<String>>){
        self.search_moves = moves;
    }
}

