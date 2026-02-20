use std::{
    fmt,
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::model::{action::Action, game_state::GameState};

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

/// Wrapper around TcpStream to prevent data from being lost between reads.
pub struct StreamHandler {
    reader: BufReader<TcpStream>,
}

impl StreamHandler {
    pub fn new(stream: TcpStream) -> StreamHandler {
        StreamHandler {
            reader: BufReader::new(stream),
        }
    }

    pub fn send_request(&mut self, request: &Request) -> io::Result<()> {
        let request = request.to_string();
        self.send(&request)?;
        Ok(())
    }

    pub fn send_response(&mut self, response: &Response) -> io::Result<()> {
        let response = response.to_string();
        self.send(&response)?;
        Ok(())
    }

    fn send(&mut self, buffer: &str) -> io::Result<()> {
        let stream = self.reader.get_mut();
        writeln!(stream, "{buffer}")?;
        // Send any buffered contents. Don't know if this is necessary because a newline is being
        // written to the string
        stream.flush()?;
        Ok(())
    }

    pub fn await_request(&mut self) -> io::Result<Request> {
        let buffer = self.await_str()?;
        let request = Request::from_str(&buffer).unwrap();
        Ok(request)
    }

    pub fn await_response(&mut self) -> io::Result<Response> {
        let buffer = self.await_str()?;
        let request = Response::from_str(&buffer).unwrap();
        Ok(request)
    }

    fn await_str(&mut self) -> io::Result<String> {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        Ok(buffer)
    }
}
