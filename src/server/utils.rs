//! This module contains a set of standard utilities for use with this crate.
//! For the most part, this includes functions that accept and validate user
//! input.  

use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

pub enum Mode {
    HostGame,
    ConnectGame,
}

/// Prompt player to choose whether to start server or connect to server.
pub fn choose_mode() -> Mode {
    println!("1. Host a game");
    println!("2. Connect to game\n");

    loop {
        let input = get_input("Choose an option [1 or 2]: ");

        let input = input.trim().get(0..1).unwrap();

        if input == "1" {
            return Mode::HostGame;
        } else if input == "2" {
            return Mode::ConnectGame;
        } else {
            println!("Invalid input, try again");
        }
    }
}

pub fn get_input(prompt: &str) -> String {
    let stdin = io::stdin();
    let input = &mut String::new();

    print!("{prompt}");
    io::stdout().flush().expect("couldn't flush stdout");

    input.clear();
    stdin.read_line(input).expect("unable to read input");

    input.trim().to_string()
}

pub fn is_zeroed(buf: &[u8]) -> bool {
    *buf == [0u8; 512]
}

pub fn get_response(stream: &mut TcpStream) -> String {
    let mut buffer = [0u8; 512];

    let _ = stream.read(&mut buffer).unwrap();

    while is_zeroed(&buffer) {
        let _ = stream.read(&mut buffer).unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    let response = String::from_utf8_lossy(&buffer);
    let response = response.trim_end_matches('\0');
    response.trim().to_string()
}
