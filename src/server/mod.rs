//! This module is responsible for all client-server communication. The basis
//! of all server communication is the `Action` struct. This struct can be
//! serialized into a string which is then sent to and from the server are strings
//! following a specific format: `ACT,{SYMBOL},{ATTACHMENT},{FROM_PLAYER},{TO_PLAYER}`.
//! The values are all separated by commas. As such, none of the values are allowed to
//! contain commas. These strings can then be deserialized into Action structs which
//! interpereted client side.

// pub mod action;
pub mod client;
pub mod host;
pub mod request_response;
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
