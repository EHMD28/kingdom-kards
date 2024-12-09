//! This module contains a set of functions for creating a server and
//! handling clients.

use core::panic;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::Arc,
    thread,
    time::Duration,
};

use crate::game::game_state::GameState;

use super::utils::is_zeroed;

pub struct ServerInstance {
    game_state: GameState,
    listener: TcpListener,
}

enum JoinResponse {
    Accept,
    Reject,
}

enum RequestType {
    Join,
    Act,
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
    pub fn create() -> Arc<ServerInstance> {
        let _join_code = 1234;
        let port = "127.0.0.1:5464".to_string();
        let listener = TcpListener::bind(port).expect("Failed to bind to port 127.0.0.1:5464");
        println!("Starting server with join code: {_join_code}");

        Arc::new(ServerInstance {
            game_state: GameState::new(),
            listener,
        })
    }

    pub fn start(self: Arc<ServerInstance>) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream, self.clone());
        }
    }

    // pub fn get_listener(&self) -> &TcpListener {
    //     &self.listener
    // }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn is_unique_name(&self, name: &str) -> bool {
        let all_players = self.get_game_state().get_all_players();
        for player in all_players.iter() {
            if player.get_name() == name {
                return false;
            }
        }

        true
    }
}

fn handle_connection(mut stream: TcpStream, server: Arc<ServerInstance>) {
    thread::spawn(move || loop {
        let mut buffer = [0u8; 512];
        let _ = stream.read(&mut buffer).unwrap();

        if !is_zeroed(&buffer) {
            let received = String::from_utf8_lossy(&buffer).to_string();
            println!("Recieved: \"{received}\"");
            let request_type = RequestType::from_str(&received).unwrap();
            match request_type {
                RequestType::Join => {
                    let join_response = match handle_join(&received, Arc::clone(&server)) {
                        JoinResponse::Accept => "ACCEPT",
                        JoinResponse::Reject => "REJECT",
                    };
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

fn handle_join(request: &str, server: Arc<ServerInstance>) -> JoinResponse {
    // The first part is ignored because it is checked elsewhere
    let parts: Vec<&str> = request.split(",").collect();

    if parts.len() != 2 {
        panic!("Invalid request format for JOIN");
    }

    let name = parts.get(1).unwrap();

    if !server.is_unique_name(name) {
        JoinResponse::Reject
    } else {
        JoinResponse::Accept
    }
}

// fn get_request_type(request: String) -> RequestType {
//     let parts: Vec<&str> = request.split(",").collect();
//     let first = parts.first().unwrap();
//     let first = *first;

//     match first {
//         "JOIN" => RequestType::Join,
//         "ACT" => RequestType::Act,
//         _ => panic!("Invalid conversion from string to RequestType"),
//     }
// }

// fn handle_received(received: String, server: &ServerInstance) -> JoinResponse {
//     let parts: Vec<&str> = received.split(",").collect();
//     let first = parts.first().unwrap();

//     if *first == "JOIN" {
//         if parts.len() != 2 {
//             panic!("Invalid JOIN request format");
//         }

//         let name = parts.get(1).unwrap();

//         if !server.is_unique_name(name) {

//         }
//     } else if *first == "ACT" {
//         if parts.len() != 5 {
//             panic!("Invalid ACT request");
//         }

//         todo!()
//     }
// }

// // TODO: Make not bad
// fn handle_recieved(recieved: &str, server: &ServerInstance) -> JoinResponse {
//     let parts: Vec<&str> = recieved.split(",").collect();
//     let first = parts.first().unwrap();

//     if *first == "JOIN" {
//         if parts.len() != 2 {
//             panic!("Invalid JOIN message");
//         }

//         let name = parts.get(1).unwrap();
//         if server.is_unique_name(name) {
//             JoinResponse::Accept
//         } else {
//             JoinResponse::Reject
//         }
//     } else if *first == "ACT" {
//         todo!()
//     } else {
//         panic!("Invalid server request");
//     }
// }

// fn read_to_buffer(mut stream: &TcpStream, buffer: &mut [u8]) -> String {
//     let _ = stream.read(buffer).unwrap();
//     let bufstr = String::from_utf8_lossy(buffer);
//     bufstr.to_string()
// }
