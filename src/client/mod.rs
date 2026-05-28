use std::{
    io::{self},
    net::TcpStream,
    thread,
    time::Duration,
};

use crate::{
    model::{
        communication::{Request, Response, StreamHandler},
        game_state::GameState,
    },
    utils::debugging::Debugging,
};

pub struct Client {
    stream_handler: StreamHandler,
    game_state: Option<GameState>,
}

impl Client {
    // Creates a new client instance connected to the TCP port 127.0.0.1:8080.
    pub fn create() -> io::Result<Client> {
        let stream = TcpStream::connect("127.0.0.1:8080")?;
        Ok(Client {
            stream_handler: StreamHandler::new(stream),
            game_state: None,
        })
    }

    /// Starts an instance of a client.
    pub fn start(client: &mut Client) -> io::Result<()> {
        client.handle_join()?;
        client.get_game_state_from_server()?;
        Ok(())
    }

    /// Prompts the player to input their name. Then, it sends a join request to the server,
    /// printing the response from the server.
    fn handle_join(&mut self) -> io::Result<()> {
        // let name = get_text_input("Enter your name: ")?;
        let name = String::from("Alice");
        let join_request = Request::Join(name);
        self.stream_handler.send_request(&join_request)?;
        let join_response = self.stream_handler.await_response()?;
        match join_response {
            // Client was accepted.
            Response::Join(true) => Debugging::print_info("Joined the server."),
            // Client was rejected because someone is already using that name.
            Response::Join(false) => {
                Debugging::print_info("Rejected by the server. Name is already in use")
            }
            _ => unreachable!("Expected join response. Received {join_response}"),
        }
        Ok(())
    }

    fn get_game_state_from_server(&mut self) -> io::Result<()> {
        self.stream_handler.send_request(&Request::GameState)?;
        let response = self.stream_handler.await_response()?;
        match response {
            Response::GameState(game_state) => self.game_state = Some(game_state),
            _ => unreachable!("Expected game state, received: {response}"),
        }
        Ok(())
    }

    pub fn _wait(&self) {
        loop {
            Debugging::print_info("Client is waiting");
            thread::sleep(Duration::from_secs(1));
        }
    }
}
