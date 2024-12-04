//! This module contains a set of functions for creating a server and
//! handling clients.

use std::{io::Write, net::TcpListener, thread};

/// Starts instance of Kingdom Kards server.
/// Hosted locally on port 5464 because 'king' - phone keypad -> '5464'
pub fn start_server() {
    let _join_code = 1234;
    let port = format!("127.0.0.1:5464");
    let listener = TcpListener::bind(port).expect("Failed to bind to port 127.0.0.1:5464");
    println!("Starting server | Join Code {{}}");

    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            stream.write(b"Hello, World\n").unwrap();
        });
    }
}
