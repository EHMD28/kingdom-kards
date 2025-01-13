use std::fmt::Display;
use std::io::{BufRead, BufReader, Read, Write};
use std::mem::zeroed;
use std::net::TcpStream;
use std::str::FromStr;

/// Used for asking the server whether an operation is valid or not.
/// `Yes` means the operation is fine, and `No` means the operation
/// is invalid.
#[derive(PartialEq, Debug)]
pub enum StatusType {
    Yes,
    No,
}

impl Display for StatusType {
    /// When converting from `StatusType` to string, only the first letter is used.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            StatusType::Yes => "Y",
            StatusType::No => "N",
        };

        write!(f, "{status}")
    }
}

impl FromStr for StatusType {
    type Err = ();

    /// Converts from status ("Y" or "N") to StatusType.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Y" => Ok(StatusType::Yes),
            "N" => Ok(StatusType::No),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ActionType {
    PlayKing,
    PlayQueen,
    PlayJack,
    PlayNumber,
    PlayBlackAce,
    PlayRedAce,
    TurnStart,
    TurnEnd,
}

impl ActionType {
    /// Converts from `ActionType` to the symbol that is used for string
    /// serialization.
    fn to_symbol(&self) -> &str {
        match self {
            ActionType::PlayKing => "K",
            ActionType::PlayQueen => "Q",
            ActionType::PlayJack => "J",
            ActionType::PlayNumber => "N",
            ActionType::PlayBlackAce => "B",
            ActionType::PlayRedAce => "R",
            ActionType::TurnStart => "S",
            ActionType::TurnEnd => "E",
        }
    }

    /// Converts from symbol that is used for string serialization back to
    /// ActionType. Returns `None` if symbol is invalid.
    fn from_symbol(symbol: &str) -> Option<ActionType> {
        match symbol {
            "K" => Some(ActionType::PlayKing),
            "Q" => Some(ActionType::PlayQueen),
            "J" => Some(ActionType::PlayJack),
            "N" => Some(ActionType::PlayNumber),
            "B" => Some(ActionType::PlayBlackAce),
            "R" => Some(ActionType::PlayRedAce),
            "S" => Some(ActionType::TurnStart),
            "E" => Some(ActionType::TurnEnd),
            _ => None,
        }
    }
}

/// Used for representing an action type that can be interperet
/// both client-side and server-side to advance the game state.
#[derive(PartialEq, Debug)]
pub struct Action {
    action_type: ActionType,
    attachment: u16,
    from_player: String,
    to_player: String,
}

impl Action {
    /// Returns a new instance of `Action` using parameters.
    pub fn new(
        action_type: ActionType,
        attachment: u16,
        from_player: String,
        to_player: String,
    ) -> Action {
        Action {
            action_type,
            attachment,
            from_player,
            to_player,
        }
    }

    /// Returns a reference to `self.action_type`.
    pub fn action_type(&self) -> &ActionType {
        &self.action_type
    }

    /// Returns a reference `self.attachment`.
    pub fn attachment(&self) -> &u16 {
        &self.attachment
    }

    /// Returns a reference to `self.from_player`.
    pub fn from_player(&self) -> &str {
        &self.from_player
    }

    /// Returns a reference to `self.to_player`.
    pub fn to_player(&self) -> &str {
        &self.to_player
    }
}

impl Default for Action {
    fn default() -> Self {
        Self {
            action_type: ActionType::TurnEnd,
            attachment: 0,
            from_player: String::default(),
            to_player: String::default(),
        }
    }
}

#[allow(clippy::enum_variant_names)]
pub enum ActionParseError {
    InvalidType,
    InvalidAttatchment,
    InvalidNumArguments,
}

impl FromStr for Action {
    type Err = ActionParseError;

    /// Converts from a string in the format `{SYMBOL},{ATTACHMENT},{FROM_PLAYER},{TO_PLAYER}`
    /// to an Action.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");

        if parts.clone().count() != 4 {
            return Err(ActionParseError::InvalidNumArguments);
        }

        let first = parts.next().unwrap();
        let action = ActionType::from_symbol(first);
        let action: ActionType = match action {
            Some(v) => v,
            None => return Err(ActionParseError::InvalidType),
        };

        let attachment = parts.next().unwrap();
        let attachment: u16 = match attachment.parse() {
            Ok(v) => v,
            Err(_) => return Err(ActionParseError::InvalidAttatchment),
        };

        let from_player = parts.next().unwrap().to_string();
        let to_player = parts.next().unwrap().to_string();

        Ok(Action {
            action_type: action,
            attachment,
            from_player,
            to_player,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum ResponseType {
    /// Format: `RES,NAME,{NAME}`.
    Name(Option<String>),
    /// Format: `RES,STATUS,{Y or N}`.
    Status(Option<StatusType>),
    /// Format: `RES,ACT,{SYMBOL},{ATTATCHMENT},{FROM_PLAYER},{TO_PLAYER}`.
    /// Types of actions are `K(ing), Q(ueen), J(ack), N(umber), B(lack Ace), R(ed Ace),
    /// (Turn) S(tart), (Turn) E(nd)`.
    PlayerAction(Option<Action>),
}

impl ToOwned for ResponseType {
    type Owned = ResponseType;

    fn to_owned(&self) -> Self::Owned {
        match self {
            ResponseType::Name(_) => ResponseType::Name(None),
            ResponseType::Status(_) => ResponseType::Status(None),
            ResponseType::PlayerAction(_) => ResponseType::PlayerAction(None),
        }
    }
}

impl Display for ResponseType {
    /// Converts from `ResponseType` to string, ignoring the value
    /// contained within any of the variants.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let response_type = match self {
            ResponseType::Name(_) => "NAME",
            ResponseType::PlayerAction(_) => "ACT",
            ResponseType::Status(_) => "STATUS",
        };

        write!(f, "{response_type}")
    }
}

impl FromStr for ResponseType {
    type Err = ();

    /// Converts from a string to `ResponseType`. The value contained within the
    /// `ResponseType` is whatever the "default" value is for each variant.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NAME" => Ok(ResponseType::Name(None)),
            "ACT" => Ok(ResponseType::PlayerAction(None)),
            "STATUS" => Ok(ResponseType::Status(None)),
            _ => Err(()),
        }
    }
}

/// This is the type used for representing responses, which are used for
/// sending data to and from the server. A response should only be given in
/// resposne to a request that was received.
#[derive(PartialEq, Debug)]
pub struct Response {
    response_type: ResponseType,
}

impl Response {
    /// Creates a new `Response` of type `response_type`.
    pub const fn new(response_type: ResponseType) -> Response {
        Response { response_type }
    }

    /// Returns a reference to `self.response_type`.
    pub fn response_type(&self) -> &ResponseType {
        &self.response_type
    }
}

impl Display for Response {
    /// Format: `RES,{RESPONSE_TYPE},{...ARGUMENTS}`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let response = match &self.response_type {
            ResponseType::Name(name) => format!("RES,NAME,{}", name.as_ref().unwrap()),
            ResponseType::PlayerAction(action) => {
                let action = action.as_ref().unwrap();
                format!(
                    "RES,ACT,{},{},{},{}",
                    action.action_type.to_symbol(),
                    action.attachment,
                    action.from_player,
                    action.to_player
                )
            }
            ResponseType::Status(status) => {
                format!("RES,STATUS,{}", status.as_ref().unwrap())
            }
        };

        write!(f, "{response}")
    }
}

#[derive(Debug)]
pub enum ResponseParseError {
    TooFewArguments,
    NotAResponse,
    InvalidType,
    ExpectedName,
    ExpectedStatus,
    UnableToParseAction,
}

impl FromStr for Response {
    type Err = ResponseParseError;

    /// Converts from a string to a `Response`. If the conversion fails,
    /// then this function will return a `ResponseParsError`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");

        if parts.clone().count() < 2 {
            return Err(ResponseParseError::TooFewArguments);
        }

        let first = parts.next().unwrap();
        if first != "RES" {
            return Err(ResponseParseError::NotAResponse);
        }

        let response_type = parts.next().unwrap();
        let response_type = ResponseType::from_str(response_type);
        if response_type.is_err() {
            return Err(ResponseParseError::InvalidType);
        }
        let response_type = response_type.unwrap();

        match response_type {
            ResponseType::Name(_) => {
                if let Some(name) = parts.next() {
                    Ok(Response {
                        response_type: ResponseType::Name(Some(name.to_string())),
                    })
                } else {
                    Err(ResponseParseError::ExpectedName)
                }
            }
            ResponseType::PlayerAction(_) => {
                let parts: Vec<&str> = parts.collect();
                let parts = parts.join(",");
                let action = Action::from_str(&parts);

                match action {
                    Ok(action) => Ok(Response {
                        response_type: ResponseType::PlayerAction(Some(action)),
                    }),
                    Err(_) => Err(ResponseParseError::UnableToParseAction),
                }
            }
            ResponseType::Status(_) => {
                if let Some(status) = parts.next() {
                    let status = StatusType::from_str(status);
                    match status {
                        Ok(status) => Ok(Response::new(ResponseType::Status(Some(status)))),
                        Err(_) => Err(ResponseParseError::ExpectedStatus),
                    }
                } else {
                    Err(ResponseParseError::ExpectedStatus)
                }
            }
        }
    }
}

/// Constant for simplyfying awaiting and sending responses.
pub const NAME_RESPONSE: Response = Response::new(ResponseType::Name(None));
/// Constant for simplyfying awaiting and sending responses.
pub const ACTION_RESPONSE: Response = Response::new(ResponseType::PlayerAction(None));
/// Constant for simplyfying awaiting and sending responses.
pub const STATUS_RESPONSE: Response = Response::new(ResponseType::Status(None));
/// Constant for simplyfying awaiting and sending responses.
pub const STATUS_RESPONSE_YES: Response =
    Response::new(ResponseType::Status(Some(StatusType::Yes)));
/// Constant for simplyfying awaiting and sending responses.
pub const STATUS_RESPONSE_NO: Response = Response::new(ResponseType::Status(Some(StatusType::No)));

// /// For handling of sending responses and receiving requests.
// pub struct ResponseHandler<'a> {
//     reader: BufReader<&'a mut TcpStream>,
// }

// impl ResponseHandler<'_> {
//     pub fn new(stream: &mut TcpStream) -> ResponseHandler {
//         ResponseHandler {
//             reader: BufReader::new(stream),
//         }
//     }

//     // pub fn from_request_handler(mut request_handler: &mut RequestHandler) -> ResponseHandler {
//     //     let mut buffer = request_handler
//     //         .reader_mut()
//     //         .fill_buf()
//     //         .expect("Unable to fill buffer");
//     //     let buf_len = buffer.len();
//     //     buffer.consume(buf_len);

//     //     let stream = request_handler.stream_mut();
//     //     // TODO: add buffer to response_handler.
//     //     let response_handler = ResponseHandler::new(stream);

//     //     BufReader::from(BufReader::new(stream))
//     // }

//     /// Sends `response` as string over `stream`.
//     pub fn send_response(&mut self, response: Response) -> std::io::Result<()> {
//         let response_type = response.response_type();
//         let mut response = response.to_string();
//         /* Newline is used a delimiting character to avoid requests being mangled. */
//         response.push('\n');
//         let response = response.as_bytes();
//         // dbg!(response);
//         let stream = self.reader.get_mut();
//         stream.write_all(response)?;
//         stream.flush()?;
//         println!("Sent response of type {response_type}");
//         Ok(())
//     }

//     /// Blocks the current thread until a `Request` is received. If the request
//     /// received is of the wrong type, then this function will return an error.
//     pub fn await_request(&mut self, request: Request) -> Result<Request, ServerError> {
//         // todo: Fix queued requests being dropped when function exits.
//         let request_type = request.request_type().to_owned();
//         let received = &mut String::new();

//         println!("Awaiting request of type {request_type}");
//         println!("Buffer is empty: {}", self.reader.buffer().is_empty());
//         // received.clear();
//         if let Err(err) = self.reader.read_line(received) {
//             return Err(ServerError::IoError(err));
//         }
//         remove_newline(received);

//         println!("Received: {received}");
//         let request = Request::from_str(received);

//         match request {
//             Ok(request) => {
//                 if variant_eq(request.request_type(), &request_type) {
//                     Ok(request)
//                 } else {
//                     Err(ServerError::ExpectedRequestType(request_type))
//                 }
//             }
//             Err(err) => Err(ServerError::RequestError(err)),
//         }
//     }
// }

// impl From<RequestHandler<'_>> for ResponseHandler<'_> {
//     fn from(value: RequestHandler<'_>) -> Self {
//         let reader = *value.reader().to_owned();
//         let reader = BufReader::from(reader);

//     }
// }
