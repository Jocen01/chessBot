use crate::{constants, singlemove::{Move, MoveType}, state::{self, CastleRights, State}, Color, PiceType};


const CAPTURE_BIT: u8 = 5;
const QUEENSIDE_CASTLE_MASK_CAPTURE: u64 = 0b1100;
const QUEENSIDE_CASTLE_MASK_PICES: u64 = 0b1110;
const KINGSIDE_CASTLE_MASK: u64 = 0b1100000;


#[derive(Debug, PartialEq, Eq)]
pub struct Pice{
    pub typ: u8, //xxxCCTTT C=1 if captured
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

    pub fn is_captured(&self) -> bool{
        self.typ & (1<<CAPTURE_BIT) != 0
    }

    pub fn capture(&mut self) {
        self.typ |= 1<<CAPTURE_BIT;
    }

    pub fn uncapture(&mut self){
        self.typ &= !(1<<CAPTURE_BIT)
    }

    pub fn pice_type(&self) -> PiceType{
        PiceType::_type(self.typ)
    }

    pub fn color(&self) -> Color{
        Color::from_int(self.typ)
    }

    pub fn move_to(&mut self, mv: &Move) {
        match mv.move_type() {
            MoveType::Normal | MoveType::Pessant | MoveType::Pawndubblemove => {
                self.pos = mv.to();
                if self.pice_type() == PiceType::Pawn{
                    assert!(self.pos >=8 || self.pos < 56);
                }
            },
            MoveType::Castle => {
                if self.pice_type() == PiceType::King{
                    self.pos = mv.to()
                }else if mv.to() == 2{
                    self.pos = 3;
                }else if mv.to() == 6{
                    self.pos = 5;
                }else if mv.to() == 58{
                    self.pos = 59;
                }else if mv.to() == 62{
                    self.pos = 61;
                }else {
                    panic!("not a valid castle move{:?}", mv);
                }
            },
            MoveType::PromotionQueen => self.promote_to(PiceType::Queen),
            MoveType::PromotionRook => self.promote_to(PiceType::Rook),
            MoveType::PromotionBishop => self.promote_to(PiceType::Bishop),
            MoveType::PromotionKnight => self.promote_to(PiceType::Knight),
        }
    }

    pub fn undo_move(&mut self, mv: &Move) {
        match mv.move_type() {
            MoveType::Normal | MoveType::Pessant | MoveType::Pawndubblemove => self.pos = mv.from(),
            MoveType::Castle => {
                if self.pice_type() == PiceType::King{
                    self.pos = mv.from()
                }else if mv.to() == 2{
                    self.pos = 0;
                }else if mv.to() == 6{
                    self.pos = 7;
                }else if mv.to() == 58{
                    self.pos = 56;
                }else if mv.to() == 62{
                    self.pos = 63;
                }else {
                    panic!("not a valid castle move{:?}", mv);
                }
            },
            MoveType::PromotionQueen | MoveType::PromotionRook | MoveType::PromotionBishop | MoveType::PromotionKnight => {
                self.demote();
                self.pos = mv.from();
            },
        }
        
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

    pub fn get_moves(&mut self, state: &State ) -> Vec<Move>{
        match PiceType::_type(self.typ) {
            PiceType::King => self.gen_king_moves(&state),
            PiceType::Queen => self.gen_queen_moves(),
            PiceType::Rook => self.gen_rook_moves(),
            PiceType::Bishop => self.gen_bishop_moves(),
            PiceType::Knight => self.gen_knight_moves(),
            PiceType::Pawn => self.gen_pawn_moves(&state),
        }
    }

    fn update_moves_king(&mut self, state: &State) {
        let moves = constants::KINGS_BIT_MOVES[self.pos as usize];
        let own = state.piceboards(self.color());
        let enemy = state.piceboards(self.color().other());
        self.moves = moves & (!enemy.capture) & (!own.bitmap_all());
        
        // if Color::from_int(self.typ) == Color::White{
        //     self.moves = moves ^ (moves & state.white_pices_bitboard);
        //     self.moves ^= self.moves & state.black_can_move;
        //     self.moves ^= self.moves & constants::KINGS_BIT_MOVES[state.black_king as usize]
        // } else {
        //     self.moves = moves ^ (moves & state.black_pices_bitboard);
        //     self.moves ^= self.moves & state.white_can_move;
        //     self.moves ^= self.moves & constants::KINGS_BIT_MOVES[state.white_king as usize]
            
        // }
        // todo!("yet to implement casle")
    }

    fn gen_king_moves(&self, state: &State) -> Vec<Move>{
        // normal moves
        let mut moves : Vec<Move> = get_set_bits(&self.moves).iter().map(|i| {
            Move::new(self.pos, *i, MoveType::Normal)
        }).collect();

        let own = state.piceboards(self.color());
        let enemy = state.piceboards(self.color().other());
        // castle
        if !state.in_check(self.color()){
            let blockers = own.bitmap_all() | enemy.bitmap_all() | enemy.capture;
            let pices = own.bitmap_all() | enemy.bitmap_all();
            if self.color() == Color::Black{
                if blockers & QUEENSIDE_CASTLE_MASK_CAPTURE<<56 ==0 && pices & QUEENSIDE_CASTLE_MASK_PICES<<56 ==0 && state.casle_right(CastleRights::BlackQueenside){
                    moves.push(Move::new(self.pos, 58, MoveType::Castle));
                }
                if blockers & KINGSIDE_CASTLE_MASK<<56 ==0 && state.casle_right(CastleRights::BlackKingside){
                    moves.push(Move::new(self.pos, 62, MoveType::Castle));
                }
            }else {
                if blockers & QUEENSIDE_CASTLE_MASK_CAPTURE ==0 && pices & QUEENSIDE_CASTLE_MASK_PICES ==0 && state.casle_right(CastleRights::WhiteQueenside){
                    moves.push(Move::new(self.pos, 2, MoveType::Castle));
                }
                if blockers & KINGSIDE_CASTLE_MASK == 0 && state.casle_right(CastleRights::WhiteKingside){
                    moves.push(Move::new(self.pos, 6, MoveType::Castle));
                }
            }           
        }

        moves
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

    fn gen_queen_moves(&self) -> Vec<Move>{
        let moves : Vec<Move> = get_set_bits(&self.moves).iter().map(|i| {
            Move::new(self.pos, *i, MoveType::Normal)
        }).collect();
        moves
    }

    fn gen_bishop_moves(&self) -> Vec<Move>{
        let moves : Vec<Move> = get_set_bits(&self.moves).iter().map(|i| {
            Move::new(self.pos, *i, MoveType::Normal)
        }).collect();
        moves
    }

    fn gen_rook_moves(&self) -> Vec<Move>{
        let moves : Vec<Move> = get_set_bits(&self.moves).iter().map(|i| {
            Move::new(self.pos, *i, MoveType::Normal)
        }).collect();
        moves
    }

    fn update_moves_knight(&mut self, state: &State ) {
        let moves = constants::HORSE_BIT_MOVES[self.pos as usize];
        self.moves = moves & (!state.piceboards(self.color()).bitmap_all());
        // if Color::from_int(self.typ) == Color::White{
        //     self.moves = moves ^ (moves & state.white_pices_bitboard);
        // } else {
        //     self.moves = moves ^ (moves & state.black_pices_bitboard);
        // }
    }

    fn gen_knight_moves(&self) -> Vec<Move>{
        // todo pinned
        let moves = get_set_bits(&self.moves).iter().map(|i| {
            Move::new(self.pos, *i, MoveType::Normal)
        }).collect();
        moves
    }

    fn update_moves_pawn(&mut self, state: &State ) {
        let mut moves = 0;
        if self.color() == Color::White{
            // if !state.pice_at(self.pos + 8){
            //     moves |= 1<<(self.pos + 8);
            // }
            // if self.pos < 16 && moves != 0 && !state.pice_at(self.pos + 16){
            //     moves |= 1<<(self.pos + 16);
            // }
            if self.pos & 0b111 != 0{
                // if state.black_at(self.pos + 7) || state.passant_at(self.pos + 7) {
                    moves |= 1<<(self.pos + 7);
                // } 
            }
            if self.pos & 0b111 != 7{
                // if state.black_at(self.pos + 9) || state.passant_at(self.pos + 9) {
                    moves |= 1<<(self.pos + 9);
                // } 
            }
        } else {
            // if !state.pice_at(self.pos - 8){
            //     moves |= 1<<(self.pos - 8);
            // }
            // if self.pos >= 48 && moves != 0 && !state.pice_at(self.pos - 16){
            //     moves |= 1<<(self.pos - 16);
            // }
            if self.pos & 0b111 != 0{
                // if state.white_at(self.pos - 9) || state.passant_at(self.pos - 9){
                    moves |= 1<<(self.pos - 9);
                // } 
            }
            if self.pos & 0b111 != 7{
                // if state.white_at(self.pos - 7) || state.passant_at(self.pos - 7) {
                    moves |= 1<<(self.pos - 7);
                // } 
            }
        }
        self.moves = moves;
    }

    fn gen_pawn_moves(&self, state: &State) -> Vec<Move>{

        fn add_move(moves: &mut Vec<Move>, from: u8, to: u8 ) {
            if to < 8 || to >= 56{
                MoveType::iter_promotions().iter().for_each(|move_type| {
                    moves.push(Move::new(from, to, *move_type));
                })
            }else {
                moves.push(Move::new(from, to, MoveType::Normal));
            }
        }

        let mut moves: Vec<Move> = vec![];

        if self.color() == Color::White{
            if !state.pice_at(self.pos + 8){
                add_move(&mut moves, self.pos, self.pos + 8);

                //first move double push
                if self.pos < 16 && !state.pice_at(self.pos + 16){
                    moves.push(Move::new(self.pos, self.pos + 16, MoveType::Pawndubblemove));
                }
            }
           
            if self.pos & 0b111 != 0{
                if state.black_at(self.pos + 7)  {
                    add_move(&mut moves, self.pos, self.pos + 7);
                    // moves.push(Move::new(self.pos, self.pos + 7, MoveType::Normal));

                }else if state.passant_at(self.pos + 7) {
                    moves.push(Move::new(self.pos, self.pos + 7, MoveType::Pessant));
                }

            }
            if self.pos & 0b111 != 7{
                if state.black_at(self.pos + 9) {
                    add_move(&mut moves, self.pos, self.pos + 9);
                    // moves.push(Move::new(self.pos, self.pos + 9, MoveType::Normal));

                }else if state.passant_at(self.pos + 9) {
                    moves.push(Move::new(self.pos, self.pos + 9, MoveType::Pessant));
                }
            }
        } else {
            if !state.pice_at(self.pos - 8){
                add_move(&mut moves, self.pos, self.pos - 8);
                // moves.push(Move::new(self.pos, self.pos - 8, MoveType::Normal));

                //first move double push
                if self.pos >= 48 && !state.pice_at(self.pos - 16){
                    moves.push(Move::new(self.pos, self.pos - 16, MoveType::Pawndubblemove));
                }
            }
            if self.pos & 0b111 != 0{
                if state.white_at(self.pos - 9)  {
                    add_move(&mut moves, self.pos, self.pos - 9);
                    // moves.push(Move::new(self.pos, self.pos - 9, MoveType::Normal));

                }else if state.passant_at(self.pos - 9) {
                    moves.push(Move::new(self.pos, self.pos - 9, MoveType::Pessant));
                }

            }
            if self.pos & 0b111 != 7{
                if state.white_at(self.pos - 7) {
                    add_move(&mut moves, self.pos, self.pos - 7);
                    // moves.push(Move::new(self.pos, self.pos - 7, MoveType::Normal));

                }else if state.passant_at(self.pos - 7) {
                    moves.push(Move::new(self.pos, self.pos - 7, MoveType::Pessant));
                }
            }
        }
        moves
    }

    fn strait_moves(&self, state: &State) -> u64{
        let file = self.pos & 0b111;
        let rank = self.pos & 0b111000;
        let mut moves: u64 = 0;
        for i in file+1..8{
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
        for i in ((rank>>3)+1)..8{
            if state.pice_at((i<<3) | file) {
                if state.opposite_color_at((i<<3) | file, self.color()){
                    moves |= 1<<((i<<3) | file);
                }
                break;
            }else {
                moves |= 1<<((i<<3) | file);
            }
        }
        for i in (0..rank>>3).rev(){
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
                if state.pice_at(file + i + ((rank + i)<<3)) {
                    if state.opposite_color_at(file + i + ((rank + i)<<3), self.color()){
                        moves |= 1<<(file + i + ((rank + i)<<3));
                    }
                    break;
                }else {
                    moves |= 1<<(file + i + ((rank + i)<<3));
                }
            }else {
                break;
            }
        }
        for i in 1..{
            if file + i < 8 && rank >= i{
                if state.pice_at(file + i + ((rank - i)<<3)) {
                    if state.opposite_color_at(file + i + ((rank - i)<<3), self.color()){
                        moves |= 1<<(file + i + ((rank - i)<<3));
                    }
                    break;
                }else {
                    moves |= 1<<(file + i + ((rank - i)<<3));
                }
            }else {
                break;
            }
        }
        for i in 1..{
            if file >= i && rank >= i{
                if state.pice_at(file - i + ((rank - i)<<3)) {
                    if state.opposite_color_at(file - i + ((rank - i)<<3), self.color()){
                        moves |= 1<<(file - i + ((rank - i)<<3));
                    }
                    break;
                }else {
                    moves |= 1<<(file - i + ((rank - i)<<3));
                }
            }else {
                break;
            }
        }
        for i in 1..{
            if file >= i && rank + i < 8{
                if state.pice_at(file - i + ((rank + i)<<3)) {
                    if state.opposite_color_at(file - i + ((rank + i)<<3), self.color()){
                        moves |= 1<<(file - i + ((rank + i)<<3));
                    }
                    break;
                }else {
                    moves |= 1<<(file - i + ((rank + i)<<3));
                }
            }else {
                break;
            }
        }
        moves
    }

    fn promote_to(&mut self, pice_type: PiceType) {
        match pice_type {
            PiceType::Queen | PiceType::Bishop | PiceType::Rook | PiceType::Knight => {
                self.typ = (self.typ & (!0b111)) | (pice_type as u8);
                assert!(self.pice_type() == pice_type);
            },
            PiceType::King | PiceType::Pawn => panic!("cant promote to king or pawn")
        }
    }

    fn demote(&mut self) {
        match self.pice_type() {
            PiceType::Queen | PiceType::Bishop | PiceType::Rook | PiceType::Knight => {
                self.typ = (self.typ & (!0b111)) | (PiceType::Pawn as u8);
                assert!(self.pos >=8 || self.pos < 56);
            },
            PiceType::King | PiceType::Pawn => panic!("cant demote from king or pawn")
        }
    }
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


#[cfg(test)]
mod tests {
    use crate::{board::Board, pice::Pice, vec_pos_to_bitmap, Color, PiceType, pice::get_set_bits};

    #[test]
    fn get_set_bits_63(){
        let i: u64 = 1;
        get_set_bits(&(i<<63));
    }

    // #[test]
    fn white_pawn_moves_default_board() {
        let mut b: Board = Board::default();
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(8).unwrap().moves, vec_pos_to_bitmap(vec![16,24]));
        assert_eq!(b.get_pice_pos(12).unwrap().moves, vec_pos_to_bitmap(vec![20,28]));
        assert_eq!(b.get_pice_pos(15).unwrap().moves, vec_pos_to_bitmap(vec![23,31]));
    }

    // #[test]
    fn black_pawn_moves_default_board() {
        let mut b: Board = Board::default();
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(48).unwrap().moves, vec_pos_to_bitmap(vec![40,32]));
        assert_eq!(b.get_pice_pos(52).unwrap().moves, vec_pos_to_bitmap(vec![44,36]));
        assert_eq!(b.get_pice_pos(55).unwrap().moves, vec_pos_to_bitmap(vec![47,39]));
    }

    // #[test]
    fn white_pawn_moves_first_move_double_block() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1pppp/8/4P3/3p4/8/PPPP1PPP/RNBQKBNR w KQkq - 0 3");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(11).unwrap().moves, vec_pos_to_bitmap(vec![19]));
    }

    // #[test]
    fn black_pawn_moves_first_move_double_block() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1pppp/8/4P3/3p4/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 3");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(52).unwrap().moves, vec_pos_to_bitmap(vec![44]));
    }

    // #[test]
    fn white_pawn_moves_capture() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(28).unwrap().moves, vec_pos_to_bitmap(vec![35,36]));
    }

    // #[test]
    fn black_pawn_moves_capture() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(35).unwrap().moves, vec_pos_to_bitmap(vec![28,27]));
    }

    // #[test]
    fn white_pawn_moves_capture_en_passant() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.state.passant, 1<<45);
        assert!(b.state.passant_at(45));
        assert_eq!(b.get_pice_pos(36).unwrap().moves, vec_pos_to_bitmap(vec![44,45]));
    }

    // #[test]
    fn black_pawn_moves_capture_en_passant() {
        let mut b: Board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pP3/5pP1/5N2/PPPP1P1P/RNBQKB1R b KQkq g3 0 4");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(29).unwrap().moves, vec_pos_to_bitmap(vec![22]));
    }

    #[test]
    fn horse_moves_default_board() {
        let mut b: Board = Board::default();
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(1).unwrap().moves, vec_pos_to_bitmap(vec![16,18]));
        assert_eq!(b.get_pice_pos(6).unwrap().moves, vec_pos_to_bitmap(vec![21,23]));
        assert_eq!(b.get_pice_pos(57).unwrap().moves, vec_pos_to_bitmap(vec![40,42]));
        assert_eq!(b.get_pice_pos(62).unwrap().moves, vec_pos_to_bitmap(vec![45,47]));
    }

    #[test]
    fn horse_moves_capture_pices() {
        let mut b: Board = Board::from_fen("rnbqkb1r/pppppppp/2N5/8/8/2n5/PPPPPPPP/RNBQKB1R w KQkq - 6 4");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(42).unwrap().moves, vec_pos_to_bitmap(vec![59,52,36,27,25,32,48,57]));
        assert_eq!(b.get_pice_pos(18).unwrap().moves, vec_pos_to_bitmap(vec![1,3,12,28,33,35,8,24]));
    }

    #[test]
    fn rook_moves_default_board() {
        let mut b: Board = Board::default();
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(0).unwrap().moves, 0);
        assert_eq!(b.get_pice_pos(7).unwrap().moves, 0);
        assert_eq!(b.get_pice_pos(56).unwrap().moves, 0);
        assert_eq!(b.get_pice_pos(63).unwrap().moves, 0);
    }

    #[test]
    fn rook_moves_full_length_1() {
        let mut b: Board = Board::from_fen("rnbqkbn1/pppppp2/r7/8/8/7R/PPPPPPP1/RNBQKBN1 w Qq - 2 5");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(23).unwrap().moves, vec_pos_to_bitmap(vec![7,15,31,39,47,55,63,16,17,18,19,20,21,22]));
        let mut b: Board = Board::from_fen("1nbqkbn1/1ppppp2/6r1/1R6/r7/7R/1PPPPPP1/1NBQKBN1 b - - 2 10");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(24).unwrap().moves, vec_pos_to_bitmap(vec![0,8,16,32,40,48,56,25,26,27,28,29,30,31]));
    }
    
    #[test]
    fn rook_moves_capture() {
        let mut b: Board = Board::from_fen("1nbqkbn1/1ppppp2/8/1R4r1/r7/7R/1PPPPPP1/1NBQKBN1 w - - 3 11");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(33).unwrap().moves, vec_pos_to_bitmap(vec![32,34,35,36,37,38,17,25,41,49]));
        assert_eq!(b.get_pice_pos(38).unwrap().moves, vec_pos_to_bitmap(vec![14,22,30,46,54,33,34,35,36,37,39]));
    }

    #[test]
    fn bishop_moves_default_board() {
        let mut b: Board = Board::default();
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(2).unwrap().moves, 0);
        assert_eq!(b.get_pice_pos(5).unwrap().moves, 0);
        assert_eq!(b.get_pice_pos(58).unwrap().moves, 0);
        assert_eq!(b.get_pice_pos(61).unwrap().moves, 0);
    }

    #[test]
    fn bishop_moves_full_length_1() {
        let mut b: Board = Board::from_fen("1nbqk1n1/r1ppppbr/pp4p1/7p/7P/PP4P1/R1PPPPBR/1NBQK1N1 w - - 2 8");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(14).unwrap().moves, vec_pos_to_bitmap(vec![5,23,7,21,28,35,42,49,56]));
        assert_eq!(b.get_pice_pos(54).unwrap().moves, vec_pos_to_bitmap(vec![47,61,0,9,18,27,36,45,63]));
    }
    
    #[test]
    fn bishop_moves_capture() {
        let mut b: Board = Board::from_fen("1n1qk1n1/rbpp1p1r/pp2p1p1/3B3p/3b3P/PP3NP1/1RPPPP1R/1NBQK3 w - - 0 11");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(35).unwrap().moves, vec_pos_to_bitmap(vec![49,42,44,26,28]));
        assert_eq!(b.get_pice_pos(27).unwrap().moves, vec_pos_to_bitmap(vec![9,18,20,13,34,36,45,54,63]));
    }

    #[test]
    fn king_moves_default_board() {
        let mut b: Board = Board::default();
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(4).unwrap().moves, 0);
        assert_eq!(b.get_pice_pos(60).unwrap().moves, 0);
    }

    // TODO by fixing the code for this test to pass would probobly speed up the move generation a LOT!
    // #[test] 
    // fn king_moves_move_into_check() {
    //     let mut b: Board = Board::from_fen("rnbq1bnr/pppp1ppp/4p3/6k1/2K5/4P3/PPPP1PPP/RNBQ1BNR w - - 8 6");
    //     b.update_moves(crate::Color::White);
        // b.update_moves(crate::Color::Black);
    //     assert_eq!(b.get_pice_pos(26).unwrap().moves, vec_pos_to_bitmap(vec![33,17,18,19,27]));
    //     assert_eq!(b.get_pice_pos(38).unwrap().moves, vec_pos_to_bitmap(vec![45,46,47,37,31]));
    // }

    // #[test] 
    fn king_moves_no_castle_default() {
        let mut b: Board = Board::default();
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);

        assert_eq!(b.get_pice_pos(19).unwrap().moves, vec_pos_to_bitmap(vec![26,27,28,18,12]));
        assert_eq!(b.get_pice_pos(45).unwrap().moves, vec_pos_to_bitmap(vec![52,46,36,37,38]));
    }

    #[test] 
    fn king_moves_move_only_blocked_by_own_pices() {
        let mut b: Board = Board::from_fen("rnbq1bnr/pppp1ppp/4pk2/8/8/3KP3/PPPP1PPP/RNBQ1BNR w - - 10 7");
        b.update_moves(crate::Color::White);
        b.update_moves(crate::Color::Black);
        assert_eq!(b.get_pice_pos(19).unwrap().moves, vec_pos_to_bitmap(vec![26,27,28,18,12]));
        assert_eq!(b.get_pice_pos(45).unwrap().moves, vec_pos_to_bitmap(vec![52,46,36,37,38]));
    }

    #[test] 
    fn promotion_queen() {
        let mut pice = Pice::new(PiceType::Pawn, crate::Color::White, 50);
        pice.promote_to(PiceType::Queen);
        assert_eq!(pice.color(), Color::White);
    }

}


// problems with moves
// 1. an pessant
// 2. casle
// 3. pins
// 4. checks
// 5. promotions

// pinned pices
// an passant pinned
// get oposition attacks
// 