use std::io;

use clap::Parser;

use crate::{client::Client, server::Server};

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
            client.start();
        }
        BinaryType::Server => {
            let mut server = Server::create()?;
            server.start();
        }
    };
    Ok(())
}
