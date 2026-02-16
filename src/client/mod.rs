use std::{
    io::{self, Write},
    net::TcpStream,
    time::SystemTime,
};

use crate::ui::cli::get_text_input;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn create() -> io::Result<Client> {
        let stream = TcpStream::connect("127.0.0.1:8080")?;
        Ok(Client { stream })
    }

    pub fn start(&mut self) {
        if let Ok(msg) = get_text_input("Enter your name: ") {
            let _ = self.stream.write_all(msg.as_bytes());
        }
    }
}
