use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

use crate::{
    model::communication::{Request, Response, StreamHandler},
    ui::cli::get_text_input,
};

pub struct Client {
    stream_handler: StreamHandler,
}

impl Client {
    // Creates a new client instance connected to the TCP port 127.0.0.1:8080.
    pub fn create() -> io::Result<Client> {
        let stream = TcpStream::connect("127.0.0.1:8080")?;
        Ok(Client {
            stream_handler: StreamHandler::new(stream),
        })
    }

    /// Starts an instance of a client.
    pub fn start(&mut self) -> io::Result<()> {
        self.handle_join()?;
        Ok(())
    }

    /// Prompts the player to input their name. Then, it sends a join request to the server,
    /// printing the response from the server.
    fn handle_join(&mut self) -> io::Result<()> {
        let name = get_text_input("Enter your name: ")?;
        let join_request = Request::Join(name);
        self.stream_handler.send_request(&join_request)?;
        let join_response = self.stream_handler.await_response()?;
        match join_response {
            // Client was accepted.
            Response::Join(true) => println!("Joined the server."),
            // Client was rejected because someone is already using that name.
            Response::Join(false) => println!("Rejected by the server. Name is already in use"),
            _ => unreachable!("Expected join response"),
        }
        Ok(())
    }

    pub fn _wait(&self) {
        loop {
            println!("Client is waiting");
            thread::sleep(Duration::from_secs(1));
        }
    }
}
