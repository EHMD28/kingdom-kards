use std::{
    fmt,
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    model::{action::Action, game_state::GameState},
    utils::debugging::Debugging,
};

#[derive(Serialize, Deserialize)]
pub enum Request {
    Join(String),
    GameState,
    Action,
}

impl FromStr for Request {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let request: Request = serde_json::from_str(s)?;
        Ok(request)
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_string(self).unwrap();
        write!(f, "{s}")
    }
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Join(bool),
    GameState(GameState),
    Action(Action),
}

impl FromStr for Response {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let request: Response = serde_json::from_str(s)?;
        Ok(request)
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_string(self).unwrap();
        write!(f, "{s}")
    }
}

/// Wrapper around TcpStream to prevent data from being lost between reads/writes.
pub struct StreamHandler {
    reader: BufReader<TcpStream>,
}

impl StreamHandler {
    /// Creates a new `StreamHandler` from a `TcpStream`.
    pub fn new(stream: TcpStream) -> StreamHandler {
        StreamHandler {
            reader: BufReader::new(stream),
        }
    }

    /// Sends a `Request` over the current stream.
    pub fn send_request(&mut self, request: &Request) -> io::Result<()> {
        let request = request.to_string();
        self.send(&request)?;
        let dbg_msg = format!("Sent request: {request}");
        Debugging::print_error(&dbg_msg);
        Ok(())
    }

    /// Sends a response over the current stream.
    pub fn send_response(&mut self, response: &Response) -> io::Result<()> {
        let response = response.to_string();
        self.send(&response)?;
        let dbg_msg = format!("Sent response: {response}");
        Debugging::print_info(&dbg_msg);
        Ok(())
    }

    /// Sends a response over the stream with a new line at the end. This new line should be
    /// consumed.
    fn send(&mut self, buffer: &str) -> io::Result<()> {
        let stream = self.reader.get_mut();
        writeln!(stream, "{buffer}")?;
        // Send any buffered contents. Don't know if this is necessary because a newline is being
        // written to the string
        stream.flush()?;
        Ok(())
    }

    /// Blocks the current thread until a message is received. Attempts to parse message as request,
    /// returning the request if successful.
    pub fn await_request(&mut self) -> io::Result<Request> {
        Debugging::print_info("Awaiting request");
        let buffer = self.await_str()?;
        let request = Request::from_str(&buffer).unwrap();
        let dbg_msg = format!("Receieved: {request}");
        Debugging::print_info(&dbg_msg);
        Ok(request)
    }

    /// Blocks the current thread until a message is received. Attempts to parse message as
    /// response, returning the response if successful.
    pub fn await_response(&mut self) -> io::Result<Response> {
        Debugging::print_info("Awaiting response");
        let buffer = self.await_str()?;
        let response = Response::from_str(&buffer).unwrap();
        let dbg_msg = format!("Receieved: {response}");
        Debugging::print_info(&dbg_msg);
        Ok(response)
    }

    /// Blocks the current thread until a message is received.
    fn await_str(&mut self) -> io::Result<String> {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        Ok(buffer)
    }
}
