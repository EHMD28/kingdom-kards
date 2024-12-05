//! This module contains a set of functions for client side commmunication
//! with the server.

use std::{io::Write, net::TcpStream, thread, time::Duration};

use crate::server::ServerError;

pub fn choose_server() -> &'static str {
    ""
}

pub fn connect_to_server(port: &str) -> ServerError {
    if let Ok(mut stream) = TcpStream::connect(port) {
        loop {
            let _ = stream.write(b"client side connection established").unwrap();
            println!("Connected to server");
            thread::sleep(Duration::from_secs(1));
        }
    } else {
        ServerError::FailedToConnect(String::from(port))
    }
}
