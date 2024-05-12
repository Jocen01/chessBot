use crate::{board::Board, searcher::Searcher, singlemove::Move, uci_message::UciMessage};
use rand::prelude::*;
use std::{sync::mpsc::{Receiver, RecvError, SendError, Sender}, time::Duration};

const DEFAULT_SEARCH_TIME: Duration = Duration::from_millis(3000);

pub struct UciEngine {
    board: Board,
    searcher: Searcher,
    #[allow(dead_code)]
    settings: String,
    debug: bool,
    tx: Sender<UciMessage>,
    rx: Receiver<UciMessage>
}

impl UciEngine {
    
    pub fn new(tx: Sender<UciMessage>, rx: Receiver<UciMessage>) -> UciEngine{
        UciEngine { 
            board: Board::default(),
            searcher: Searcher::new(1000000, tx.clone()), 
            settings: "".into(), 
            debug: false,
            tx,
            rx
        }
    }

    pub fn run(&mut self) -> Result<(), RecvError>{
        loop {
            let message = self.rx.recv()?;
            if let UciMessage::Quit = message{
                self.tx.send(UciMessage::Quit).err(); // dont care about error thread is quitting
                break;
            }
            if let Err(_) = self.execute(message){
                return Err(RecvError);
            }

        }
        Ok(())
    }

    pub fn execute(&mut self, message: UciMessage) -> Result<(), SendError<UciMessage>>{
        match message {
            UciMessage::Uci => {
                self.tx.send(UciEngine::uci_id())?;
                self.options().iter().for_each(|option| {
                    self.tx.send(option.clone()).unwrap();
                });
                self.tx.send(UciMessage::UciOk)?;
            },
            UciMessage::Debug(on) => {
                self.debug = on;
            },
            UciMessage::IsReady => {
                self.tx.send(UciMessage::ReadyOk)?;
            },
            UciMessage::SetOption { .. } => {
                //panic!("options not implemented")
                
            },
            UciMessage::Register { .. } => {
                todo!()
            },
            UciMessage::UciNewGame => {
                self.board = Board::default();
                // self.searcher.clear(); TODO
            },
            UciMessage::Position { fen, moves } => {
                self.board = if let Some(fe) = fen { Board::from_fen(&fe) } else { Board::default() };
                for mv in moves{
                    self.board.make_move(mv);
                    self.board.add_state_to_history();
                }
            },
            UciMessage::Go { 
                    search_moves,
                    move_time,
                    .. 
                } => {
                self.searcher.set_search_moves(search_moves);
                if let Some(millis) = move_time {
                    self.searcher.duration = Duration::from_millis(millis);
                }else {
                    self.searcher.duration = DEFAULT_SEARCH_TIME;
                }
                let mv = self.best_move();
                let res = UciMessage::BestMove { best_move: mv, ponder: None };
                self.tx.send(res)?;
            },
            UciMessage::Ponderhit => {
                panic!("not implemented")
            },
            UciMessage::Stop => {
                let mv = self.best_move();
                let res = UciMessage::BestMove { best_move: mv, ponder: None };
                self.tx.send(res)?;
            },
            UciMessage::Quit => {
                panic!("quit is not for the engine")
            }
            _ => {
                
            }
            
        }
        Ok(())
    }

    fn options(&self) -> Vec<UciMessage>{
        vec![] //no options currently implemented
    }

    fn uci_id() -> UciMessage{
        UciMessage::Id { name: UciEngine::name().into(), author: UciEngine::author().into() }
    }

    fn author() -> &'static str{
        "Jonathan Cederlund"
    }

    fn name() -> &'static str {
        "alfa-beta-bot"
    }

    fn best_move(&mut self) -> Move{
        // let mut moves = self.board.get_possible_moves_turn();
        // let mut rng = rand::thread_rng();

        // moves.shuffle(&mut rng);
        // if let Some(mv) = moves.first() {
        //     *mv
        // }else {
        //     panic!("no leagal moves")
        // }
        // let (mv, _score) = self.searcher.search(&mut self.board, 4);
        let (mv, _score) = self.searcher.iterative_deepening(&mut self.board);
        
        // if no move was found play a random move
        if mv.is_null_move(){
            let mut moves = self.board.get_possible_moves_turn();
            let mut rng = rand::thread_rng();
            moves.shuffle(&mut rng);

            // a first best move
            if let Some(mv) = moves.first() {
                return  *mv;
            }else{
                panic!("no leagal moves")
            };
        }

        // let (mv, _score) = self.searcher.search_2(&mut self.board, 4);
        // let (mv, _score) = self.searcher.search_3(&mut self.board, 4);
        // let (mv, _score) = searcher::search(&mut self.board, 4);
        mv
    }
}

// #[cfg(test)]
// mod test{
//     use std::sync::mpsc;

//     use crate::uci_message::UciMessage;

//     use super::UciEngine;

//     static SETUP_COMMANDS: [UciMessage; 3] = [
//         UciMessage::Uci,
//         UciMessage::IsReady,
//         UciMessage::UciNewGame
//     ];

//     fn setup_engine() -> UciEngine {
//         let (tx, rx) = mpsc::channel::<UciMessage>();
//         let mut engine = UciEngine::new(tx, rx);
//         SETUP_COMMANDS.iter().for_each(|msg | {
//             engine.execute(msg.clone());
//         });
//         engine
//     }

//     #[test]
//     fn crash_pos_1(){
//         let mut engine = setup_engine();
//         let msg = UciMessage::parse("position startpos moves e2e4 g8f6 e4e5 a7a5 e5f6 d7d5 f6g7 b8d7 g7h8q b7b6 h8e5 b6b5 e5d6 c8b7 d6b6 d7b6 d1g4 a8a6 g4e6 d8a8 e6b6 c7c6 b6b7 f8g7 b7a6 a8c8 a6a7 g7h6 d2d3 c8c7 a7c7 h6e3 c1e3 c6c5 e3f4 b5b4 f1e2 f7f5 g1f3 e8f8 e1g1 f8g8 c7d6 c5c4 d6h6 g8h8 h6f6 h8g8 f6h6 g8f7 f4e5 f7g8 h6h5 f5f4 f3g5 b4b3 e5c7 g8f8 b1d2 b3c2 a1b1 e7e5 g1h1 c4d3 h1g1 c2b1q g1h1 h7h6 h5g6 h6h5 h1g1 b1c1 g1h1 d3e2 h1g1 f8e7 g1h1 c1b2 h1g1 b2a2 g1h1 a2d2 h1g1 d5d4 g6d3 e2e1n d3d2".into());
//         engine.execute(msg);
//         let algebraic_moves: Vec<String> = engine.board.get_possible_moves_turn().iter().map(|mv| mv.long_algebraic_notation()).collect();
//         assert!(!algebraic_moves.contains(&"d2b2".into()));

//     }

//     #[test]
//     fn crash_pos_2(){
//         let mut engine = setup_engine();
//         let msg = UciMessage::parse("position startpos moves e2e4 e7e5 c2c3 d7d5 d1b3 d5e4 e1d1 e4e3 b3f7 e8f7 c3c4 e3d2 b2b3 d2c1q d1c1 f8a3 b1a3 c8f5 g1h3 d8d3 g2g3 d3c2 a3c2 f5c2 f1g2 b8c6 g2f1 c6d4 h1g1 g8f6 a2a3 f7g6 a3a4 g6f5 g1g2 f5g4 h3f4 g4f3 h2h4 f3g4 a4a5 g4f3 h4h5 f3g4 a1a3 e5e4 a3a1 e4e3 c4c5 e3e2 b3b4 e2f1q c1d2 f1b1 f4d3 g4h3 g3g4 h3h4 g4g5 h4h3 g2g4 h3h2 c5c6 h2h1 a1a4 h1h2 g4g1 h2h3 a5a6 h3h4 d3e1 h4h3 a4a5 h3h2 a5f5 h2h3 e1c2 h3h4 f5b5 h4h3 f2f3 h3h2 g1g4 h2h3 c6b7 h3h2 b7b8n h2h1 g4g1 h1h2".into());
//         engine.execute(msg);
//         let algebraic_moves: Vec<String> = engine.board.get_possible_moves_turn().iter().map(|mv| mv.long_algebraic_notation()).collect();
//         assert!(!algebraic_moves.contains(&"h8e8".into()));
//     }

    
    
//     // #[test]
//     // fn scholars_mate_in_one(){
//     //     let mut engine = setup_engine();
//     //     let msg = UciMessage::parse("position startpos moves e2e3 b8a6 f1c4 a6b8 d1f3 b8a6".into());
//     //     engine.execute(msg);
//     //     if let Some(vec_msg) = engine.execute(UciMessage::parse("go".into())){
//     //         assert_eq!(vec_msg.len(), 1);
//     //         let msg = vec_msg.first().unwrap();
//     //         match msg {
//     //             UciMessage::BestMove { best_move, ponder: _ } => {
//     //                 assert_eq!(best_move.to(), 53)
//     //             },
//     //             _ => panic!("go doesnt return best move command")
//     //         }
//     //     }
//     // }

//     // #[test]
//     // fn nbr_eval(){
//     //     let mut engine = setup_engine();
//     //     let msg = UciMessage::parse("position startpos moves e2e4 c7c6 f1c4 h7h5 d1f3 e7e6 e1f1 f8e7 f3e2 h8h7 c4b3 e7h4 g2g3 h4f6 c2c3 e8e7 f1g2 g7g5 b3c4 d8b6 a2a4".into());
//     //     engine.execute(msg);
//     //     if let Some(_vec_msg) = engine.execute(UciMessage::parse("go".into())){
            
//     //         println!("{}",engine.searcher.searches);
//     //         // assert!(1==2);
//     //     }
//     // }

//     // #[test]
//     // fn casle_while_in_check(){
//     //     let mut engine = setup_engine();
//     //     let msg = UciMessage::parse("position startpos moves d2d4 e7e6 b1c3 b8c6 e2e4 f8b4 c1e3 g8f6 d4d5 e6d5 e4d5 c6e5 g1f3 f6g4 e3d4 e5f3 d1f3 b4c3 b2c3 d8e7".into());
//     //     engine.execute(msg);
//     //     if let Some(vec_msg) = engine.execute(UciMessage::parse("go".into())){
//     //         assert!(vec_msg.len() >= 1);
//     //         let msg = vec_msg.first().unwrap();
//     //         match msg {
//     //             UciMessage::BestMove { best_move, ponder: _ } => {
//     //                 assert_ne!(best_move.move_type(), MoveType::Castle);
//     //                 println!("{}", best_move);

//     //             },
//     //             _ => { panic!() }
//     //         }
//     //         println!("{}",engine.searcher.searches);
//     //         // assert!(1==2);
//     //     }
//     //     // r1b1k2r/pppp1ppp/5n2/3Pq3/2B5/2b1B3/PPP2PPP/R2QK2R w KQkq - 0 10
//     //     // r1b1k2r/ppppqppp/8/3P4/3B2n1/2P2Q2/P1P2PPP/R3KB1R w KQkq - 1 11
//     // }

//     // #[test]
//     // fn null_move_iterative(){
//     //     // 2rqkb1r/p3ppp1/3n1n1p/3p1b2/1p1pP3/1PP2P2/P1Q1N1PP/RNBK1B1R b k - 0 16
//     //     // position startpos moves c2c3 d7d5 d2d4 b8c6 d1d3 c8e6 d3b5 a8b8 b5a4 g8f6 e1d2 b7b5 a4b3 h7h6 f2f3 c6a5 b3c2 a5c4 d2d1 b8c8 b2b3 c4d6 b1a3 c7c5 e2e3 b5b4 a3b1 c5d4 g1e2 e6f5 e3e4
//     //     let mut engine = setup_engine();
//     //     let msg = UciMessage::parse("position startpos moves c2c3 d7d5 d2d4 b8c6 d1d3 c8e6 d3b5 a8b8 b5a4 g8f6 e1d2 b7b5 a4b3 h7h6 f2f3 c6a5 b3c2 a5c4 d2d1 b8c8 b2b3 c4d6 b1a3 c7c5 e2e3 b5b4 a3b1 c5d4 g1e2 e6f5 e3e4".into());
//     //     engine.execute(msg);
//     //     if let Some(vec_msg) = engine.execute(UciMessage::parse("go".into())){
//     //         assert!(vec_msg.len() >= 1);
//     //         let msg = vec_msg.first().unwrap();
//     //         match msg {
//     //             UciMessage::BestMove { best_move, ponder: _ } => {
//     //                 assert!(!best_move.is_null_move());

//     //             },
//     //             _ => { panic!() }
//     //         }
//     //     }
//     // }
// }

// position startpos moves e2e4 e7e5 d2d4 e5d4 g1f3 f8c5 c2c3 d4c3 b1c3 b8c6 c1g5 g8f6 e4e5 d8e7
// it takes about 1.3 seconds to find the best move and has to reach depth 4