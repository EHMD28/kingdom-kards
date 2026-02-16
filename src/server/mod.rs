use std::{
    io::{self, Read},
    net::TcpListener,
    thread,
};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    /// Returns a new of instance of `Server` on port `127.0.0.1:8080`.
    pub fn create() -> io::Result<Server> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        println!("Started server");
        for stream in listener.incoming() {
            thread::spawn(|| {
                let mut stream = stream.unwrap();
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer).unwrap();
                println!("{buffer}");
            });
        }
        Ok(Server { listener })
    }
}
