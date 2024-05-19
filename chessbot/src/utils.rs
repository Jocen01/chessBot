
#[allow(dead_code)]
pub fn vec_pos_to_bitmap(pos: Vec<u8>) -> u64{
    let mut res = 0;
    for i in pos{
        res |= 1<<i;
    }
    res
}

pub fn get_set_bits(pos: &u64) -> Vec<u8>{
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