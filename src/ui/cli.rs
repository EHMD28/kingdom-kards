use std::io::{self, Write};

use crate::utils::trim_newline;

/// Returns the text the user entered with the newline removed.
pub fn get_text_input(prompt: &str) -> io::Result<String> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    print!("{prompt}");
    // This is necessary because the prompt doesn't have a newline.
    stdout.flush()?;
    stdin.read_line(&mut buffer)?;
    trim_newline(&mut buffer);
    Ok(buffer)
}
