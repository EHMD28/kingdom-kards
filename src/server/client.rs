//! This module contains a set of functions for client side commmunication
//! with the server.

use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use crate::server::request::{Request, NAME_REQUEST, STATUS_REQUEST};
use crate::server::response::{Response, ResponseType, StatusType, STATUS_RESPONSE};
use crate::server::utils::get_input;
use crate::server::ServerError;
use crate::utils::variant_eq;

use super::request::RequestType;
use super::StreamHandler;

/// This is the type used for representing client-side errors.
pub enum ClientError {
    InvalidCharacterFound,
    CommaFound,
}

/// This is the type used for representing a single client instance.
/// There should only be one `ClientInstance` per running process of
/// `kingdom-kards`.
pub struct ClientInstance {
    stream: Option<TcpStream>,
    name: Option<String>,
}

impl ClientInstance {
    /// Creates a new client instance with parameters uninitialized.
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
                eprintln!("{error}");
                io::stdout().flush().expect("Unable to flush stdout");

                const MAX_INPUT_LEN: usize = 5;
                let input = get_input("Try again [y/n]: ", MAX_INPUT_LEN).to_lowercase();

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

    /// Starts the the gameplay loop client side. First, client must choose a username,
    /// then, they will start the actual game. `connect_to_server()` must be called before
    /// this function is called.
    ///
    /// # Panics
    ///
    /// This function will panic if there is no connection to the server
    /// (i.e. `connect_to_server()`) was not called or it failed.
    pub fn start(&mut self) {
        self.choose_player_name();
    }

    /// This function is used to initiate communcation with the server so the
    /// player can choose a unique username.
    ///
    /// # Panics
    ///
    /// This function will panic if an invalid type is received from the server,
    /// but that should be impossible because `await_request()` and `await_response()`
    /// both check what type is received.
    pub fn choose_player_name(&mut self) {
        let stream = self.stream.as_mut().unwrap();
        let stream_handler = &mut StreamHandler::new(stream);

        let name_request = stream_handler.await_request(NAME_REQUEST);
        Request::validate(name_request, RequestType::Name);

        const MAX_INPUT_LEN: usize = 25;
        let name = loop {
            let name = get_input("Enter a username: ", MAX_INPUT_LEN);
            if ClientInstance::validate_name_input(&name).is_ok() {
                break name;
            }
        };

        let name_response = Response::new(ResponseType::Name(Some(name)));
        if let Err(err) = stream_handler.send_response(name_response) {
            eprintln!("An error occured in choose_player_name(): {err}");
        }

        if let Err(err) = stream_handler.send_request(STATUS_REQUEST) {
            eprintln!("An error occured in choose_player_name(): {err}");
        }

        let status_response = stream_handler.await_response(STATUS_RESPONSE);
        match status_response {
            Ok(response) => {
                if let ResponseType::Status(status) = response.response_type() {
                    match status {
                        Some(StatusType::Yes) => println!("Name was accepted by server."),
                        Some(StatusType::No) => println!("Name was rejected by server."),
                        None => unreachable!("Status was empty"),
                    }
                } else {
                    unreachable!("Received invalid response type");
                }
            }
            Err(err) => eprintln!("An error occured in choose_player_name(): {err}"),
        }
    }

    fn validate_name_input(name: &str) -> Result<(), ()> {
        if !name.is_ascii() {
            println!("Error! Username cannot contain invalid characters (e.g. ç or ♥︎).");
            Err(())
        } else if name.contains(",") {
            println!("Error! Username cannot contain commas");
            Err(())
        } else {
            Ok(())
        }
    }

    /// This function is for testing purposes only. It blocks the main thread in an
    /// infinite loop to prevent the program from immediately exiting.
    pub fn _wait(&self) {
        println!("Client is waiting");

        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
