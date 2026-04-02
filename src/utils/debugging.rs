const LOGGING_ENABLED: bool = true;
// ANSI Terminal Color Codes
const FG_RED: &str = "\x1b[0;31m";
const FG_GREEN: &str = "\x1b[0;32m";
const FG_BLUE: &str = "\x1b[0;34m";
const COLOR_RESET: &str = "\x1b[0m";

pub struct Debugging;

impl Debugging {
    pub fn print_info(msg: &str) {
        if LOGGING_ENABLED {
            println!("{FG_BLUE}{msg}{COLOR_RESET}");
        }
    }

    pub fn print_error(msg: &str) {
        if LOGGING_ENABLED {
            eprintln!("{FG_RED}{msg}{COLOR_RESET}");
        }
    }

    pub fn print_checkpoint(num: u8) {
        if LOGGING_ENABLED {
            println!("{FG_GREEN}Checkpoint {num}{COLOR_RESET}");
        }
    }
}
