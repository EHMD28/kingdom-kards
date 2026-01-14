use std::io;

use crate::server::Server;

mod client;
mod model;
mod server;
mod ui;

fn main() -> io::Result<()> {
    let _ = Server::create()?;
    Ok(())
}
