use crate::{board::Board, constants, state::State, Color, PiceType};

#[derive(Debug)]
pub struct Pice{
    pub typ: u8,
    pub pos: u8,
    pub moves: u64,
    pinned: bool,
}

impl Pice {
    pub fn new(pice: PiceType, color: Color, pos: u8) -> Pice{
        Pice { typ: pice as u8 | color as u8, pos: pos, moves: 0, pinned: false }
    }

    pub fn from_char(c: char, pos: u8) -> Pice{
        let color = Color::from_char(c);
        let pice = PiceType::from_char(c);
        Pice::new(pice, color, pos)
    }

    pub fn char(&self) -> String{
        let mut c: String = PiceType::char(self.typ).into();
        if self.typ & 8 == 8{
            c = c.to_uppercase();
        }
        c
    }

    pub fn pice_type(&self) -> PiceType{
        PiceType::_type(self.typ)
    }

    pub fn color(&self) -> Color{
        Color::from_int(self.typ)
    }

    pub fn update_moves(&mut self, state: &State ) {
        match PiceType::_type(self.typ) {
            PiceType::King => self.update_moves_king(&state),
            PiceType::Queen => self.update_moves_queen(&state),
            PiceType::Rook => self.update_moves_rook(&state),
            PiceType::Bishop => self.update_moves_bishop(&state),
            PiceType::Knight => self.update_moves_knight(&state),
            PiceType::Pawn => self.update_moves_pawn(&state),
        }
    }

    fn update_moves_king(&mut self, state: &State ) {
        let moves = constants::KINGS_BIT_MOVES[self.pos as usize];
        if Color::from_int(self.typ) == Color::White{
            self.moves = moves ^ (moves & state.white_pices_bitboard);
            self.moves ^= self.moves & state.black_can_move;
        } else {
            self.moves = moves ^ (moves & state.black_pices_bitboard);
            self.moves ^= self.moves & state.white_can_move;
        }
        todo!("yet to implement casle")
    }

    fn update_moves_queen(&mut self, state: &State ) {
        self.moves = self.strait_moves(state) | self.diagonal_moves(state);
    }

    fn update_moves_rook(&mut self, state: &State ) {
        self.moves = self.strait_moves(state);
    }

    fn update_moves_bishop(&mut self, state: &State ) {
        self.moves = self.diagonal_moves(state);
    }

    fn update_moves_knight(&mut self, state: &State ) {
        let moves = constants::HORSE_BIT_MOVES[self.pos as usize];
        if Color::from_int(self.typ) == Color::White{
            self.moves = moves ^ (moves & state.white_pices_bitboard);
        } else {
            self.moves = moves ^ (moves & state.black_pices_bitboard);
        }
    }

    fn update_moves_pawn(&mut self, state: &State ) {
        let mut moves = 0;
        if self.color() == Color::White{
            if !state.pice_at(self.pos + 8){
                moves |= 1<<(self.pos + 8);
            }
            if self.pos < 16 && moves != 0 && !state.pice_at(self.pos + 16){
                moves |= 1<<(self.pos + 16);
            }
            if self.pos & 0b111 != 0{
                if state.black_at(self.pos + 7) || state.passant_at(self.pos + 7) {
                    moves |= 1<<(self.pos + 7);
                } 
            }
            if self.pos & 0b111 != 7{
                if state.black_at(self.pos + 9) || state.passant_at(self.pos + 9) {
                    moves |= 1<<(self.pos + 9);
                } 
            }
        } else {
            if !state.pice_at(self.pos - 8){
                moves |= 1<<(self.pos - 8);
            }
            if self.pos >= 48 && moves != 0 && !state.pice_at(self.pos - 16){
                moves |= 1<<(self.pos - 16);
            }
            if self.pos & 0b111 != 0{
                if state.white_at(self.pos - 9) || state.passant_at(self.pos - 9){
                    moves |= 1<<(self.pos - 9);
                } 
            }
            if self.pos & 0b111 != 7{
                if state.white_at(self.pos - 7) || state.passant_at(self.pos - 7) {
                    moves |= 1<<(self.pos - 7);
                } 
            }
        }
        self.moves = moves;
    }

    fn strait_moves(&self, state: &State) -> u64{
        let file = self.pos & 0b111;
        let rank = self.pos & 0b111000;
        let mut moves: u64 = 0;
        for i in file..8{
            if state.pice_at(i | rank) {
                if state.opposite_color_at(i | rank, self.color()){
                    moves |= 1<<(i | rank);
                }
                break;
            }else {
                moves |= 1<<(i | rank);
            }
        }
        for i in (0..file).rev(){
            if state.pice_at(i | rank) {
                if state.opposite_color_at(i | rank, self.color()){
                    moves |= 1<<(i | rank);
                }
                break;
            }else {
                moves |= 1<<(i | rank);
            }
        }
        for i in rank..8{
            if state.pice_at((i<<3) | file) {
                if state.opposite_color_at((i<<3) | file, self.color()){
                    moves |= 1<<((i<<3) | file);
                }
                break;
            }else {
                moves |= 1<<((i<<3) | file);
            }
        }
        for i in (0..rank).rev(){
            if state.pice_at((i<<3) | file) {
                if state.opposite_color_at((i<<3) | file, self.color()){
                    moves |= 1<<((i<<3) | file);
                }
                break;
            }else {
                moves |= 1<<((i<<3) | file);
            }
        }
        moves
    }

    fn diagonal_moves(&self, state: &State) -> u64{
        let mut moves = 0;
        let file = self.pos & 0b111;
        let rank = (self.pos>>3) & 0b111;
        for i in 1..{
            if file + i < 8 && rank + i < 8{
                if state.pice_at(file + i + (rank + i)<<3) {
                    if state.opposite_color_at(file + i + (rank + i)<<3, self.color()){
                        moves |= 1<<(file + i + (rank + i)<<3);
                    }
                    break;
                }else {
                    moves |= 1<<(file + i + (rank + i)<<3);
                }
            }else {
                break;
            }
        }
        for i in 1..{
            if file + i < 8 && rank >= i{
                if state.pice_at(file + i + (rank - i)<<3) {
                    if state.opposite_color_at(file + i + (rank - i)<<3, self.color()){
                        moves |= 1<<(file + i + (rank - i)<<3);
                    }
                    break;
                }else {
                    moves |= 1<<(file + i + (rank - i)<<3);
                }
            }else {
                break;
            }
        }
        for i in 1..{
            if file >= i && rank >= i{
                if state.pice_at(file - i + (rank - i)<<3) {
                    if state.opposite_color_at(file - i + (rank - i)<<3, self.color()){
                        moves |= 1<<(file - i + (rank - i)<<3);
                    }
                    break;
                }else {
                    moves |= 1<<(file - i + (rank - i)<<3);
                }
            }else {
                break;
            }
        }
        for i in 1..{
            if file >= i && rank + i < 8{
                if state.pice_at(file - i + (rank + i)<<3) {
                    if state.opposite_color_at(file - i + (rank + i)<<3, self.color()){
                        moves |= 1<<(file - i + (rank + i)<<3);
                    }
                    break;
                }else {
                    moves |= 1<<(file - i + (rank + i)<<3);
                }
            }else {
                break;
            }
        }
        moves
    }
}


#[cfg(test)]
mod tests {
    use crate::{board::Board, pice::Pice, Color, PiceType, vec_pos_to_bitmap};

    #[test]
    fn white_pawn_moves_default_board() {
        let mut b: Board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        b.update_moves();
        assert_eq!(b.get_pice_pos(8).unwrap().moves, vec_pos_to_bitmap(vec![16,24]));
        assert_eq!(b.get_pice_pos(12).unwrap().moves, vec_pos_to_bitmap(vec![20,28]));
        assert_eq!(b.get_pice_pos(15).unwrap().moves, vec_pos_to_bitmap(vec![23,31]));
    }

    #[test]
    fn black_pawn_moves_default_board() {
        let mut b: Board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        b.update_moves();
        assert_eq!(b.get_pice_pos(48).unwrap().moves, vec_pos_to_bitmap(vec![40,32]));
        assert_eq!(b.get_pice_pos(52).unwrap().moves, vec_pos_to_bitmap(vec![44,36]));
        assert_eq!(b.get_pice_pos(55).unwrap().moves, vec_pos_to_bitmap(vec![47,39]));
    }

    #[test]
    fn white_pawn_moves_first_move_double_block() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1pppp/8/4P3/3p4/8/PPPP1PPP/RNBQKBNR w KQkq - 0 3");
        b.update_moves();
        assert_eq!(b.get_pice_pos(11).unwrap().moves, vec_pos_to_bitmap(vec![19]));
    }

    #[test]
    fn black_pawn_moves_first_move_double_block() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1pppp/8/4P3/3p4/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 3");
        b.update_moves();
        assert_eq!(b.get_pice_pos(52).unwrap().moves, vec_pos_to_bitmap(vec![44]));
    }

    #[test]
    fn white_pawn_moves_capture() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        b.update_moves();
        assert_eq!(b.get_pice_pos(28).unwrap().moves, vec_pos_to_bitmap(vec![35,36]));
    }

    #[test]
    fn black_pawn_moves_capture() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        b.update_moves();
        assert_eq!(b.get_pice_pos(35).unwrap().moves, vec_pos_to_bitmap(vec![28,27]));
    }

    #[test]
    fn white_pawn_moves_capture_en_passant() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3");
        b.update_moves();
        assert_eq!(b.state.passant, 1<<45);
        assert!(b.state.passant_at(45));
        assert_eq!(b.get_pice_pos(36).unwrap().moves, vec_pos_to_bitmap(vec![44,45]));
    }

    #[test]
    fn black_pawn_moves_capture_en_passant() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pP3/5pP1/5N2/PPPP1P1P/RNBQKB1R b KQkq g3 0 4");
        b.update_moves();
        assert_eq!(b.get_pice_pos(29).unwrap().moves, vec_pos_to_bitmap(vec![22]));
    }
}


// problems with moves
// 1. an pessant
// 2. casle
// 3. pins
// 4. checks
// 5. promotions