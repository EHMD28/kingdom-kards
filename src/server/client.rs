//! This module contains a set of functions for client side commmunication
//! with the server.

use std::{io::Write, net::TcpStream};

use crate::server::ServerError;

use super::utils::get_input;

pub fn choose_server() -> &'static str {
    ""
}

pub fn connect_to_server(port: &str) -> Result<TcpStream, ServerError> {
    if let Ok(mut stream) = TcpStream::connect(port) {
        let _ = stream.write(b"Client Side Connection Established").unwrap();
        println!("Connected To Server");
        Ok(stream)
    } else {
        Err(ServerError::FailedToConnect(String::from(port)))
    }
}

pub enum Error {
    InvalidCharacterFound,
    CommaFound,
}

pub fn get_player_name() -> Result<String, self::Error> {
    let name = get_input("Enter a user name: ");

    if !name.is_ascii() {
        Err(Error::InvalidCharacterFound)
    } else if name.contains(",") {
        Err(Error::CommaFound)
    } else {
        return Ok(name);
    }
}
