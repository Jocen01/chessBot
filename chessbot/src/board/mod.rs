use std::fmt;

use crate::{pice::{self, Pice}, singlemove::{Move, MoveType}, state::State, Color, PiceType};

pub struct Board{
    pub pices: Vec<Pice>,
    board: [Option<usize>;64],
    turn: Color,
    moves: Vec<Move>,
    pub state: State,
}

impl Board {
    fn new(pices: Vec<Pice>, board: [Option<usize>;64], turn: Color, state: State) -> Board{
        Board { pices, board, turn, moves: vec![], state}
    }

    pub fn default() -> Board{
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn from_fen(s: &str) -> Board{
        let seq: Vec<&str> = s.split(" ").collect();
        let mut pices: Vec<Pice> = vec![];
        let mut board: [Option<usize>;64] = [None;64];
        let mut i = 56;
        for (_, c) in seq[0].char_indices(){
            if c.is_numeric(){
                i += c.to_digit(10).unwrap() as u8;
            }else if c == '/' {
                i -= 16
            }else {
                pices.push(Pice::from_char(c, i));
                i+=1;
            }
        }
        pices.iter().enumerate().for_each(|(idx, pice)| {
            board[pice.pos as usize] = Some(idx);
        });
        let turn = Color::from_char(seq[1].chars().nth(0).unwrap());
        let mut state = State::from_pices(&pices, 0, 0);
        if seq[3] != "-"{
            state.passant = 1<<Board::square_to_bitboard(seq[3])
        }
        state.casle_rights = Board::str_to_casle_rights(seq[2]);


        Board::new(pices, board, turn, state)
    }

    pub fn get_pice_pos(&self, p: u8) -> Option<&Pice>{
        self.pices.iter().find(|&pice| pice.pos == p && !pice.is_captured())
    }

    pub fn update_moves(&mut self, color: Color) {
        self.pices.iter_mut().filter(|pice| pice.color() == color).for_each(|pice|{
            pice.update_moves(&self.state);
        });
        self.reset_state_can_move(color);

        // self.reset_state_pices_bitboard();
        // for i in 0..self.pices.len(){
        //     self.pices[i].update_moves(&self.state);
        // }
        // self.reset_state_can_move();
        // for i in 0..self.pices.len(){
        //     if self.pices[i].pice_type() == PiceType::King{
        //         self.pices[i].update_moves(&self.state);
        //     }
        // }
    }

    // fn reset_state_pices_bitboard(&mut self){
    //     todo!();
    //     // self.state.reset_pices_bitboard();
    //     // self.pices.iter().filter(|&pice| !pice.is_captured()).for_each(|pice| {
    //     //     if pice.color() == Color::White{
    //     //         self.state.white_pices_bitboard |= 1<<pice.pos
    //     //     }else {
    //     //         self.state.black_pices_bitboard |= 1<<pice.pos
    //     //     }
    //     // });
    // }

    fn reset_state_can_move(&mut self, color: Color){
        self.state.reset_can_capture(color, &self.pices);
        // self.state.reset_can_move();
        // self.pices.iter().for_each(|pice| {
        //     if pice.color() == Color::White{
        //         self.state.white_can_move |= pice.moves
        //     }else {
        //         self.state.black_can_move |= pice.moves
        //     }
        // });
    }

    pub fn get_possible_moves(&mut self, color: Color) -> Vec<Move>{
        let mut moves: Vec<Move> = vec![];
        self.update_moves(color);
        self.pices.iter_mut().filter(|p| p.color() == color && !p.is_captured()).for_each(|pice| {
            let mut add = pice.get_moves(&self.state);
            moves.append(&mut add);
        });
        moves.retain(|mv| {
            self.make_move(*mv);
            self.update_moves(self.turn);

            let res = self.state.in_check(self.turn.other());
            self.undo_last_move();
            !res
        });
        moves
    }

    pub fn make_move(&mut self, mut mv: Move) {
        if let Some(pice_pos) = self.board[mv.to() as usize] {
            self.pices[pice_pos].capture();
            mv.capture(pice_pos);
            self.state.remove_pice(mv.to(), &self.pices[pice_pos]);
        }
        self.move_pice(mv);
        match mv.move_type() {
            MoveType::Normal | MoveType::PromotionQueen | MoveType::PromotionRook | MoveType::PromotionBishop | MoveType::PromotionKnight | MoveType::Pawndubblemove => {
                
            },
            MoveType::Castle => {
                todo!();
            },
            MoveType::Pessant => {
                todo!();
            }
        }
        
        self.moves.push(mv);
        self.turn = self.turn.other();
    }

    pub fn undo_last_move(&mut self){
        if let Some(mv) = self.moves.pop() {
            match mv.move_type() {
                MoveType::Normal | MoveType::PromotionQueen | MoveType::PromotionRook | MoveType::PromotionBishop | MoveType::PromotionKnight | MoveType::Pawndubblemove => {
                    self.undo_move_pice(mv);
                    if let Some(captured_pos) = mv.get_captured() {
                        self.pices[captured_pos].uncapture();
                        self.board[mv.to() as usize] = Some(captured_pos);
                        self.state.reinstate_pice(mv.to(), &self.pices[captured_pos]);
                    }
                },
                MoveType::Castle => {
                    todo!();
                },
                MoveType::Pessant => {
                    todo!();
                }
            }
            
        }
        self.turn = self.turn.other();
        self.update_moves(self.turn);

    }

    fn move_pice(&mut self, mv: Move){
        if let Some(pice_pos) = self.board[mv.from() as usize] {
            self.pices[pice_pos].move_to(&mv);
            self.board[mv.to() as usize] = self.board[mv.from() as usize];
            self.board[mv.from() as usize] = None;
            self.state.move_pice(mv.from(), mv.to(), &self.pices[pice_pos]);
        }
    }

    fn undo_move_pice(&mut self, mv: Move){
        if let Some(pice_pos) = self.board[mv.to() as usize] {
            self.pices[pice_pos].undo_move(&mv);
            self.board[mv.from() as usize] = self.board[mv.to() as usize];
            self.board[mv.to() as usize] = None;
            self.state.move_pice(mv.to(), mv.from(), &self.pices[pice_pos]);
        }
    }

    fn square_to_bitboard(pos: &str) -> u8{
        let mut bitboard: u8 = 0;
        bitboard += pos.chars().nth(0).unwrap() as u8 - 'a' as u8;
        bitboard + (((pos.chars().nth(1).unwrap().to_digit(10).unwrap() as u8)-1)<<3)

    }

    fn str_to_casle_rights(s: &str) -> u8{
        let mut rights = 0;
        for (idx, c) in vec!['K','Q','k','q'].iter().rev().enumerate(){
            if s.contains(*c){
                rights |= 1<<idx
            }
        }
        rights
    }
}

fn get_set_bits(&pos: &u64) -> Vec<u8>{
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

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let seq: Vec<String> = (0..64).map(| i | {
            match &self.get_pice_pos(i) {
                Some(p) => p.char(),
                None => ".".into()
            }
        }).collect();
        for row in seq.chunks(8).rev() {
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Turn: {:?}", self.turn)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::{get_set_bits, Board}, pice::Pice, Color, PiceType};

    #[test]
    fn fen_default() {
        let b: Board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(b.turn, Color::White);
        let pices: Vec<Option<&Pice>> = (0..64).map(|i| b.get_pice_pos(i)).collect();
        for p in &pices[0..16]{
            assert_eq!(p.unwrap().color(), Color::White)
        }
        for p in &pices[16..48]{
            assert!(p.is_none())
        }
        for p in &pices[48..]{
            assert_eq!(p.unwrap().color(), Color::Black)
        }
    }

    #[test]
    fn fen1() {
        let b: Board = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
        assert_eq!(b.turn, Color::Black);
        let pices: Vec<Option<&Pice>> = (0..64).map(|i| b.get_pice_pos(i)).collect();
        assert_eq!(pices[21].unwrap().pice_type(), PiceType::Knight);
        assert_eq!(pices[21].unwrap().color(), Color::White);
        assert_eq!(pices[28].unwrap().pice_type(), PiceType::Pawn);
        assert_eq!(pices[28].unwrap().color(), Color::White);
        assert_eq!(pices[34].unwrap().pice_type(), PiceType::Pawn);
        assert_eq!(pices[34].unwrap().color(), Color::Black);
        assert!(pices[6].is_none());
        assert!(pices[12].is_none());
        assert!(pices[50].is_none());
    }

    #[test]
    fn from_fen_en_passant_black() {
        let b: Board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3");
        assert_eq!(b.state.passant, 1<<45);
    }

    #[test]
    fn from_fen_en_passant_white() {
        let b: Board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pP3/5pP1/5N2/PPPP1P1P/RNBQKB1R b KQkq g3 0 4");
        assert_eq!(b.state.passant, 1<<22);
    }

    #[allow(non_snake_case)]
    #[test]
    fn from_fen_casle_rights_KQkq() {
        let b: Board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pP3/5pP1/5N2/PPPP1P1P/RNBQKB1R b KQkq g3 0 4");
        assert_eq!(b.state.casle_rights, 0b1111);
    }

    #[allow(non_snake_case)]
    #[test]
    fn from_fen_casle_rights_Kk() {
        let b: Board = Board::from_fen("1nbqkbnr/rpp1pppp/p7/4P3/3p4/P2P4/RPP2PPP/1NBQKBNR b Kk - 2 5");
        assert_eq!(b.state.casle_rights, 0b1010);
    }

    #[allow(non_snake_case)]
    #[test]
    fn from_fen_casle_rights_Kq() {
        let b: Board = Board::from_fen("rnbqkbn1/pppppppr/7p/8/8/P7/RPPPPPPP/1NBQKBNR w Kq - 2 3");
        assert_eq!(b.state.casle_rights, 0b1001);
    }

    #[test]
    fn square_to_bitboard() {
        assert_eq!(Board::square_to_bitboard("a1"), 0);
        assert_eq!(Board::square_to_bitboard("b1"), 1);
        assert_eq!(Board::square_to_bitboard("c1"), 2);
        assert_eq!(Board::square_to_bitboard("d4"), 27);
        assert_eq!(Board::square_to_bitboard("h8"), 63);
    }

    #[test]
    fn test_get_set_bits() {
        assert_eq!(get_set_bits(&0b1001001101110), vec![1,2,3,5,6,9,12]);
        assert_eq!(get_set_bits(&0), vec![]);
        assert_eq!(get_set_bits(&0b111), vec![0,1,2]);
    }

    // #[test]
    fn count_moves_one_move() {
        let mut board = Board::from_fen("r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 1 2");
        assert_eq!(count_moves_print(&mut board, 2, 1), 380);
    }

    #[test]
    fn count_moves_one_move_2() {
        let mut board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 2");
        assert_eq!(count_moves_print(&mut board, 2, 1), 656);
    }

    #[test]
    fn count_moves_default() {
        let mut board = Board::default();
        // assert_eq!(count_moves(&mut board, 1), 20);
        // assert_eq!(count_moves(&mut board, 2), 400);
        // assert_eq!(count_moves(&mut board, 3), 8_902);
        // assert!(1==2);

        assert_eq!(count_moves(&mut board, 4), 197_281);
        assert_eq!(count_moves(&mut board, 5), 4_865_609);
        // assert_eq!(count_moves(&mut board, 6), 119_060_324);
        // assert_eq!(count_moves(&mut board, 7), 3_195_901_860);
        // assert_eq!(count_moves(&mut board, 8), 84_998_978_956);
    }

    fn count_moves(board: &mut Board, depth: u8) -> u64{
        if depth == 0{ 
            // println!("{}", board);
            return 1;
        }
        let moves = board.get_possible_moves(board.turn);

        let mut res = 0;
        for m in moves{
            board.make_move(m);
            res += count_moves(board, depth - 1);
            board.undo_last_move();
        }
        res
    }

    fn count_moves_print(board: &mut Board, depth: u8, print_depth: i32) -> u64{
        if depth == 0{ 
            // println!("{}", board);
            return 1;
        }
        let moves = board.get_possible_moves(board.turn);
        
        let mut res = 0;
        for m in moves{
            board.make_move(m);
            let a = count_moves(board, depth - 1);
            if print_depth > 0{
                println!("from: {}, to {} : {}", m.from(), m.to(), a);
            }
            res += a;
            board.undo_last_move();
        }
        res
    }
}


// problems with moves
// 1. an pessant
// 2. casle
// 3. pins
// 4. checks
// 5. promotions