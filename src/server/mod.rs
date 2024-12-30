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
pub mod request;
pub mod response;
pub mod utils;

use std::{fmt, io};

use request::{RequestParseError, RequestType};
use response::{ResponseParseError, ResponseType};

pub enum ServerError {
    NoError,
    FailedToConnect(String),
    ExpectedRequestType(RequestType),
    ExpectedResponseType(ResponseType),
    RequestError(RequestParseError),
    ReponseError(ResponseParseError),
    IoError(io::Error),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::FailedToConnect(msg) => write!(f, "Failed to connect to {msg}"),
            ServerError::NoError => write!(f, "No error"),
            ServerError::RequestError(err) => write!(f, "Request error: {err:?}"),
            ServerError::ReponseError(err) => write!(f, "Response error: {err:?}"),
            ServerError::IoError(err) => write!(f, "IO Error: {err:?}"),
            ServerError::ExpectedRequestType(request_type) => {
                write!(f, "Expected request of type {request_type}")
            }
            ServerError::ExpectedResponseType(response_type) => {
                write!(f, "Expected response of type {response_type}")
            }
        }
    }
}
