use crate::singlemove::Move;

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
}

impl TranspositionsTable {
    pub fn new(size: usize) -> TranspositionsTable{
        TranspositionsTable{
            hash_table: (0..size).map(|_| None).collect(),
            size
        }
    }

    pub fn clear(&mut self){
        self.hash_table = (0..self.size).map(|_| None).collect();
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

    pub fn record_entry(&mut self, zobrist: u64, depth: usize, value: i32, flag: TranspositionsFlag, best_move: Option<Move>){
        let entry = Entry::new(zobrist, depth, flag, value, best_move);
        self.hash_table[(zobrist as usize) % self.size] = Some(entry);
    }

    pub fn get_best_move(&self, zobrist: u64) -> Option<Move>{
        if let Some(entry) = &self.hash_table[(zobrist as usize) % self.size]{
            entry.best
        }else {
            None
        }
    }
}