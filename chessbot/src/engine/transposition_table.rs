use crate::movegeneration::singlemove::Move;

#[derive(Debug, PartialEq, Eq)]
pub enum TranspositionsFlag{
    Exact = 0,
    LowerBound = 1,
    UpperBound = 2,
}

struct Entry{
    zobrist: u64,
    depth: usize,
    flag: TranspositionsFlag,
    value: i32,
    best: Option<Move>
}

impl Entry {
    fn new(zobrits: u64, depth: usize, flag: TranspositionsFlag, value: i32, best: Option<Move>) -> Entry{
        Entry{
            zobrist: zobrits,
            depth,
            flag,
            value,
            best
        }
    }
}

pub struct TranspositionsTable{
    hash_table: Vec<Option<Entry>>,
    size: usize,
    nbr_filled: usize
}
// https://web.archive.org/web/20071031100051/http://www.brucemo.com/compchess/programming/hashing.htm
impl TranspositionsTable {
    pub fn new(size: usize) -> TranspositionsTable{
        // println!("entry size: {}, nbr_entries: {}", std::mem::size_of::<Entry>(), (32*1024*2024)/std::mem::size_of::<Entry>());
        TranspositionsTable{
            hash_table: (0..size).map(|_| None).collect(),
            size,
            nbr_filled: 0
        }
    }

    pub fn clear(&mut self){
        self.nbr_filled = 0;
        // self.hash_table = (0..self.size).map(|_| None).collect();
        self.hash_table.iter_mut().for_each(|entry| *entry = None);
    }

    pub fn lookup_eval(&self, zobrist: u64, depth: usize, alpha: i32, beta: i32) -> Option<i32> {
        if let Some(entry) = &self.hash_table[(zobrist as usize) % self.size]{
            if entry.zobrist == zobrist{
                if entry.depth >= depth{
                    if entry.flag == TranspositionsFlag::Exact{
                        return Some(entry.value)
                    }else if entry.flag == TranspositionsFlag::UpperBound && entry.value <= alpha {
                        return Some(entry.value)
                    }else if entry.flag == TranspositionsFlag::LowerBound && entry.value >= beta {
                        return Some(entry.value)
                    }
                }
            }
        }
        None
    }

    pub fn record_entry(&mut self, zobrist: u64, depth: usize, value: i32, flag: TranspositionsFlag, mut best_move: Option<Move>){
        if let Some(mv) = best_move{
            if mv.is_null_move(){
                best_move = None;
            }
        }
        let entry = Entry::new(zobrist, depth, flag, value, best_move);
        if let None = self.hash_table[(zobrist as usize) % self.size] { self.nbr_filled+=1 };
        self.hash_table[(zobrist as usize) % self.size] = Some(entry);
    }

    pub fn get_best_move(&self, zobrist: u64) -> Option<Move>{
        if let Some(entry) = &self.hash_table[(zobrist as usize) % self.size]{
            if entry.zobrist == zobrist{
                return entry.best;
            }
        }
        None
    }

    pub fn get_permill_fill(&self) -> u16{
        (self.nbr_filled * 1000 / self.size) as u16
    }
}