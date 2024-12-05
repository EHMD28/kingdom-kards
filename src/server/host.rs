//! This module contains a set of functions for creating a server and
//! handling clients.

use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

/// Starts instance of Kingdom Kards server.
/// Hosted locally on port 5464 because 'king' - phone keypad -> '5464'
pub fn start_server() {
    let _join_code = 1234;
    let port = format!("127.0.0.1:5464");
    let listener = TcpListener::bind(port).expect("Failed to bind to port 127.0.0.1:5464");
    println!("Starting server | Join Code {{}}");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0u8; 512];

        thread::spawn(move || loop {
            stream.read(&mut buffer).unwrap();
            let bufstr = String::from_utf8_lossy(&buffer);
            if !bufstr.is_empty() {
                println!("Recieved: \"{}\"", bufstr);
                buffer.iter_mut().for_each(|v| *v = 0);
            } else {
                println!("Buffer is empty");
            }
            thread::sleep(Duration::from_secs(1));
        });
    }
}
