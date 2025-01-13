//! This module contains a set of functions for creating a server and
//! handling clients.

use std::{
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::{
    game::game_state::{GameState, PlayerDetails},
    utils::variant_eq,
};

use super::{
    request::{Request, RequestType, NAME_REQUEST, STATUS_REQUEST},
    response::{
        ResponseType, NAME_RESPONSE, STATUS_RESPONSE, STATUS_RESPONSE_NO, STATUS_RESPONSE_YES,
    },
    StreamHandler,
};

pub struct ServerInstance {
    game_state: GameState,
    listener: TcpListener,
    clients: Vec<(TcpStream, SocketAddr)>,
    join_code: String,
}

impl ServerInstance {
    /// Starts instance of Kingdom Kards server.
    /// Hosted locally on port 5464 because 'king' - phone keypad -> '5464'
    pub fn create() -> ServerInstance {
        let port = "127.0.0.1:5464".to_string();
        let listener = TcpListener::bind(port).expect("Failed to bind to port 127.0.0.1:5464");

        ServerInstance {
            game_state: GameState::new(),
            listener,
            clients: Vec::new(),
            join_code: "1234".to_string(),
        }
    }

    /// Returns a reference to the member `game_state` of this struct.
    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    /// Starts up server operations. First, the server accepts the number
    /// of players (between 2 and 6). Next, players will enter their usernames
    /// and the server will validate that the names are unique. After that, the
    /// core gameplay loop starts.
    pub fn start(&mut self) {
        // let num_players = get_num_input("Enter number of players (min. 2, max. 6): ", 2, 6);

        // TODO: change back for full application
        self.accept_players(1);

        println!("Starting server with join code: {}", self.join_code);

        self.name_players();
    }

    /// Allows `num_players` clients to join, exits after all players have joined.
    fn accept_players(&mut self, num_players: u8) {
        let mut num_connections = 0;

        println!("Accepting players...");
        while num_connections < num_players {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    println!("New client joined from {addr}");
                    self.clients.push((stream, addr));
                    num_connections += 1;
                }
                Err(e) => {
                    println!("Failed to connect. Error: {e}");
                }
            }
        }
    }

    /// Prompts every user to enter a username and verifies that each username is unique.
    fn name_players(&mut self) {
        for (client, _) in self.clients.iter_mut() {
            let client_handler = &mut StreamHandler::new(client);
            if let Err(e) = client_handler.send_request(NAME_REQUEST) {
                eprintln!("An error occured while sending request: {e}");
            }

            let name_response = client_handler.await_response(NAME_RESPONSE);
            let name = match name_response {
                Ok(response) => {
                    if let ResponseType::Name(name) = response.response_type() {
                        name.as_ref().unwrap().to_owned()
                    } else {
                        unreachable!("Received response of incorrect type");
                    }
                }
                Err(err) => {
                    eprintln!("An error occured in name_players(): {err}");
                    String::new()
                }
            };

            let status_request = client_handler.await_request(STATUS_REQUEST);
            match status_request {
                Ok(request) => {
                    if variant_eq(request.request_type(), &RequestType::Status) {
                        if self.game_state.is_unique_name(&name) {
                            let new_player = PlayerDetails::new(name, 100);
                            self.game_state.add_player(new_player);

                            if let Err(err) = client_handler.send_response(STATUS_RESPONSE_YES) {
                                eprintln!("An error occured in name_players(): {err}");
                            }
                        } else if let Err(err) = client_handler.send_response(STATUS_RESPONSE_NO) {
                            eprintln!("An error occured in name_players(): {err}");
                        }
                    } else {
                        unreachable!("Received an invalid type");
                    }
                }
                Err(err) => eprintln!("An error occured in name_players(): {err}"),
            }
        }
    }

    /// Starts core gameplay loop.
    fn game_loop(&self) {
        todo!()
    }

    /// This function is for testing purposes only. It blocks the main thread in an
    /// infinite loop to prevent the program from immediately exiting.
    pub fn _wait(&self) {
        println!("Server is waiting");

        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
