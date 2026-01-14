use std::{
    io::{self, Write},
    net::TcpStream,
};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn create() -> io::Result<Client> {
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        stream.write_all("Hello, World".as_bytes())?;
        Ok(Client { stream })
    }
}
