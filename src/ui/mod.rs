//! This module contains all the code responsible for the program's user interface.

use std::io::{self, Write};

/// Prompts the user with `prompt` and returns input (with whitespace trimmed).
pub fn get_input(prompt: &str, max_len: usize) -> String {
    let input = &mut String::new();
    let stdout = &mut io::stdout();
    loop {
        print!("{prompt}");
        stdout.flush().expect("couldn't flush stdout");
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

pub fn get_bool_input(prompt: &str, true_opt: &str, false_opt: &str) -> bool {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let input = &mut String::new();
    loop {
        print!("{prompt}");
        stdout.flush().expect("Unable to flush stdout");
        input.clear();
        stdin.read_line(input).expect("Unable to read input");
        let input = input.trim().to_owned();
        if input == true_opt {
            return true;
        } else if input == false_opt {
            return false;
        }
    }
}

/// Prompts the user to use arrow keys to select an option, returning the index (starting from 0) of
/// the selected option.
pub fn select_option(options: &[&str]) -> usize {
    todo!()
}
