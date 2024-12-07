use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use kingdom_kards::server::client::{choose_player_name, connect_to_server};
use kingdom_kards::server::host::start_server;
use kingdom_kards::server::utils::{choose_mode, get_input, Mode};
use kingdom_kards::utils::clear_screen;

fn main() {
    clear_screen();
    println!("Starting Kingdom Kards...\n");

    let mode = choose_mode();

    match mode {
        Mode::HostGame => {
            start_server();
        }
        Mode::ConnectGame => {
            if let Some(mut stream) = try_connect() {
                loop {
                    if let Ok(name) = choose_player_name() {
                        let _ = stream.write(name.as_bytes()).unwrap();
                        break;
                    }
                }
            }
        }
    }
}

fn try_connect() -> Option<TcpStream> {
    loop {
        match connect_to_server("127.0.0.1:5464") {
            Ok(stream) => {
                println!("Successfully connected to server");
                return Some(stream);
            }
            Err(e) => {
                println!("{e}");
                io::stdout().flush().expect("unable to flush stdout");
                let input = get_input("Try again [y/n]: ").to_lowercase();

                if input == "n" || input == "no" {
                    println!("Okay. Closing application");
                    return None;
                } else {
                    println!("Trying again...");
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
    }
}
