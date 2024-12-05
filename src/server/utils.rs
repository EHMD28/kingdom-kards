//! This module contains a set of standard utilities for use with this crate.
//! For the most part, this includes functions that accept and validate user
//! input.  

use std::io::{self, Write};

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
