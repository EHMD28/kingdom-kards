use kingdom_kards::server::client::{
    choose_player_name, send_join_request, try_connect, ClientError,
};
use kingdom_kards::server::host::ServerInstance;
use kingdom_kards::server::utils::{choose_mode, Mode};
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
        }
    }
}
