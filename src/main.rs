use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use kingdom_kards::server::client::connect_to_server;
use kingdom_kards::server::host::start_server;
use kingdom_kards::server::utils::{choose_mode, get_input, Mode};
use kingdom_kards::server::ServerError;
use kingdom_kards::utils::clear_screen;

fn main() {
    clear_screen();
    println!("Starting Kingdom Kards...\n");

    let mode = choose_mode();

    match mode {
        Mode::HostGame => {
            start_server();
        }
        Mode::ConnectGame => loop {
            let error = connect_to_server("127.0.0.1:5464");
            if let ServerError::FailedToConnect(_) = error {
                println!("{}", error);
                io::stdout().flush().expect("unable to flush stdout");
                let input = get_input("Try again [y/n]: ").to_lowercase();

                if input == "n" || input == "no" {
                    break;
                } else {
                    thread::sleep(Duration::from_millis(500));
                }
            }
        },
    }
}
