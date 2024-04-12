use std::fmt;

use crate::{pice::Pice, singlemove::Move, state::State, Color, PiceType};

pub struct Board{
    pub pices: Vec<Pice>,
    // board: Vec<Option<&Pice>>,
    turn: Color,
    moves: Vec<Move>,
    pub state: State,
}

impl Board {
    fn new(pices: Vec<Pice>, turn: Color, state: State) -> Board{
        Board { pices: pices, turn: turn, moves: vec![], state: state}
    }

    pub fn default() -> Board{
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn from_fen(s: &str) -> Board{
        let seq: Vec<&str> = s.split(" ").collect();
        let mut pices: Vec<Pice> = vec![];
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
        let turn = Color::from_char(seq[1].chars().nth(0).unwrap());
        let mut state = State::default();
        if seq[3] != "-"{
            state.passant = 1<<Board::square_to_bitboard(seq[3])
        }
        state.casle_rights = Board::str_to_casle_rights(seq[2]);


        Board::new(pices, turn, state)
    }

    pub fn get_pice_pos(&self, p: u8) -> Option<&Pice>{
        self.pices.iter().find(|&pice| pice.pos == p)
    }

    pub fn update_moves(&mut self) {
        self.reset_state_pices_bitboard();
        for i in 0..self.pices.len(){
            if self.pices[i].pice_type() == PiceType::Pawn{
                self.pices[i].update_moves(&self.state);
            }
            if self.pices[i].pice_type() == PiceType::Knight{
                self.pices[i].update_moves(&self.state);
            }
        }
        self.reset_state_can_move();
    }

    fn reset_state_pices_bitboard(&mut self){
        self.state.reset_pices_bitboard();
        self.pices.iter().for_each(|pice| {
            if pice.color() == Color::White{
                self.state.white_pices_bitboard |= 1<<pice.pos
            }else {
                self.state.black_pices_bitboard |= 1<<pice.pos
            }
        });
    }

    fn reset_state_can_move(&mut self){
        self.state.reset_can_move();
        self.pices.iter().for_each(|pice| {
            if pice.color() == Color::White{
                self.state.white_can_move |= pice.moves
            }else {
                self.state.black_can_move |= pice.moves
            }
        });
    }

    pub fn get_possible_moves(&self) -> Vec<Move>{
        todo!()
    }

    pub fn make_move(&mut self, mv: Move) {
        todo!()
    }

    pub fn undo_last_move(&mut self) {
        todo!();
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
    use crate::{board::Board, pice::Pice, Color, PiceType};

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

    #[test]
    fn from_fen_casle_rights_KQkq() {
        let b: Board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pP3/5pP1/5N2/PPPP1P1P/RNBQKB1R b KQkq g3 0 4");
        assert_eq!(b.state.casle_rights, 0b1111);
    }

    #[test]
    fn from_fen_casle_rights_Kk() {
        let b: Board = Board::from_fen("1nbqkbnr/rpp1pppp/p7/4P3/3p4/P2P4/RPP2PPP/1NBQKBNR b Kk - 2 5");
        assert_eq!(b.state.casle_rights, 0b1010);
    }
    #[test]
    fn from_fen_casle_rights_Kq() {
        let b: Board = Board::from_fen("rnbqkbn1/pppppppr/7p/8/8/P7/RPPPPPPP/1NBQKBNR w Kq - 2 3");
        assert_eq!(b.state.casle_rights, 0b1001);
    }

    #[test]
    fn t() {
        assert_eq!(Board::square_to_bitboard("a1"), 0);
        assert_eq!(Board::square_to_bitboard("b1"), 1);
        assert_eq!(Board::square_to_bitboard("c1"), 2);
        assert_eq!(Board::square_to_bitboard("d4"), 27);
        assert_eq!(Board::square_to_bitboard("h8"), 63);
    }
}


// problems with moves
// 1. an pessant
// 2. casle
// 3. pins
// 4. checks
// 5. promotions