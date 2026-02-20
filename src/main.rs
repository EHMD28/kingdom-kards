use crate::{client::Client, server::Server};
use clap::Parser;
use std::{io, thread, time::Duration};

mod client;
mod model;
mod server;
mod ui;

#[derive(clap::ValueEnum, Clone)]
enum BinaryType {
    Client,
    Server,
}

#[derive(clap::Parser)]
struct CliArgs {
    bin_type: BinaryType,
}

fn main() -> io::Result<()> {
    let args = CliArgs::parse();
    match args.bin_type {
        BinaryType::Client => {
            let mut client = Client::create()?;
            client.start()?;
        }
        BinaryType::Server => {
            let server = Server::create()?;
            Server::start(&server)?;
            loop {
                println!("Server is waiting");
                thread::sleep(Duration::from_secs(2));
            }
        }
    };
    Ok(())
}
