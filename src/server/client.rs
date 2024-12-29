//! This module contains a set of functions for client side commmunication
//! with the server.

use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use crate::server::ServerError;

use super::utils::{get_input, get_response};

pub enum ClientError {
    InvalidCharacterFound,
    CommaFound,
}

pub struct ClientInstance {
    stream: Option<TcpStream>,
    name: Option<String>,
}

impl ClientInstance {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ClientInstance {
        ClientInstance {
            stream: None,
            name: None,
        }
    }

    /// This function connects client instance to server at `port`. If the client is unable to
    /// connect to the server initially, then the program will prompt the user to try again. If
    /// they answer no, then the program will exit. If they answer yes, then the client will
    /// attempt to connect to the server again.
    ///
    /// ## Returns  
    ///
    /// This function will return `None` if the server fails to connect. Otherwise, it will return
    /// `Some(())`
    pub fn connect_to_server(&mut self, port: &str) -> Option<()> {
        loop {
            if let Ok(stream) = TcpStream::connect(port) {
                self.stream = Some(stream);
                return Some(());
            } else {
                let error = ServerError::FailedToConnect(String::from(port));
                println!("{error}");
                io::stdout().flush().expect("Unable to flush stdout");

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

    pub fn choose_player_name(&mut self) {
        let name = loop {
            let username = get_input("Enter a username: ");

            if !username.is_ascii() {
                println!("Error! Username contains invalid character (e.g. ç or ♥︎)");
            } else if username.contains(",") {
                println!("Error! Username cannot contain commas");
            } else {
                let response = self.send_join_request(&username);
                /* Name was rejected by server */
                if response.is_none() {
                    println!("Name was rejected by server");
                } else {
                    break Some(username);
                }
            }
        };

        println!(
            "Successfully joined server as \"{}\"",
            name.as_ref().unwrap()
        );
        self.name = name;
    }

    fn send_join_request(&mut self, name: &str) -> Option<()> {
        let message = format!("JOIN,{name}");
        let stream = self.stream.as_mut().unwrap();

        let _ = stream.write(message.as_bytes()).unwrap();

        let response = get_response(stream);

        if response == "REJECT" {
            None
        } else if response == "ACCEPT" {
            Some(())
        } else {
            panic!("Invalid join response");
        }
    }

    pub fn _wait(&mut self) {
        let msg = "hello".as_bytes();
        let stream = self.stream.as_mut().unwrap();

        loop {
            let _ = stream.write(msg).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    }
}
