//! This module contains a set of functions for creating a server and
//! handling clients.

use core::panic;
use std::{
    fmt,
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
            join_code: "1234".to_string(),
        }
    }

    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn start(&mut self) {
        let num_players: u8 = loop {
            let result = get_input("Enter number of players (max. 6): ").parse::<u8>();

            if let Ok(num_players) = result {
                if num_players > 6 {
                    println!("Too many players. Maximum is 6.");
                } else if num_players < 2 {
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

    fn handle_connections(&self) {
        for (client, _) in self.clients.iter() {}
    }

    pub fn _wait(&self) {
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
