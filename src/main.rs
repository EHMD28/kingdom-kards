use kingdom_kards::server::client::connect_to_server;
use kingdom_kards::server::host::start_server;
use kingdom_kards::server::utils::{choose_mode, Mode};

fn main() {
    let mode = choose_mode();

    match mode {
        Mode::HostGame => {
            start_server();
        }
        Mode::ConnectGame => {
            connect_to_server("127.0.0.1:5464");
        }
    }
}
