//! This module contains a set of functions for creating a server and
//! handling clients.

use core::panic;
use std::{
    fmt,
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    thread,
    time::Duration,
};

use crate::{game::game_state::GameState, server::utils::get_input};

use super::utils::is_zeroed;

pub struct ServerInstance {
    game_state: GameState,
    listener: TcpListener,
    clients: Vec<(TcpStream, SocketAddr)>,
    join_code: String,
}

// enum JoinResponse {
//     Accept(String),
//     Reject,
// }

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
    pub fn create() -> ServerInstance {
        let port = "127.0.0.1:5464".to_string();
        let listener = TcpListener::bind(port).expect("Failed to bind to port 127.0.0.1:5464");

        ServerInstance {
            game_state: GameState::new(),
            listener,
            clients: Vec::new(),
            join_code: 1234.to_string(),
        }
    }

    pub fn start(&mut self) {
        let num_players: u8 = loop {
            let result = get_input("Enter number of players (max. 6): ").parse::<u8>();

            if let Ok(num_players) = result {
                if num_players > 6 {
                    println!("Too many players. Maximum is 6.");
                } else if num_players <= 1 {
                    println!("Two few players. Minimum is 2.");
                } else {
                    break num_players;
                }
            } else {
                println!("Invalid input. Try again.");
            }
        };

        self.accept_players(num_players);

        println!("Starting server with join code: {}", self.join_code);

        loop {
            self.handle_connections();
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn accept_players(&mut self, num_players: u8) {
        let mut num_connections = 0;

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

    fn handle_connections(&mut self) {
        for (stream, _) in self.clients.iter_mut() {
            ServerInstance::handle_connection(stream);
        }
    }

    fn handle_connection(stream: &mut TcpStream) {
        let mut buffer = [0u8; 512];
        let _ = stream.read(&mut buffer).unwrap();

        if !is_zeroed(&buffer) {
            let receieved = String::from_utf8_lossy(&buffer).to_string();

            println!("Recieved: {}", receieved);
        }

        // for (stream, _) in self.clients.iter_mut() {
        //     let msg = "Connect to server";
        //     let _ = stream.write(msg.as_bytes()).unwrap();
        // }
    }

    // fn handle_connection(stream: TcpStream, server: ServerInstance) {
    // thread::spawn(move || loop {
    //     let mut buffer = [0u8; 512];
    //     let _ = stream.read(&mut buffer).unwrap();

    //     if !is_zeroed(&buffer) {
    //         let received = String::from_utf8_lossy(&buffer).to_string();
    //         let received = received.trim_matches('\0');
    //         println!("Recieved: \"{received}\"");
    //         let request_type = RequestType::from_str(received).unwrap();
    //         println!("Request Type: {request_type}");
    //         match request_type {
    //             RequestType::Join => {
    //                 println!("Recieved join request");

    //                 let join_response = match handle_join(received, Arc::clone(&server)) {
    //                     JoinResponse::Accept(name) => {
    //                         println!("Accepted player\n");
    //                         server
    //                             .lock()
    //                             .unwrap()
    //                             .game_state
    //                             .add_player(PlayerDetails::new(name, 100));
    //                         "ACCEPT"
    //                     }
    //                     JoinResponse::Reject => {
    //                         println!("Rejected player\n");
    //                         "REJECT"
    //                     }
    //                 };

    //                 println!("Writing join response: {join_response}");
    //                 let _ = stream.write(join_response.as_bytes()).unwrap();
    //             }
    //             RequestType::Act => {
    //                 // todo!()
    //             }
    //         }

    //         buffer.iter_mut().for_each(|v| *v = 0);
    //     }

    pub fn _wait(&self) {
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }

    // pub fn get_game_state(&self) -> &GameState {
    //     &self.game_state
    // }

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

//     thread::sleep(Duration::from_secs(1));
// });
// }

// fn handle_join(request: &str, server: ServerInstance) {
// println!("Handling join request: \"{request}\"");

// let parts: Vec<&str> = request.split(",").collect();

// if parts.len() != 2 {
//     panic!("Invalid request format for JOIN request");
// }

// /* The first part is ignored because it is checked elsewhere */
// let name = *parts.get(1).unwrap();

// println!("Checkpoint One");
// let is_unique = server.lock().unwrap().game_state.is_unique_name(name);
// println!("Checkpoint Two");

// if !is_unique {
//     println!("Name is not unique");
//     JoinResponse::Reject
// } else {
//     println!("Name is unique");
//     JoinResponse::Accept(name.to_string())
// }
// }
