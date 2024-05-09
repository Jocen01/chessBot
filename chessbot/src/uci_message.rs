use crate::{board::Board, singlemove::{MoveType,Move}};

#[derive(Debug, Clone, Copy)]
pub struct Score{
    cp: Option<i32>,
    mate: Option<i32>,
    lowerbound: Option<i32>,
    upperbound: Option<i32>,
}

impl Score {
    pub fn empty() -> Score{
        Score { cp: None, mate: None, lowerbound: None, upperbound: None }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum UciMessage {
    Uci,
    Debug(bool),
    IsReady,
    SetOption{
        name: String,
        value: String
    },
    Register{
        later: bool,
        name: String,
        code: String
    },
    UciNewGame,
    Position{
        fen: Option<String>,
        moves: Vec<Move>
    },
    Go{
        search_moves: Option<Vec<String>>,
        ponder: bool,
        wtime: Option<u64>,
        btime: Option<u64>,
        winc: Option<u64>,
        binc: Option<u64>,
        depth: Option<u8>,
        nodes: Option<u64>,
        mate: Option<u8>,
        move_time: Option<u64>,
        infinite: bool
    },
    Stop,
    Ponderhit,
    Quit,

    // Engine to GUI

    Id{
        name: String,
        author: String
    },
    UciOk,
    ReadyOk,
    BestMove{
        best_move: Move,
        ponder: Option<Move>
    },
    CopyProtection,
    Registration,
    Info{
        depth: Option<u8>,
        seldepth: Option<u8>,
        time: Option<u64>,
        nodes: Option<u64>,
        pv: Option<Vec<Move>>,
        multipv: Option<Vec<Vec<Move>>>,
        score: Option<Score>,
        currmove: Option<Move>,
        currmovenumber: Option<u16>,
        hashfull: Option<u16>,
        nps: Option<u32>,
        tbhits: Option<u64>,
        sbhits: Option<u64>,
        cpuload: Option<u16>,
        string: Option<String>,
        refutation: Option<Vec<Move>>,
        currline: Option<(Option<u16>, Vec<Move>)>
    },
    Option,
    Unknown(String),
}

impl UciMessage {
    pub fn parse(s: String) -> UciMessage{
        if s == "uci"{
            UciMessage::Uci
        } else if s.starts_with("debug") {
            if s.contains("on"){
                UciMessage::Debug(true)
            }else {
                UciMessage::Debug(false)
            }
            
        } else if s == "isready" {
            UciMessage::IsReady
        } else if s.starts_with("setoption") {
            let name_pos: usize = s.find("name").unwrap();
            let value_pos: usize = s.find("value").unwrap();

            // Extract the substring between "name" and "value"
            let name_value = s[name_pos + 4..value_pos].trim();

            // Extract the substring after "value"
            let value_value = s[value_pos + 5..].trim();
            UciMessage::SetOption { name: name_value.into(), value: value_value.into() }
        } else if s.starts_with("register") {
            panic!("not implemented")
        } else if s == "ucinewgame" {
            UciMessage::UciNewGame
        } else if s.starts_with("position") {
            let mut moves_res = vec![];
            let moves_pos = s.find("moves").unwrap_or(s.len());
            
            let fen: Option<String> = if s.contains("startpos") { None } else {
                let f = if s.contains("fen"){ 12 } else { 8 };
                Some(s[f..moves_pos].trim().into())
            };

            if s.contains("moves") {
            
                let mut board = if let Some(fenn) = fen.clone() {
                    Board::from_fen(&fenn)
                }else{
                    Board::default()
                };
                s[moves_pos + 5..].split_whitespace().into_iter().for_each(|move_str| {
                    let moves = board.get_possible_moves_turn();
                    let (from_square, to_square) = parse_algebraic_notation(move_str);
                    moves.iter().filter(|&mv| {
                        mv.from() == from_square && mv.to() == to_square
                    }).filter(|&mv|{
                        if move_str.len() == 5{
                            match move_str.chars().nth(4).unwrap() {
                                'q' => mv.move_type() == MoveType::PromotionQueen,
                                'r' => mv.move_type() == MoveType::PromotionRook,
                                'b' => mv.move_type() == MoveType::PromotionBishop,
                                'n' => mv.move_type() == MoveType::PromotionKnight,
                                _ => true
                            }
                        }else {
                            true
                        }
                    }).for_each(|mv| {
                        moves_res.push(*mv);
                        board.make_move(*mv);
                        board.add_state_to_history();

                    });
                });
            }
            
            UciMessage::Position { fen: fen, moves: moves_res }
        }else if s.starts_with("go") {
            let search_moves = get_vec_uci_moves(&s, "searchmoves");
            let ponder = s.contains("ponder");
            let wtime: Option<u64> = get_variable_value(&s, "wtime");
            let btime: Option<u64> = get_variable_value(&s, "btime");
            let winc: Option<u64> = get_variable_value(&s, "winc");
            let binc: Option<u64> = get_variable_value(&s, "binc");
            let depth: Option<u8> = get_variable_value(&s, "depth");
            let nodes: Option<u64> = get_variable_value(&s, "nodes");
            let mate: Option<u8> = get_variable_value(&s, "mate");
            let move_time: Option<u64> = get_variable_value(&s, "movetime");
            let infinite = s.contains("infinite");
            UciMessage::Go { 
                search_moves, 
                ponder, 
                wtime,
                btime, 
                winc, 
                binc, 
                depth, 
                nodes, 
                mate, 
                move_time, 
                infinite 
            }
        } else if s == "stop" {
            UciMessage::Stop
        } else if s == "ponderhit" {
            UciMessage::Ponderhit
        } else if s == "quit" {
            UciMessage::Quit
        } else {
            UciMessage::Unknown(s)
        }
    }

    pub fn serialize(&self) -> String{
        match self {
            UciMessage::Uci 
            | UciMessage::Debug(..) 
            | UciMessage::IsReady 
            | UciMessage::Register { .. } 
            | UciMessage::UciNewGame 
            | UciMessage::Position { .. } 
            | UciMessage::Go { .. } 
            | UciMessage::Stop 
            | UciMessage::Ponderhit 
            | UciMessage::Quit 
            | UciMessage::SetOption { .. } => {
                panic!("not serializable");
            },
            UciMessage::Id { name, author } => {
                format!("id name {}\nid author {}",name, author)
            },
            UciMessage::UciOk => {
                "uciok".into()
            },
            UciMessage::ReadyOk => {
                "readyok".into()
            },
            UciMessage::BestMove { best_move, ponder } => {
                if let Some(ponder_best) = ponder {
                    format!("bestmove {} ponder {}", best_move, ponder_best)
                }else {
                    format!("bestmove {}", best_move.long_algebraic_notation())
                }
            },
            UciMessage::CopyProtection => {
                panic!("not impl")
            },
            UciMessage::Registration => {
                panic!("not impl")
            },
            UciMessage::Info { depth,
                    seldepth,
                    time,
                    nodes,
                    pv,
                    multipv: _,
                    score,
                    currmove,
                    currmovenumber,
                    hashfull,
                    nps,
                    tbhits,
                    sbhits,
                    cpuload,
                    string,
                    refutation,
                    currline
                 } => {
                let mut s: Vec<String> = vec!["info".into()];
                push_variable(&mut s, "depth", depth);
                push_variable(&mut s, "seldepth", seldepth);
                push_variable(&mut s, "time", time);
                push_variable(&mut s, "nodes", nodes);
                if let Some(Score { cp, mate, lowerbound, upperbound }) = score {
                    push_variable(&mut s, "cp", cp);
                    push_variable(&mut s, "mate", mate);
                    push_variable(&mut s, "lowerbound", lowerbound);
                    push_variable(&mut s, "upperbound", upperbound);
                    
                }
                // push_vec_move(&mut s, "multipv", multipv);
                push_variable(&mut s, "currmove", currmove);
                push_variable(&mut s, "currmovenumber", currmovenumber);
                push_variable(&mut s, "hashfull", hashfull);
                push_variable(&mut s, "nps", nps);
                push_variable(&mut s, "tbhits", tbhits);
                push_variable(&mut s, "sbhits", sbhits);
                push_variable(&mut s, "cpuload", cpuload);
                push_vec_move(&mut s, "pv", pv);
                push_variable(&mut s, "string", string);
                push_vec_move(&mut s, "refutation", refutation);
                if let Some((_, v)) = currline{
                    push_vec_move_without_option(&mut s, "currline", v);
                }

                s.concat()
            },
            UciMessage::Option => {
                panic!("not impl")
            },
            UciMessage::Unknown(s) => {
                format!("use help command to find all commands, {} is not a command", s)
            }
        }
    }

    pub fn new_empty_info() -> UciMessage{
        UciMessage::Info { 
            depth:              None,
            seldepth:           None,
            time:               None,
            nodes:              None,
            pv:                 None,
            multipv:            None,
            score:              None,
            currmove:           None,
            currmovenumber:     None,
            hashfull:           None,
            nps:                None,
            tbhits:             None,
            sbhits:             None,
            cpuload:            None,
            string:             None,
            refutation:         None,
            currline:           None 
        }
    }

    #[allow(dead_code)]
    pub fn info_add_depth(&mut self, depth: u8) -> bool {
        if let UciMessage::Info { depth: d, .. } = self {
            *d = Some(depth);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_seldepth(&mut self, seldepth: u8) -> bool {
        if let UciMessage::Info { seldepth: sd, depth, .. } = self {
            if let Some(_) = depth {
                *sd = Some(seldepth);
                return true; // Indicate that the name was updated
            }
        }
        false // Indicate that the enum is not SetOption
        
    }

    #[allow(dead_code)]
    pub fn info_add_time(&mut self, time: u64) -> bool {
        if let UciMessage::Info { time: t, .. } = self {
            *t = Some(time);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_nodes(&mut self, nodes: u64) -> bool {
        if let UciMessage::Info { nodes: n, .. } = self {
            *n = Some(nodes);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_pv(&mut self, pv: Vec<Move>) -> bool {
        if let UciMessage::Info { pv: p, .. } = self {
            *p = Some(pv);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_score_cp(&mut self, cp: i32) -> bool {
        if let UciMessage::Info { score, .. } = self {
            if score.is_none(){
                *score = Some(Score::empty());
            }           
            if let Some(Score{ cp: c, .. }) = score {
                *c = Some(cp);
                true // Indicate that the name was updated
            }else {
                false
            }   
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_score_mate(&mut self, mate: i32) -> bool {
        if let UciMessage::Info { score, .. } = self {
            if score.is_none(){
                *score = Some(Score::empty());
            }
            if let Some(Score{ mate: m, .. }) = score {
                *m = Some(mate);
                true // Indicate that the name was updated
            }else {
                false
            }   
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_score_lowerbound(&mut self, lowerbound: i32) -> bool {
        if let UciMessage::Info { score, .. } = self {
            if score.is_none(){
                *score = Some(Score::empty());
            }  
            if let Some(Score { lowerbound: low, .. }) = score {
                *low = Some(lowerbound);
                true // Indicate that the name was updated
            }else {
                false
            }   
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_score_upperbound(&mut self, upperbound: i32) -> bool {
        if let UciMessage::Info { score, .. } = self {
            if score.is_none(){
                *score = Some(Score::empty());
            }
            if let Some(Score { upperbound: upper, .. }) = score {
                *upper = Some(upperbound);
                true // Indicate that the name was updated
            }else {
                false
            }   
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_currmove(&mut self, currmove: Move) -> bool {
        if let UciMessage::Info { currmove: cm, .. } = self {
            *cm = Some(currmove);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_currmovenumber(&mut self, currmovenumber: u16) -> bool {
        if let UciMessage::Info { currmovenumber: cmnbr, .. } = self {
            *cmnbr = Some(currmovenumber);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_hashfull(&mut self, hashfull: u16) -> bool {
        if let UciMessage::Info { hashfull: full, .. } = self {
            *full = Some(hashfull);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_nps(&mut self, nps: u32) -> bool {
        if let UciMessage::Info { nps: n, .. } = self {
            *n = Some(nps);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_tbhits(&mut self, tbhits: u64) -> bool {
        if let UciMessage::Info { tbhits: tbh, .. } = self {
            *tbh = Some(tbhits);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_sbhits(&mut self, sbhits: u64) -> bool {
        if let UciMessage::Info { sbhits: sbh, .. } = self {
            *sbh = Some(sbhits);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_cpuload(&mut self, cpuload: u16) -> bool {
        if let UciMessage::Info { cpuload: cpu, .. } = self {
            *cpu = Some(cpuload);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_string(&mut self, string: String) -> bool {
        if let UciMessage::Info { string: s, .. } = self {
            *s = Some(string);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_refutation(&mut self, refutation: Vec<Move>) -> bool {
        if let UciMessage::Info { refutation: r, .. } = self {
            *r = Some(refutation);
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }

    #[allow(dead_code)]
    pub fn info_add_currline(&mut self, currline: Vec<Move>) -> bool {
        if let UciMessage::Info { currline: cl, .. } = self {
            *cl = Some((None, currline));
            true // Indicate that the name was updated
        } else {
            false // Indicate that the enum is not SetOption
        }
    }
}

fn parse_algebraic_notation(alg_notation: &str) -> (u8, u8) {

    let from_file = alg_notation.chars().nth(0).unwrap();
    let from_rank = alg_notation.chars().nth(1).unwrap();
    let to_file = alg_notation.chars().nth(2).unwrap();
    let to_rank = alg_notation.chars().nth(3).unwrap();

    let from_square = map_to_square(from_file, from_rank).unwrap();
    let to_square = map_to_square(to_file, to_rank).unwrap();

    (from_square, to_square)
}

fn map_to_square(file: char, rank: char) -> Option<u8> {
    let file_index = match file {
        'a'..='h' => file as u8 - b'a',
        _ => return None, // Invalid file
    };
    let rank_index = match rank {
        '1'..='8' => rank as u8 - b'1',
        _ => return None, // Invalid rank
    };
    Some(rank_index * 8 + file_index)
}

fn get_variable_value<T>(inp: &str, flag: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    if let Some(idx) = inp.find(flag) {
        let remaining = &inp[(idx + flag.len())..];
        let next_word = remaining
            .split_whitespace()
            .next()
            .and_then(|word| word.parse::<T>().ok());
        next_word
    } else {
        None
    }
}

fn push_variable<T>(v: &mut Vec<String>, name: &str, variable: &Option<T>)
where
    T: ToString,
{
    if let Some(var) = variable {
        v.push(format!(" {} {}", name, var.to_string()));
    }
}

fn push_vec_move(v: &mut Vec<String>, name: &str, variable: &Option<Vec<Move>>){
    if let Some(moves) = variable {
        v.push(format!(" {}", name));
        for mv in moves{
            v.push(" ".to_string());
            v.push(mv.long_algebraic_notation());
        }
    }
}

fn push_vec_move_without_option(v: &mut Vec<String>, name: &str, moves: &Vec<Move>){
    v.push(format!(" {}", name));
    for mv in moves{
        v.push(" ".to_string());
        v.push(mv.long_algebraic_notation());
    }
}

fn is_file(c: char) -> bool{
    match c {
        'a'..='h' => true,
        _ => false,
    }
}
fn is_promotion(c: char) -> bool{
    match c {
        'q' | 'r' | 'k' | 'b' => true,
        _ => false,
    }
}

fn is_uci_move(mv: String) -> bool{
    if 4 <= mv.len() && mv.len() <= 5{
        if let Some(c) = mv.chars().nth(0) {
            if !c.is_alphabetic() || !is_file(c){
                return false;
            }
        }
        if let Some(c) = mv.chars().nth(1) {
            if !c.is_ascii_digit() || (c as u8 - '0' as u8) < 1 || (c as u8 - '0' as u8) > 8{
                return false;
            }
        }
        if let Some(c) = mv.chars().nth(2) {
            if !c.is_alphabetic() || !is_file(c){
                return false;
            }
        }
        if let Some(c) = mv.chars().nth(3) {
            if !c.is_ascii_digit() || (c as u8 - '0' as u8) < 1 || (c as u8 - '0' as u8) > 8{
                return false;
            }
        }
        if let Some(c) = mv.chars().nth(4) {
            if !is_promotion(c){
                return false;
            }
        }
        true
    }else {
        false
    }
}

fn get_vec_uci_moves(inp: &String, flag: &str) -> Option<Vec<String>>{
    if let Some(idx) = inp.find(flag) {
        let remaining = &inp[(idx + flag.len())..];
        let next_word: Vec<String> = remaining
            .split_whitespace()
            .take_while(|mv| is_uci_move(mv.to_string()))
            .map(|mv| mv.to_string())
            .collect();
        Some(next_word)
    } else {
        None
    }
}