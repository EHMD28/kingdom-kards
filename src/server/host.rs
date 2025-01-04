//! This module contains a set of functions for creating a server and
//! handling clients.

use std::{
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::game::game_state::{GameState, PlayerDetails};

use super::{
    request::{await_request, send_request, Request, RequestType},
    response::{await_response, send_response, Response, ResponseType, StatusType},
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

    /// Returns a mutable ference to the member `game_state` of this struct.
    pub fn game_state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
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
            if let Err(e) = send_request(client, Request::new(RequestType::Name)) {
                eprintln!("An error occured while sending request: {e}");
            }
            let response = await_response(client, ResponseType::Name(String::default()));

            match response {
                Ok(response) => match response.response_type() {
                    ResponseType::Name(name) => {
                        let request = await_request(client, RequestType::Status);
                        match request {
                            Ok(request) => {
                                if let RequestType::Status = request.request_type() {
                                } else {
                                    panic!("Received invalid type");
                                }
                            }
                            Err(err) => eprintln!("An error occured in name_players(): {err}"),
                        }

                        if self.game_state.is_unique_name(name) {
                            self.game_state
                                .add_player(PlayerDetails::new(name.to_owned(), 100));
                            let response = send_response(
                                client,
                                Response::new(ResponseType::Status(StatusType::Yes)),
                            );
                            if let Err(err) = response {
                                eprintln!("An error occured in name_players(): {err}.");
                            }
                        } else {
                            let response = send_response(
                                client,
                                Response::new(ResponseType::Status(StatusType::No)),
                            );
                            if let Err(err) = response {
                                eprintln!("An error occured in name_players(): {err}");
                            }
                        }
                    }
                    _ => panic!("Received an invalid type"),
                },
                Err(err) => {
                    eprintln!("An error occured in name_players(): {err}");
                }
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
