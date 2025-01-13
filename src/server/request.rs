use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Debug)]
pub enum RequestType {
    /// Format: `"REQ,NAME"`.
    Name,
    /// Format: `REQ,STATUS`.
    Status,
    /// Format `"REQ,ACT"`.
    PlayerAction,
}

impl ToOwned for RequestType {
    type Owned = RequestType;

    fn to_owned(&self) -> Self::Owned {
        match self {
            RequestType::Name => RequestType::Name,
            RequestType::Status => RequestType::Status,
            RequestType::PlayerAction => RequestType::PlayerAction,
        }
    }
}

impl Display for RequestType {
    /// Used for converting a request type to a string for the purposes
    /// of serialization.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            RequestType::Name => "NAME",
            RequestType::PlayerAction => "ACT",
            RequestType::Status => "STATUS",
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
    /// Creates a new `Request` of type `request_type`.
    pub const fn new(request_type: RequestType) -> Request {
        Request { request_type }
    }

    /// Returns a reference to the `RequestType` of self.
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

    /// Converts from a string to a `Request`. This function will
    /// return `Err(RequestParseError)` if request string is not properly
    /// formatted, including information about what specifically went wrong
    /// when parsing the string.
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
            "STATUS" => Ok(Request {
                request_type: RequestType::Status,
            }),
            _ => Err(RequestParseError::InvalidType),
        }
    }
}

/// Constant for simplyfying awaiting and sending requests.
pub const NAME_REQUEST: Request = Request::new(RequestType::Name);
/// Constant for simplyfying awaiting and sending requests.
pub const ACTION_REQUEST: Request = Request::new(RequestType::PlayerAction);
/// Constant for simplyfying awaiting and sending requests.
pub const STATUS_REQUEST: Request = Request::new(RequestType::Status);

// /// For handling of sending requests and receiving responses.
// pub struct RequestHandler<'a> {
//     reader: BufReader<&'a mut TcpStream>,
// }

// impl<'a> RequestHandler<'a> {
//     /// Create a new request handler using stream.
//     pub fn new(stream: &mut TcpStream) -> RequestHandler {
//         RequestHandler {
//             reader: BufReader::new(stream),
//         }
//     }

//     pub fn reader(&self) -> &BufReader<&mut TcpStream> {
//         &self.reader
//     }

//     pub fn reader_mut(&mut self) -> &'a mut BufReader<&mut TcpStream> {
//         &mut self.reader
//     }

//     pub fn stream_mut(&mut self) -> &mut TcpStream {
//         self.reader.get_mut()
//     }

//     /// Sends `request` over stream as a string.
//     pub fn send_request(&mut self, request: Request) -> std::io::Result<()> {
//         let request_type = request.request_type();
//         let mut request = request.to_string();
//         /* Newline is used a delimiting character to avoid requests being mangled. */
//         request.push('\n');
//         let request = request.as_bytes();
//         let stream = self.reader.get_mut();
//         stream.write_all(request)?;
//         stream.flush()?;
//         println!("Sent request of type {}", request_type);
//         Ok(())
//     }

//     /// Blocks the current thread until a `Response` is received. If the response
//     /// received is of the wrong type, then this function will return an error.
//     pub fn await_response(&mut self, response: Response) -> Result<Response, ServerError> {
//         let response_type = response.response_type().to_owned();
//         let received = &mut String::new();

//         println!("Awaiting response of type {response_type}");
//         /* Using read_line() because requests/responses are separated by newline delimeter */
//         // received.clear();
//         if let Err(e) = self.reader.read_line(received) {
//             return Err(ServerError::IoError(e));
//         }

//         remove_newline(received);

//         println!("Received: {received}");
//         let response = Response::from_str(received);

//         match response {
//             Ok(request) => {
//                 if variant_eq(request.response_type(), &response_type) {
//                     Ok(request)
//                 } else {
//                     Err(ServerError::ExpectedResponseType(response_type))
//                 }
//             }
//             Err(err) => Err(ServerError::ReponseError(err)),
//         }
//     }
// }

// impl From<ResponseHandler<'_>> for RequestHandler<'_> {
//     fn from(value: ResponseHandler) -> Self {
//         todo!()
//     }
// }
