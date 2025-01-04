//! This module contains a set of functions for client side commmunication
//! with the server.

use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use crate::server::utils::get_input;
use crate::server::ServerError;

use super::request::{await_request, send_request, Request, RequestType};
use super::response::{self, await_response, send_response, Response, ResponseType, StatusType};

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

            let status = send_request(stream, Request::new(RequestType::Status));
            if let Err(err) = status {
                eprintln!("An error occured in choose_player_name(): {err}");
            }

            let response = await_response(stream, ResponseType::Status(StatusType::No));
            match response {
                Ok(response) => {
                    if let ResponseType::Status(status) = response.response_type() {
                        match status {
                            StatusType::Yes => println!("Name was accepted by server."),
                            StatusType::No => println!("Name was rejected by server."),
                        }
                    } else {
                        panic!("Received response of invalid type.");
                    }
                }
                Err(err) => eprintln!("An error occured in choose_player_name(): {err}"),
            }
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
