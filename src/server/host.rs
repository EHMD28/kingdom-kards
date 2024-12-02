use std::net::{TcpListener, TcpStream};

const KINGDOM_KARDS_PORT: &'static str = "5464";

/// Starts instance of Kingdom Kards server.
/// Hosted locally on port 5464 because 'king' - phone keypad -> '5464'
pub fn start_server() {
    let join_code = 1234;
    let port = format!("127.0.0.1:{KINGDOM_KARDS_PORT}");
    let listener = TcpListener::bind(port).expect("Failed to bind to port 8080");
    println!("Starting server | Join Code {join_code}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    todo!()
}
