use crate::{board::Board, constants, magic, singlemove::{Move, MoveType}, state::{CastleRights, PiceBoards, State}, Color};

const QUEENSIDE_CASTLE_MASK_CAPTURE: u64 = 0b1100;
const QUEENSIDE_CASTLE_MASK_PICES: u64 = 0b1110;
const KINGSIDE_CASTLE_MASK: u64 = 0b1100000;

const MAGICS: bool = true;

pub struct MoveGenerator{
    own: PiceBoards,
    opponent: PiceBoards,
    own_pices: u64,
    opponent_pices: u64,
    all: u64,
    pinns: u64,
    state: State,
    checks: u64,
    own_king: u8,
    opponent_king: u8,
    white_to_move: bool,
    opponent_captures: u64,
    checkline: u64,
    captures_only: u64,
}

impl MoveGenerator {
    pub fn new(board: &Board) -> MoveGenerator{
        let (own, opponent) = match board.get_turn() {
            Color::White => (board.state.white, board.state.black),
            Color::Black => (board.state.black, board.state.white),
        };
        MoveGenerator { 
            own, 
            opponent, 
            own_pices: own.bitmap_all(),
            opponent_pices: opponent.bitmap_all(),
            all: 0,
            pinns: 0,
            state: board.state.clone(),
            checks: 0,
            own_king: 0,
            opponent_king: 0,
            white_to_move: true,
            opponent_captures: 0,
            checkline: 0,
            captures_only: u64::MAX
        }
    }

    pub fn gen_moves_turn(&mut self, board: &Board, captures_only: bool) -> Vec<Move>{
        self.state = board.state.clone();
        (self.own, self.opponent) = match board.get_turn() {
            Color::White => (board.state.white, board.state.black),
            Color::Black => (board.state.black, board.state.white),
        };
        self.own_pices = self.own.bitmap_all();
        self.opponent_pices = self.opponent.bitmap_all();
        self.all = self.own_pices | self.opponent_pices;
        self.own_king = self.own.king.trailing_zeros() as u8;
        self.opponent_king = self.opponent.king.trailing_zeros() as u8;
        self.white_to_move = match board.get_turn() {
            Color::White => true,
            Color::Black => false
        };
        self.opponent_captures = 0;
        self.checkline = 0;
        self.captures_only = if captures_only { self.opponent_pices } else { u64::MAX };

        self.update_oponent();

        let mut moves = vec![];
        self.gen_king_moves(&mut moves);

        if self.checks.count_ones() <= 1{
            self.gen_pawn_moves(&mut moves);
            self.gen_orthogonal_moves(&mut moves);
            self.gen_diagonal_moves(&mut moves);
            self.gen_knight_moves(&mut moves);
        }
        

        moves
    }

    fn gen_king_moves(&self, moves: &mut Vec<Move>){
        let mut m = constants::KINGS_BIT_MOVES[self.own_king as usize];
        m = m & (!self.opponent_captures) & (!self.own_pices) & self.captures_only;

        // normal moves
        get_set_bits(&m).iter()
        .for_each(|i| {
            moves.push(Move::new(self.own_king, *i, MoveType::Normal));
        });

        // return early if captures_only
        // castle is not considerd capture :)
        if self.captures_only != u64::MAX { return; };

        // castle
        if self.checks == 0{
            let blockers = self.all | self.opponent_captures;
            if !self.white_to_move{
                if blockers & QUEENSIDE_CASTLE_MASK_CAPTURE<<56 ==0 && self.all & QUEENSIDE_CASTLE_MASK_PICES<<56 ==0 && self.state.casle_right(CastleRights::BlackQueenside){
                    moves.push(Move::new(self.own_king, 58, MoveType::Castle));
                }
                if blockers & KINGSIDE_CASTLE_MASK<<56 ==0 && self.state.casle_right(CastleRights::BlackKingside){
                    moves.push(Move::new(self.own_king, 62, MoveType::Castle));
                }
            }else {
                if blockers & QUEENSIDE_CASTLE_MASK_CAPTURE ==0 && self.all & QUEENSIDE_CASTLE_MASK_PICES ==0 && self.state.casle_right(CastleRights::WhiteQueenside){
                    moves.push(Move::new(self.own_king, 2, MoveType::Castle));
                }
                if blockers & KINGSIDE_CASTLE_MASK == 0 && self.state.casle_right(CastleRights::WhiteKingside){
                    moves.push(Move::new(self.own_king, 6, MoveType::Castle));
                }
            }           
        }
    }

    fn gen_pawn_moves(&self, moves: &mut Vec<Move>){
        let mask = (!self.own_pices) & self.checkline & self.captures_only;
        if self.white_to_move{
            get_set_bits(&self.own.pawns).iter().for_each(|pos|{
                let mut m = constants::WHITE_PAWN_CAPTURES[*pos as usize] & self.opponent_pices;
                if (1<<(pos+8)) & self.all == 0{
                    m |= 1<<(pos+8);
                    if (pos < &16) && ((1<<(pos+16)) & self.all) == 0{
                        m |= 1<<(pos+16);
                    }
                }
                m &= mask;

                if self.pinns & (1<<pos) != 0 {
                    m &= self.pinned_ray(&pos);
                }

                get_set_bits(&m).iter()
                .for_each(|to| {
                    if to - pos == 16{
                        moves.push(Move::new(*pos, *to, MoveType::Pawndubblemove));
                    } else if to >= &56{
                        self.gen_promotions(moves, *pos, *to);
                    } else {
                        moves.push(Move::new(*pos, *to, MoveType::Normal));
                    }
                });

                
                m = constants::WHITE_PAWN_CAPTURES[*pos as usize];
                if m & self.state.passant != 0{
                    if (self.pinns & (1<<pos) != 0 && (self.pinned_ray(&pos) & self.state.passant) != 0) || 
                       (self.pinns & (1<<pos) == 0 && (self.checks == 0 || self.checks == (self.state.passant>>8))){
                        if !self.en_passant_pinned(1<<pos, self.state.passant>>8){
                            moves.push(Move::new(*pos, self.state.passant.trailing_zeros() as u8, MoveType::Pessant))
                        }
                    }
                }

            })
        }else {
            get_set_bits(&self.own.pawns).iter().for_each(|pos|{
                let mut m = constants::BLACK_PAWN_CAPTURES[*pos as usize] & self.opponent_pices;
                if (1<<(pos-8)) & self.all == 0{
                    m |= 1<<(pos-8);
                    if (pos >= &48) && ((1<<(pos-16)) & self.all) == 0{
                        m |= 1<<(pos-16);
                    }
                }
                m &= mask;
                if self.pinns & (1<<pos) != 0 {
                    m &= self.pinned_ray(&pos);
                }
                get_set_bits(&m).iter()
                .for_each(|to| {
                    if pos - to == 16{
                        moves.push(Move::new(*pos, *to, MoveType::Pawndubblemove));
                    } else if to < &8{
                        self.gen_promotions(moves, *pos, *to);
                    } else {
                        moves.push(Move::new(*pos, *to, MoveType::Normal));
                    }
                });

                
                m = constants::BLACK_PAWN_CAPTURES[*pos as usize];
                if m & self.state.passant != 0{
                    if (self.pinns & (1<<pos) != 0 && (self.pinned_ray(&pos) & self.state.passant) != 0) || 
                       (self.pinns & (1<<pos) == 0 && (self.checks == 0 || self.checks == (self.state.passant<<8))){
                        if !self.en_passant_pinned(1<<pos, self.state.passant<<8){
                            moves.push(Move::new(*pos, self.state.passant.trailing_zeros() as u8, MoveType::Pessant))
                        }
                    }
                }

            })
        }
        
    }

    fn gen_knight_moves(&self, moves: &mut Vec<Move>){
        let knights_can_move = self.own.knights & (!self.pinns);
        let mask = (!self.own_pices) & self.checkline & self.captures_only;
        get_set_bits(&knights_can_move).iter()
        .for_each(|pos| {
            get_set_bits(&(constants::HORSE_BIT_MOVES[*pos as usize] & mask)).iter()
            .for_each(|to| {
                moves.push(Move::new(*pos, *to, MoveType::Normal));
            })
        })
    }

    fn gen_diagonal_moves(&self, moves: &mut Vec<Move>){
        let mask = (!self.own_pices) & self.checkline & self.captures_only;
        let mut pices = self.own.diagonal_sliders;
        if self.checks != 0{
            pices &= !self.pinns;
        }
        get_set_bits(&pices).iter()
        .for_each(|pos| {
            let mut m = self.diagonal_mask(*pos, self.all) & mask;
            
            if (1<<pos) & self.pinns != 0{
                m &= self.pinned_ray(&pos);
            }

            get_set_bits(&m).iter()
            .for_each(|to| {
                moves.push(Move::new(*pos, *to, MoveType::Normal));
            })
        })
    }

    fn gen_orthogonal_moves(&self, moves: &mut Vec<Move>){
        let mask = (!self.own_pices) & self.checkline & self.captures_only;
        let mut pices = self.own.orthoganal_sliders;
        if self.checks != 0{
            pices &= !self.pinns;
        }
        get_set_bits(&pices).iter()
        .for_each(|pos| {
            let mut m = self.orthogonal_mask(*pos, self.all) & mask;
            if (1<<pos) & self.pinns != 0{
                m &= self.pinned_ray(&pos);
            }
            get_set_bits(&m).iter()
            .for_each(|to| {
                moves.push(Move::new(*pos, *to, MoveType::Normal));
            })
        })
    }

    fn update_oponent(&mut self){
        let mut knights = 0;
        get_set_bits(&self.opponent.knights).iter()
        .for_each(|pos|{
            let m = constants::HORSE_BIT_MOVES[*pos as usize];
            if m & self.own.king != 0 { 
                self.checks |= 1<<pos;
                self.checkline |= 1<<pos;
            }
            knights |= m;
        });

        let mut orthogolal = 0;
        get_set_bits(&self.opponent.orthoganal_sliders).iter()
        .for_each(|pos|{
            let m = self.orthogonal_mask(*pos, self.all ^ self.own.king);
            let between = if self.is_orthogolal_adj(*pos, self.own_king) {
                constants::BETWEEN[*pos as usize][self.own_king as usize]
            } else { 0 };
            if m & self.own.king != 0 { 
                self.checks |= 1<<pos; 
                self.checkline |= between | (1<<pos);
            }
            if between != 0 && (between & self.own_pices).count_ones() <= 1 && (between & self.opponent_pices).count_ones() == 0{
                self.pinns |= between;
            }
            orthogolal |= m;
        });

        let mut diagonal = 0;
        get_set_bits(&self.opponent.diagonal_sliders).iter()
        .for_each(|pos|{
            let m = self.diagonal_mask(*pos, self.all ^ self.own.king);
            let between = if self.is_diagonal_adj(pos, &self.own_king) {
                constants::BETWEEN[*pos as usize][self.own_king as usize]
            } else {0};
            if m & self.own.king != 0 { 
                self.checks |= 1<<pos;
                self.checkline |= between | (1<<pos);
            }
            if between != 0 && (between & self.own_pices).count_ones() <= 1 && (between & self.opponent_pices).count_ones() == 0{
                self.pinns |= between;
            }
            diagonal |= m;
        });

        let king = constants::KINGS_BIT_MOVES[self.opponent_king as usize];
        let mut pawn_capture = 0;
        get_set_bits(&self.opponent.pawns).iter()
        .for_each(|pos|{
            let m = if self.white_to_move {
                constants::BLACK_PAWN_CAPTURES[*pos as usize]
            }else{
                constants::WHITE_PAWN_CAPTURES[*pos as usize]
            };
            if m & self.own.king != 0 { 
                self.checks |= 1<<pos;
                self.checkline |= 1<<pos;
            }
            pawn_capture |= m;
        });

        // no need to calculate pawn quiet moves

        self.opponent_captures = pawn_capture | knights | orthogolal | diagonal | king;

        if self.checks == 0{
            self.checkline = u64::MAX;
        }
    }

    fn en_passant_pinned(&self, own_pawn: u64, opponent_pawn: u64) -> bool{
        if self.opponent.orthoganal_sliders != 0{
            let all = (self.own_pices | self.opponent_pices) ^ (self.state.passant | own_pawn | opponent_pawn);
            let king_see = self.orthogonal_mask(self.own_king, all);
            return (king_see & self.opponent.orthoganal_sliders) != 0;
        }
        false
    }

    fn orthogonal_mask(&self, pos: u8, blockers: u64) -> u64{
        if MAGICS{
            return magic::get_orthogonal_moves(pos as usize, blockers);
        }
        let file = pos & 0b111;
        let rank = pos & 0b111000;
        let mut moves: u64 = 0;
        for f in file+1..8{
            moves |= 1<<(rank | f);
            if ((1<<(rank | f)) & blockers) != 0{ break; }
        }
        for f in (0..file).rev(){
            moves |= 1<<(rank | f);
            if ((1<<(rank | f)) & blockers) != 0{ break; }
        }
        for r in ((rank>>3)+1)..8{
            moves |= 1<<((r<<3) | file);
            if ((1<<((r<<3) | file)) & blockers) != 0{ break; }
        }
        for r in (0..rank>>3).rev(){
            moves |= 1<<((r<<3) | file);
            if ((1<<((r<<3) | file)) & blockers) != 0{ break; }
        }
        moves
    }

    fn diagonal_mask(&self, pos: u8, blockers: u64) -> u64{
        if MAGICS{
            return magic::get_diagonal_moves(pos as usize, blockers);
        }
        let mut moves = 0;
        let file = pos & 0b111;
        let rank = (pos>>3) & 0b111;
        for i in 1..{
            if file + i < 8 && rank + i < 8{
                moves |= 1<<(file + i + ((rank + i)<<3));
                if blockers & (1<<(file + i + ((rank + i)<<3))) != 0 {
                    break;
                }
            }else {
                break;
            }
        }
        for i in 1..{
            if file + i < 8 && rank >= i{
                moves |= 1<<(file + i + ((rank - i)<<3));
                if blockers & (1<<(file + i + ((rank - i)<<3))) != 0{
                    break;
                }
            }else {
                break;
            }
        }
        for i in 1..{
            if file >= i && rank >= i{
                moves |= 1<<(file - i + ((rank - i)<<3));
                if blockers & (1<<(file - i + ((rank - i)<<3))) != 0{
                    break;
                }
            }else {
                break;
            }
        }
        for i in 1..{
            if file >= i && rank + i < 8{
                moves |= 1<<(file - i + ((rank + i)<<3));
                if blockers & (1<<(file - i + ((rank + i)<<3))) != 0 {
                    break;
                }
            }else {
                break;
            }
        }
        moves
    }

    fn pinned_ray(&self, pos: &u8) -> u64{
        if (pos & 0b111) == (self.own_king & 0b111){
            return constants::FILES_MASK[(pos & 0b111) as usize];
        }
        if (pos & 0b111000) == (self.own_king & 0b111000){
            return ((1<<8)-1)<<(pos & 0b111000);
        }
        // primary diag
        let p = prim_diag_idx(pos);
        if p == prim_diag_idx(&self.own_king){
            return constants::PRIMARY_DIGONAL_MASK[p];
        }

        // secundary diag
        let p = secund_diag_idx(pos);
        if p == secund_diag_idx(&self.own_king){
            return constants::SECUNDARY_DIGONAL_MASK[p];
        }
        u64::MAX
    }

    fn gen_promotions(&self, moves: &mut Vec<Move>, from: u8, to: u8){
        moves.push(Move::new(from, to, MoveType::PromotionQueen));
        moves.push(Move::new(from, to, MoveType::PromotionRook));
        moves.push(Move::new(from, to, MoveType::PromotionBishop));
        moves.push(Move::new(from, to, MoveType::PromotionKnight));
    }

    fn is_diagonal_adj(&self, i: &u8, j: &u8) -> bool{
        (prim_diag_idx(i) == prim_diag_idx(j)) ||
        (secund_diag_idx(i) == secund_diag_idx(j))
    }

    fn is_orthogolal_adj(&self, i: u8, j: u8) -> bool{
        (i & 0b111) == (j & 0b111) ||
        (i & 0b111000) == (j & 0b111000)
    }

    // only works after generation of moves
    pub fn in_check(&self) -> bool{
        self.checks != 0
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

fn prim_diag_idx(pos: &u8) -> usize{
    ((pos & 0b111) + (pos>>3)) as usize
}

fn secund_diag_idx(pos: &u8) -> usize{
    (7 - (pos & 0b111) + (pos>>3)) as usize
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, constants, movegenerator::prim_diag_idx};

    use super::MoveGenerator;


    #[test]
    fn movegenerator_test_1(){
        let board = Board::default();
        let mut mg = MoveGenerator::new(&board);
        let moves = mg.gen_moves_turn(&board, false);
        assert_eq!(20, moves.len());
    }

    #[test]
    fn movegenerator_pinned_ray(){
        let board = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
        let mut mg = MoveGenerator::new(&board);
        mg.gen_moves_turn(&board, false);
        assert_eq!(mg.pinned_ray(&14),constants::FILES_MASK[6]);
    }

    #[test]
    fn movegenerator_adj(){
        let board = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
        let mg = MoveGenerator::new(&board);
        assert!(mg.is_diagonal_adj(&0, &9));
        assert!(!mg.is_diagonal_adj(&0, &8));
        assert!(!mg.is_diagonal_adj(&0, &1));
        assert!(mg.is_diagonal_adj(&47, &61));
        assert!(constants::BETWEEN[47][61] != 0);
    }

    #[test]
    fn movegenerator_prim_secund_diag(){
        assert_eq!(prim_diag_idx(&8),prim_diag_idx(&1));
        assert_eq!(prim_diag_idx(&0),0);
    }
}