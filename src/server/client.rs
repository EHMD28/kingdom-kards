//! This module contains a set of functions for client side commmunication
//! with the server.

use core::panic;
use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use crate::server::ServerError;

use super::utils::{get_input, get_response};

pub struct ClientInstance {}

pub fn connect_to_server(port: &str) -> Result<TcpStream, ServerError> {
    if let Ok(stream) = TcpStream::connect(port) {
        println!("Connected To Server");
        Ok(stream)
    } else {
        Err(ServerError::FailedToConnect(String::from(port)))
    }
}

pub enum ClientError {
    InvalidCharacterFound,
    CommaFound,
}

pub fn choose_player_name() -> Result<String, self::ClientError> {
    let name = get_input("Enter a user name: ");

    if !name.is_ascii() {
        Err(ClientError::InvalidCharacterFound)
    } else if name.contains(",") {
        Err(ClientError::CommaFound)
    } else {
        return Ok(name);
    }
}

pub fn try_connect() -> Option<TcpStream> {
    loop {
        match connect_to_server("127.0.0.1:5464") {
            Ok(stream) => {
                println!("Successfully connected to server");
                return Some(stream);
            }
            Err(e) => {
                println!("{e}");
                io::stdout().flush().expect("unable to flush stdout");
                let input = get_input("Try again [y/n]: ").to_lowercase();

                if input == "n" || input == "no" {
                    println!("Okay. Closing application");
                    return None;
                } else {
                    println!("Trying again...");
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
    }
}

pub fn send_join_request(stream: &mut TcpStream, name: &str) {
    let message = format!("JOIN,{name}");
    let _ = stream.write(message.as_bytes()).unwrap();

    let response = get_response(stream);

    if response == "REJECT" {
        todo!();
    } else if response == "ACCEPT" {
        println!("Successfully joined server");
    } else {
        println!("Recieved invalid response: \"{response}\"");
        panic!();
    }
}
