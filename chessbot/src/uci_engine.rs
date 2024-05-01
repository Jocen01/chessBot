use crate::{board::Board, searcher, singlemove::Move, uci_message::UciMessage};


pub struct UciEngine {
    board: Board,
    #[allow(dead_code)]
    settings: String,
    debug: bool
}

impl UciEngine {
    
    pub fn new() -> UciEngine{
        UciEngine { board: Board::default(), settings: "".into(), debug: false }
    }

    pub fn execute(&mut self, message: UciMessage) -> Option<Vec<UciMessage>>{
        match message {
            UciMessage::Uci => {
                let mut res = vec![];
                res.push(UciEngine::uci_id());
                self.options().iter().for_each(|option| res.push(option.clone()));
                res.push(UciMessage::UciOk);
                Some(res)
            },
            UciMessage::Debug(on) => {
                self.debug = on;
                None
            },
            UciMessage::IsReady => {
                Some(vec![UciMessage::ReadyOk])
            },
            UciMessage::SetOption { .. } => {
                panic!("options not implemented")
            },
            UciMessage::Register { .. } => {
                todo!()
            },
            UciMessage::UciNewGame => {
                self.board = Board::default();
                None
            },
            UciMessage::Position { fen, moves } => {
                self.board = if let Some(fe) = fen { Board::from_fen(&fe) } else { Board::default() };
                for mv in moves{
                    self.board.make_move(mv);
                }
                None
            },
            UciMessage::Go { .. } => {
                let mv = self.best_move();
                let res = UciMessage::BestMove { best_move: mv, ponder: None };
                Some(vec![res])
            },
            UciMessage::Ponderhit => {
                panic!("not implemented")
            },
            UciMessage::Stop => {
                let mv = self.best_move();
                let res = UciMessage::BestMove { best_move: mv, ponder: None };
                Some(vec![res])
            },
            UciMessage::Quit => {
                panic!("quit is not for the engine")
            }
            _ => {
                panic!()
            }
        }
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
        let (mv, _score) = searcher::search(&mut self.board, 5);
        mv
    }
}

#[cfg(test)]
mod test{
    use crate::uci_message::UciMessage;

    use super::UciEngine;

    static SETUP_COMMANDS: [UciMessage; 3] = [
        UciMessage::Uci,
        UciMessage::IsReady,
        UciMessage::UciNewGame
    ];

    #[test]
    fn crash_pos_1(){
        let mut engine = UciEngine::new();
        SETUP_COMMANDS.iter().for_each(|msg | {
            engine.execute(msg.clone());
        });
        let msg = UciMessage::parse("position startpos moves e2e4 g8f6 e4e5 a7a5 e5f6 d7d5 f6g7 b8d7 g7h8q b7b6 h8e5 b6b5 e5d6 c8b7 d6b6 d7b6 d1g4 a8a6 g4e6 d8a8 e6b6 c7c6 b6b7 f8g7 b7a6 a8c8 a6a7 g7h6 d2d3 c8c7 a7c7 h6e3 c1e3 c6c5 e3f4 b5b4 f1e2 f7f5 g1f3 e8f8 e1g1 f8g8 c7d6 c5c4 d6h6 g8h8 h6f6 h8g8 f6h6 g8f7 f4e5 f7g8 h6h5 f5f4 f3g5 b4b3 e5c7 g8f8 b1d2 b3c2 a1b1 e7e5 g1h1 c4d3 h1g1 c2b1q g1h1 h7h6 h5g6 h6h5 h1g1 b1c1 g1h1 d3e2 h1g1 f8e7 g1h1 c1b2 h1g1 b2a2 g1h1 a2d2 h1g1 d5d4 g6d3 e2e1n d3d2".into());
        engine.execute(msg);
        let algebraic_moves: Vec<String> = engine.board.get_possible_moves_turn().iter().map(|mv| mv.long_algebraic_notation()).collect();
        assert!(!algebraic_moves.contains(&"d2b2".into()));

    }

    #[test]
    fn crash_pos_2(){
        let mut engine = UciEngine::new();
        SETUP_COMMANDS.iter().for_each(|msg | {
            engine.execute(msg.clone());
        });
        let msg = UciMessage::parse("position startpos moves e2e4 e7e5 c2c3 d7d5 d1b3 d5e4 e1d1 e4e3 b3f7 e8f7 c3c4 e3d2 b2b3 d2c1q d1c1 f8a3 b1a3 c8f5 g1h3 d8d3 g2g3 d3c2 a3c2 f5c2 f1g2 b8c6 g2f1 c6d4 h1g1 g8f6 a2a3 f7g6 a3a4 g6f5 g1g2 f5g4 h3f4 g4f3 h2h4 f3g4 a4a5 g4f3 h4h5 f3g4 a1a3 e5e4 a3a1 e4e3 c4c5 e3e2 b3b4 e2f1q c1d2 f1b1 f4d3 g4h3 g3g4 h3h4 g4g5 h4h3 g2g4 h3h2 c5c6 h2h1 a1a4 h1h2 g4g1 h2h3 a5a6 h3h4 d3e1 h4h3 a4a5 h3h2 a5f5 h2h3 e1c2 h3h4 f5b5 h4h3 f2f3 h3h2 g1g4 h2h3 c6b7 h3h2 b7b8n h2h1 g4g1 h1h2".into());
        engine.execute(msg);
        let algebraic_moves: Vec<String> = engine.board.get_possible_moves_turn().iter().map(|mv| mv.long_algebraic_notation()).collect();
        assert!(!algebraic_moves.contains(&"h8e8".into()));

    }
}