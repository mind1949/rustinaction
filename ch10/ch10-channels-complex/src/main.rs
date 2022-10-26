#[macro_use]
extern crate crossbeam;

use crossbeam::channel::unbounded;
use std::thread;

use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

fn main() {
    let n_messages = 3;
    let (request_tx, request_rx) = unbounded();
    let (response_tx, response_rx) = unbounded();

    thread::spawn(move || loop {
        match request_rx.recv().unwrap() {
            Pong => eprintln!("unexpected pong response"),
            Ping => response_tx.send(Pong).unwrap(),
            Pang => return,
        }
    });

    for _ in 0..n_messages {
        request_tx.send(Ping).unwrap();
    }
    request_tx.send(Pang).unwrap();

    for _ in 0..n_messages {
        select! {
            recv(response_rx) -> msg => println!("{:?}", msg),
        }
    }
}
