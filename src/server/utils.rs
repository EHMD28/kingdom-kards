//! This module contains a set of standard utilities for use with this crate.
//! For the most part, this includes functions that accept and validate user
//! input.  


use crate::ui::get_num_input;

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
        _ => unreachable!("Invalid mode chosen."),
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
