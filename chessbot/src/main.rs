use std::error;
use engine::UciEngine;
use uci::uciio;

mod board;
mod constants;
mod engine;
mod uci;
mod movegeneration;
mod utils;




fn main() -> Result<(), Box<dyn error::Error>> {
    movegeneration::setup(); //set up magics, not needed but will speed up the first movegeneration a bit by not having to do it during the first search
    let (thread_in,rx) = uciio::new_uci_in_tread();
    let (thread_out, tx) = uciio::new_uci_out_tread();
    let mut engine = UciEngine::new(tx, rx);
    engine.run()?;
    thread_in.join().expect("something went wrong");
    thread_out.join().expect("something went wrong");
    Ok(())
}




