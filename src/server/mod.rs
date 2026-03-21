use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    thread::{self},
    time::Duration,
};

use crate::model::{
    communication::{Request, Response, StreamHandler},
    game_state::GameState,
    player::Player,
};

// Top-Level Constants
const MAX_PLAYERS: usize = 1;
const IP_PORT: &str = "127.0.0.1:8080";

/// Type for representing a server instance.
pub struct Server {
    clients: HashMap<String, StreamHandler>,
    game_state: GameState,
}

/// A type alias to lessen code repetition.
type ServerType = Arc<Mutex<Server>>;

impl Server {
    /// Returns a new of instance of `Server` with all fields intialized to the default value.
    pub fn create() -> io::Result<ServerType> {
        Ok(Arc::new(Mutex::new(Server {
            clients: HashMap::new(),
            game_state: GameState::default(),
        })))
    }

    /// Starts an instance of server. Server is behind a mutex, which is why the method doesn't use
    /// `&mut self`.
    pub fn start(server: &ServerType) -> io::Result<()> {
        println!("Started server");
        println!("Waiting for players to join.");
        Server::accept_clients(server)?;
        Ok(())
    }

    /// Block the current thread until the correct number of clients join. Each client attempting to
    /// connect spawns a new thread.
    ///
    /// TODO: Possibly refactor to accept num players as argument.
    fn accept_clients(server: &ServerType) -> io::Result<()> {
        let listener = TcpListener::bind(IP_PORT)?;
        let mut handlers = Vec::new();
        for _ in 0..MAX_PLAYERS {
            let server_clone = Arc::clone(server);
            let (stream, _) = listener.accept()?;
            let stream_handler = StreamHandler::new(stream);
            let handler = thread::spawn(move || {
                handle_join(&server_clone, stream_handler).unwrap();
            });
            handlers.push(handler);
        }
        // Ensures that all of the client threads are finished executing before continuing the main
        // thread.
        for handler in handlers {
            handler.join().unwrap();
        }
        Ok(())
    }
}

/// Accept incoming TCP clients until it reaches the maximum. If a player has a unique name, the
/// stream is added to the `server.clients` hash map and the client is also added as a player to
/// `server.game_state`.
fn handle_join(server: &ServerType, mut stream_handler: StreamHandler) -> io::Result<()> {
    let join_request = stream_handler.await_request()?;
    if let Request::Join(name) = join_request {
        let mut server = server.lock().unwrap();
        let response = if server.game_state.is_unique_name(&name) {
            let new_player = Player::new(&name);
            server.game_state.add_player(new_player);
            println!("'{name}' joined the server");
            Response::Join(true)
        } else {
            println!("The name '{name}' is already being used");
            // If a player tries to join using a name that is already taken, then reject them.
            Response::Join(false)
        };
        stream_handler.send_response(&response)?;
        server
            .clients
            .entry(name.to_owned())
            .or_insert(stream_handler);
        Ok(())
    } else {
        unreachable!("Expected join request.")
    }
}
