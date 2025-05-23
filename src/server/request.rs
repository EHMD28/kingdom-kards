use std::{fmt::Display, str::FromStr};

use crate::utils::variant_eq;

use super::{
    response::{Response, ResponseType},
    ServerError,
};

#[derive(PartialEq, Debug)]
pub enum RequestType {
    /// Format: `REQ,NAME`.
    Name,
    /// Format: `REQ,STATUS`.
    Status,
    /// Format `REQ,ACT`.
    PlayerAction,
    /// Format: `REQ,GAME`.
    GameState,
}

impl ToOwned for RequestType {
    type Owned = RequestType;

    fn to_owned(&self) -> Self::Owned {
        match self {
            RequestType::Name => RequestType::Name,
            RequestType::Status => RequestType::Status,
            RequestType::PlayerAction => RequestType::PlayerAction,
            RequestType::GameState => RequestType::GameState,
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
            RequestType::GameState => "GAME",
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

    pub fn from_response(response: &Response) -> Request {
        let request_type = match response.response_type() {
            ResponseType::Name(_) => RequestType::Name,
            ResponseType::Status(_) => RequestType::Status,
            ResponseType::PlayerAction(_) => RequestType::PlayerAction,
            ResponseType::GameState(_) => RequestType::GameState,
        };
        Request { request_type }
    }

    /// Returns a reference to the `RequestType` of self.
    pub fn request_type(&self) -> &RequestType {
        &self.request_type
    }

    /// Checks to see if there were any errors on request, unwrapping it and returing it if there
    /// were none.
    pub fn validate(request: Result<Request, ServerError>, request_type: RequestType) {
        match request {
            Ok(request) if variant_eq(request.request_type(), &request_type) => (),
            Err(err) => eprintln!("An error occured: {err}"),
            _ => unreachable!("Received request of incorrect type."),
        }
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
            "GAME" => Ok(Request {
                request_type: RequestType::GameState,
            }),
            _ => Err(RequestParseError::InvalidType),
        }
    }
}
