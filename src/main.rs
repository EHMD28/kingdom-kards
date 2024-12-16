use kingdom_kards::server::client::ClientInstance;
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
            let mut client = ClientInstance::new();

            if client.connect_to_server("127.0.0.1:5464").is_none() {
                return; /* close application */
            }

            client.choose_player_name();
        }
    }
}
