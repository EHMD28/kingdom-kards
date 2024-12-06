pub mod action;
pub mod client;
pub mod host;
pub mod utils;

use std::fmt;

pub enum ServerError {
    NoError,
    FailedToConnect(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::FailedToConnect(msg) => write!(f, "Failed to connect to {msg}"),
            ServerError::NoError => write!(f, "No error"),
        }
    }
}
