//! This module contains a set of functions for client side commmunication
//! with the server.

use std::{io::Write, net::TcpStream, thread, time::Duration};

/// Returns a list of available Kingdom Kards servers.
pub fn find_servers() {
    println!("Finding servers...");
}

pub fn choose_server() -> &'static str {
    ""
}

pub fn connect_to_server(port: &str) {
    if let Ok(mut stream) = TcpStream::connect(port) {
        loop {
            stream.write(b"client side connection established").unwrap();
            println!("Connected to server");
            thread::sleep(Duration::from_secs(1));
        }
    } else {
        panic!("Failed to connect to server");
    }
}
