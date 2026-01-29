use std::io;

use clap::Parser;

use crate::{
    client::Client,
    model::{Action, ActionType, Card, Suit, Value},
    server::Server,
};

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
            let _ = Client::create()?;
        }
        BinaryType::Server => {
            let _ = Server::create()?;
        }
    };
    Ok(())
}
