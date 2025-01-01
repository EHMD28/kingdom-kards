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

/// Prompts the user with `prompt` and returns input (with whitespace trimmed).
pub fn get_input(prompt: &str) -> String {
    let stdin = io::stdin();
    let input = &mut String::new();

    print!("{prompt}");
    io::stdout().flush().expect("couldn't flush stdout");

    input.clear();
    stdin.read_line(input).expect("unable to read input");

    input.trim().to_string()
}

pub fn get_num_input(prompt: &str, min: i32, max: i32) -> i32 {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let input = &mut String::new();

    loop {
        print!("{prompt}");
        stdout.flush().expect("Unable to flush stdout");
        input.clear();
        stdin.read_line(input).expect("Unable to read input");
        let input = input.trim().to_owned();

        let num_input = input.parse::<i32>();
        if let Ok(n) = num_input {
            if (n < min) || (n > max) {
                continue;
            } else {
                return num_input.unwrap();
            }
        }
    }
}

pub fn is_zeroed(buf: &[u8]) -> bool {
    *buf == [0u8; 512]
}
