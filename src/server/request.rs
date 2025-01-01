use std::{
    fmt::Display,
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    str::FromStr,
    thread,
    time::Duration,
};

use crate::server::utils::is_zeroed;

use super::ServerError;

#[derive(PartialEq, Debug)]
pub enum RequestType {
    /// Format: `"REQ,NAME"`.
    Name,
    /// Format `"REQ,ACT"`.
    PlayerAction,
}

impl Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            RequestType::Name => "NAME",
            RequestType::PlayerAction => "ACT",
        };

        write!(f, "{type_str}")
    }
}

/// This is the struct used for representing requests that are sent
/// to and from the server. When being transmitted, requests are
/// serialized into strings before being deserialized back into
/// a `Request` struct.
#[derive(PartialEq, Debug)]
pub struct Request {
    request_type: RequestType,
}

impl Request {
    pub fn new(request_type: RequestType) -> Request {
        Request { request_type }
    }

    pub fn request_type(&self) -> &RequestType {
        &self.request_type
    }
}

impl Display for Request {
    /// Request strings are formatted as a series of comma-separated values. The first
    /// value must be "REQ", to verify that strings is actually a request string. The next
    /// value is the type of the request. The possible types are "NAME" and "ACT". For "NAME",
    /// the format is `"REQ,NAME,{NAME}"` and for "ACT", the format is
    /// `"REQ,ACT,{SYMBOL},{ATTACHMENT},{FROM_PLAYER},{TO_PLAYER}"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REQ,{}", self.request_type)
    }
}

#[derive(Debug)]
pub enum RequestParseError {
    /// Request string does not start with "REQ".
    NotARequest,
    /// There are two few arguments in request string.
    InvalidNumArguments,
    /// The type found in the request string is invalid.
    InvalidType,
}

impl FromStr for Request {
    type Err = RequestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let first = parts.next().unwrap();

        if first != "REQ" {
            return Err(RequestParseError::NotARequest);
        }

        /* Count should be 1 instead of 2 because first is consumed. */
        if parts.clone().count() != 1 {
            return Err(RequestParseError::InvalidNumArguments);
        }

        let request_type = parts.next().unwrap();

        match request_type {
            "NAME" => Ok(Request {
                request_type: RequestType::Name,
            }),
            "ACT" => Ok(Request {
                request_type: RequestType::PlayerAction,
            }),
            _ => Err(RequestParseError::InvalidType),
        }
    }
}

pub fn send_request(stream: &mut TcpStream, request: Request) -> std::io::Result<()> {
    let request = request.to_string();
    let request = request.as_bytes();
    stream.write_all(request)?;
    println!("Sent request");
    Ok(())
}

pub fn await_request(
    stream: &mut TcpStream,
    request_type: RequestType,
) -> Result<Request, ServerError> {
    let mut buffer = [0u8; 512];

    while is_zeroed(&buffer) {
        println!("Awaiting request");

        if let Err(e) = stream.read(&mut buffer) {
            if e.kind() != ErrorKind::Interrupted {
                return Err(ServerError::IoError(e));
            }
        }

        thread::sleep(Duration::from_millis(500));
    }

    let received = String::from_utf8_lossy(&buffer);
    let received = received.trim_matches('\0');
    let request = Request::from_str(received);

    match request {
        Ok(request) => {
            if *request.request_type() != request_type {
                Err(ServerError::ExpectedRequestType(request_type))
            } else {
                Ok(request)
            }
        }
        Err(err) => match err {
            RequestParseError::NotARequest
            | RequestParseError::InvalidNumArguments
            | RequestParseError::InvalidType => Err(ServerError::RequestError(err)),
        },
    }
}
