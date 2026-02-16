use std::{
    io::{self, Read},
    net::TcpListener,
    sync::mpsc::{Receiver, Sender, channel},
    thread,
};

const MAX_PLAYERS: usize = 2;

pub struct Server {
    listener: TcpListener,
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl Server {
    /// Returns a new of instance of `Server` on port `127.0.0.1:8080`.
    pub fn create() -> io::Result<Server> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        let (sender, receiver) = channel::<String>();
        Ok(Server {
            listener,
            sender,
            receiver,
        })
    }

    pub fn start(&mut self) {
        self.accept_clients();
        self.handle_clients();
    }

    fn accept_clients(&mut self) {
        println!("Started server");
        for _ in 0..MAX_PLAYERS {
            if let Ok((mut stream, _)) = self.listener.accept() {
                let new_sender = self.sender.clone();
                thread::spawn(move || {
                    let mut buffer = String::new();
                    stream.read_to_string(&mut buffer).unwrap();
                    let _ = new_sender.send(buffer);
                });
            }
        }
    }

    fn handle_clients(&mut self) {
        while let Ok(message) = self.receiver.recv() {
            println!("Receieved: '{message}'");
        }
    }
}
