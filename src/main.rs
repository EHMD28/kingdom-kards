use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use kingdom_kards::server::client::{
    choose_player_name, connect_to_server, send_join_request, try_connect, ClientError,
};
use kingdom_kards::server::host::ServerInstance;
use kingdom_kards::server::utils::{choose_mode, get_input, get_response, Mode};
use kingdom_kards::utils::clear_screen;

fn main() {
    clear_screen();
    println!("Starting Kingdom Kards...\n");

    let mode = choose_mode();

    match mode {
        Mode::HostGame => {
            let server = ServerInstance::create();
            server.start();
        }
        Mode::ConnectGame => {
            if let Some(mut stream) = try_connect() {
                loop {
                    match choose_player_name() {
                        Ok(name) => {
                            send_join_request(&mut stream, &name);
                        }
                        Err(err) => match err {
                            ClientError::InvalidCharacterFound => {
                                println!("Name cannot contain invalid character (e.g. ç or ♥)")
                            }
                            ClientError::CommaFound => println!("Name cannot contain a comma"),
                        },
                    }
                }
            } else {
                todo!()
            }

            // if let Some(mut stream) = try_connect() {
            //     loop {
            //         match choose_player_name() {
            //             Ok(name) => {
            //                 let message = format!("JOIN,{name}");
            //                 let _ = stream.write(message.as_bytes()).unwrap();
            //                 thread::sleep(Duration::from_millis(500));

            //                 // LEFT OFF HERE
            //                 let response = get_response(&mut stream);

            //                 // name was rejected
            //                 if response == "REJECT" {
            //                     continue;
            //                 } else {
            //                     println!("Sucessfully joined server");
            //                 }

            //                 break;
            //             }
            //             Err(err) => match err {
            //                 ClientError::InvalidCharacterFound => {
            //                     println!(
            //                         "Name cannot contain special characters (e.g. ç or emoji)"
            //                     );
            //                 }
            //                 ClientError::CommaFound => println!("Name cannot contain comma"),
            //             },
            //         }
            //     }
        }
    }
}
