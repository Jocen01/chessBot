use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use crate::uci_message::UciMessage;

macro_rules! read_str {
    ($out:ident) => {
        #[allow(unused_mut)]
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

pub fn new_uci_in_tread() -> (JoinHandle<()>, Receiver<UciMessage>){
    let (tx, rx) = mpsc::channel::<UciMessage>();
    let join = thread::spawn(move || {
        loop {
            read_str!(msg_str);
            let msg = UciMessage::parse(msg_str.into());
            if let UciMessage::Quit = msg {
                tx.send(UciMessage::Quit).err(); // can ignore error since the thread is quiting
                break;
            }
            if let Err(_) = tx.send(msg) {
                let _ = tx.send(UciMessage::Quit);
                break;
            };
        }
    });
    (join, rx)
}

pub fn new_uci_out_tread() -> (JoinHandle<()>, Sender<UciMessage>){
    let (tx, rx) = mpsc::channel::<UciMessage>();
    let join = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    match msg {
                        UciMessage::Quit => {
                            break;
                        },
                        _ => {
                            println!("{}", msg.serialize());
                        }
                    }  
                },
                Err(_) => {
                    break;
                }
            }
        }
    });
    (join, tx)
}