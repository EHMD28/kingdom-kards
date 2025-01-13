//! This module contains a set of standard utilities for use with this crate.
//! For the most part, this includes functions that accept and validate user
//! input.  

use std::io::{self, Write};

/// Whether the user wants to be a server or client.
pub enum Mode {
    HostGame,
    ConnectGame,
}

/// Prompt player to choose whether to start server or connect to server.
pub fn choose_mode() -> Mode {
    println!("1. Host a game");
    println!("2. Join a game\n");

    let input = get_num_input("Choose an option [1 or 2]: ", 1, 2);

    match input {
        1 => Mode::HostGame,
        2 => Mode::ConnectGame,
        /* get_num_input() ensures that num is between 1 and 2 */
        _ => panic!("Invalid mode chosen."),
    }
}

/// Prompts the user with `prompt` and returns input (with whitespace trimmed).
pub fn get_input(prompt: &str, max_len: usize) -> String {
    let input = &mut String::new();

    loop {
        print!("{prompt}");
        io::stdout().flush().expect("couldn't flush stdout");

        input.clear();
        io::stdin().read_line(input).expect("unable to read input");
        let input = input.trim().to_string();

        if input.len() < max_len {
            return input;
        }
    }
}

/// Prompts the user to enter a number. This function will keep prompting
/// the user until they enter a number that fits within the range
/// restrictions.
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
                return n;
            }
        }
    }
}

/// Removes both `\n` and `\r\n` from the end of a string.
pub fn remove_newline(s: &mut String) {
    if let Some('\n') = s.chars().last() {
        s.pop();
        if let Some('\r') = s.chars().last() {
            s.pop();
        }
    }
}
