//! This module contains a set of functions for client side commmunication
//! with the server.

use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use crate::server::utils::get_input;
use crate::server::ServerError;

use super::request::{await_request, Request, RequestType};
use super::response::{send_response, Response, ResponseType};

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

    pub fn start(&mut self) {
        self.choose_player_name();
    }

    pub fn choose_player_name(&mut self) {
        let stream = self.stream.as_mut().unwrap();
        let request = await_request(stream, RequestType::Name);

        match request {
            Ok(request) => {
                match request.request_type() {
                    RequestType::Name => (),
                    /* It should be impossible to recieve a request of the wrong type. */
                    _ => panic!("Recieved request of incorrect type."),
                }
            }
            Err(err) => {
                eprintln!("ServerError: {err}");
                return;
            }
        }

        let name = get_input("Enter a username: ");
        if !name.is_ascii() {
            println!("Error! Username contains invalid character (e.g. ç or ♥︎).");
        } else if name.contains(",") {
            println!("Error! Username cannot contain commas.");
        } else {
            let status = send_response(stream, Response::new(ResponseType::Name(name)));
            if status.is_err() {
                eprintln!("An error occured when sending NAME response");
            }
        }
    }

    pub fn _do_something(&mut self) {
        let stream = self.stream.as_mut().unwrap();
        let _ = send_response(
            stream,
            Response::new(ResponseType::Name("John Smith".to_string())),
        );
    }

    pub fn _wait(&mut self) {
        println!("Client is waiting");

        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
