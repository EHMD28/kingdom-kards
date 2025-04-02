//! This module is responsible for all client-server communication. The basis
//! of all server communication is the `Action` struct. This struct can be
//! serialized into a string which is then sent to and from the server are strings
//! following a specific format: `ACT,{SYMBOL},{ATTACHMENT},{FROM_PLAYER},{TO_PLAYER}`.
//! The values are all separated by commas. As such, none of the values are allowed to
//! contain commas. These strings can then be deserialized into Action structs which
//! interpereted client side.

pub mod client;
pub mod constants;
pub mod host;
pub mod request;
pub mod response;
pub mod utils;

use std::{
    fmt,
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    str::FromStr,
};

use request::{Request, RequestParseError, RequestType};
use response::{Response, ResponseParseError, ResponseType};
use utils::remove_newline;

use crate::utils::variant_eq;

/// This is the type used for representing server-side errors.
pub enum ServerError {
    /// Failed to connect to server at port.
    FailedToConnect(String),
    /// Expected a request of a different type.
    ExpectedRequestType(RequestType),
    /// Expected a response of a different type.
    ExpectedResponseType(ResponseType),
    /// Encounted a request parsing error.
    RequestError(RequestParseError),
    /// Encountered a response parsing error.
    ReponseError(ResponseParseError),
    /// Encountered a standard io::Error.
    IoError(io::Error),
}

impl fmt::Display for ServerError {
    /// When converted to strings, ServerErrors contain a descriptive message
    /// of what the error was as well as any specific details about the error.
    /// It does not include where the error occured.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::FailedToConnect(msg) => write!(f, "Failed to connect to {msg}"),
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

/// The `StreamHandler` struct is responsible for handling all sending and
/// receiving of requests and responses. When sending requests/responses, use
/// the `send_request` and `send_response` methods respectively. When receiving
///  requests/responses, use the `await_request` and `await_response` methods.
/// A `send_request` should always be followed by an `await_response` (same
/// with) `send_response` and `await_request`.
pub struct StreamHandler {
    reader: BufReader<TcpStream>,
}

impl StreamHandler {
    /// Creates a new instance of `StreamHandler` attached to stream.
    /// Multiple `StreamHandler`s should not be attached to the same
    /// stream at the same time.
    pub fn new(stream: TcpStream) -> StreamHandler {
        StreamHandler {
            reader: BufReader::new(stream),
        }
    }

    /// Sends `response` as string over `stream`.
    pub fn send_response(&mut self, response: &Response) -> std::io::Result<()> {
        let response_type = response.response_type();
        let mut response = response.to_string();
        /* Newline is used a delimiting character to avoid requests being mangled. */
        response.push('\n');
        let response = response.as_bytes();
        // dbg!(response);
        let stream = self.reader.get_mut();
        stream.write_all(response)?;
        stream.flush()?;
        println!("Sent response of type {response_type}");
        Ok(())
    }

    /// Blocks the current thread until a `Request` is received. If the request
    /// received is of the wrong type, then this function will return an error.
    pub fn await_request(&mut self, request: &Request) -> Result<Request, ServerError> {
        let request_type = request.request_type().to_owned();
        let received = &mut String::new();

        println!("Awaiting request of type {request_type}");
        // received.clear();
        if let Err(err) = self.reader.read_line(received) {
            return Err(ServerError::IoError(err));
        }
        remove_newline(received);

        println!("Received: {received}");
        let request = Request::from_str(received);

        match request {
            Ok(request) => {
                if variant_eq(request.request_type(), &request_type) {
                    Ok(request)
                } else {
                    Err(ServerError::ExpectedRequestType(request_type))
                }
            }
            Err(err) => Err(ServerError::RequestError(err)),
        }
    }

    /// Sends `request` over stream as a string.
    pub fn send_request(&mut self, request: &Request) -> std::io::Result<()> {
        let request_type = request.request_type();
        let mut request = request.to_string();
        /* Newline is used a delimiting character to avoid requests being mangled. */
        request.push('\n');
        let request = request.as_bytes();
        let stream = self.reader.get_mut();
        stream.write_all(request)?;
        stream.flush()?;
        println!("Sent request of type {}", request_type);
        Ok(())
    }

    /// Blocks the current thread until a `Response` is received. If the response
    /// received is of the wrong type, then this function will return an error.
    pub fn await_response(&mut self, response: &Response) -> Result<Response, ServerError> {
        let response_type = response.response_type().to_owned();
        let received = &mut String::new();

        println!("Awaiting response of type {response_type}");
        /* Using read_line() because requests/responses are separated by newline delimeter */
        // received.clear();
        if let Err(e) = self.reader.read_line(received) {
            return Err(ServerError::IoError(e));
        }

        remove_newline(received);

        println!("Received: {received}");
        let response = Response::from_str(received);

        match response {
            Ok(request) => {
                if variant_eq(request.response_type(), &response_type) {
                    Ok(request)
                } else {
                    Err(ServerError::ExpectedResponseType(response_type))
                }
            }
            Err(err) => Err(ServerError::ReponseError(err)),
        }
    }
}
