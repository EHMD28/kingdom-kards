//! This module contains a set of functions for creating a server and
//! handling clients.

use core::panic;
use std::{
    fmt,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::game::game_state::{GameState, PlayerDetails};

use super::utils::is_zeroed;

pub struct ServerInstance {
    game_state: GameState,
    listener: TcpListener,
}

enum JoinResponse {
    Accept(String),
    Reject,
}

enum RequestType {
    Join,
    Act,
}

impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestType::Join => write!(f, "RequestType::Join"),
            RequestType::Act => write!(f, "RequestType::Act"),
        }
    }
}

impl FromStr for RequestType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();
        let first = parts.first().unwrap();
        let first = *first;

        match first {
            "JOIN" => Ok(RequestType::Join),
            "ACT" => Ok(RequestType::Act),
            _ => panic!("Invalid conversion from string to RequestType"),
        }
    }
}

impl ServerInstance {
    /// Starts instance of Kingdom Kards server.
    /// Hosted locally on port 5464 because 'king' - phone keypad -> '5464'
    pub fn create() -> Arc<Mutex<ServerInstance>> {
        let _join_code = 1234;
        let port = "127.0.0.1:5464".to_string();
        let listener = TcpListener::bind(port).expect("Failed to bind to port 127.0.0.1:5464");
        println!("Starting server with join code: {_join_code}");

        Arc::new(Mutex::new(ServerInstance {
            game_state: GameState::new(),
            listener,
        }))
    }

    pub fn start(server: Arc<Mutex<ServerInstance>>) {
        for stream in server.lock().unwrap().listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream, server.clone());
        }
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

    // pub fn is_unique_name(&self, name: &str) -> bool {
    //     let all_players = self.get_game_state().get_all_players();
    //     for player in all_players.iter() {
    //         if player.get_name() == name {
    //             return false;
    //         }
    //     }

    //     true
    // }
}

fn handle_connection(mut stream: TcpStream, server: Arc<Mutex<ServerInstance>>) {
    thread::spawn(move || loop {
        let mut buffer = [0u8; 512];
        let _ = stream.read(&mut buffer).unwrap();

        if !is_zeroed(&buffer) {
            let received = String::from_utf8_lossy(&buffer).to_string();
            let received = received.trim_matches('\0');
            println!("Recieved: \"{received}\"");
            let request_type = RequestType::from_str(received).unwrap();
            println!("Request Type: {request_type}");
            match request_type {
                RequestType::Join => {
                    println!("Recieved join request");

                    let join_response = match handle_join(received, Arc::clone(&server)) {
                        JoinResponse::Accept(name) => {
                            println!("Accepted player\n");
                            server
                                .lock()
                                .unwrap()
                                .game_state
                                .add_player(PlayerDetails::new(name, 100));
                            "ACCEPT"
                        }
                        JoinResponse::Reject => {
                            println!("Rejected player\n");
                            "REJECT"
                        }
                    };

                    println!("Writing join response: {join_response}");
                    let _ = stream.write(join_response.as_bytes()).unwrap();
                }
                RequestType::Act => {
                    todo!()
                }
            }

            buffer.iter_mut().for_each(|v| *v = 0);
        }

        thread::sleep(Duration::from_secs(1));
    });
}

fn handle_join(request: &str, server: Arc<Mutex<ServerInstance>>) -> JoinResponse {
    println!("Handling join request: \"{request}\"");

    let parts: Vec<&str> = request.split(",").collect();

    if parts.len() != 2 {
        panic!("Invalid request format for JOIN request");
    }

    /* The first part is ignored because it is checked elsewhere */
    let name = *parts.get(1).unwrap();

    println!("Checkpoint One");
    let is_unique = server.lock().unwrap().game_state.is_unique_name(name);
    println!("Checkpoint Two");

    if !is_unique {
        println!("Name is not unique");
        JoinResponse::Reject
    } else {
        println!("Name is unique");
        JoinResponse::Accept(name.to_string())
    }
}
