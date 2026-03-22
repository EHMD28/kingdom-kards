use crate::{
    client::Client,
    server::Server,
    ui::cli::{display_client_wait, display_server_wait},
};
use clap::Parser;
use std::io;

mod client;
mod model;
mod server;
mod ui;
mod utils;

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
            Client::start(&mut client)?;
            display_client_wait();
        }
        BinaryType::Server => {
            let server = Server::create()?;
            Server::start(&server)?;
            display_server_wait();
        }
    };
    Ok(())
}
