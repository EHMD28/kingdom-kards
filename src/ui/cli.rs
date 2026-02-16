use std::io::{self, Write};

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

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
