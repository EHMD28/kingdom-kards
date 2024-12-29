//! All communication between client and server is performed via `Request` and `Response` structs,
//! which are serialized as strings for the ease of data transmission. Both requests are responses
//! as strings is a list of comma separated values. The first value is either `REQ` or `RES`.

use std::{fmt::Display, str::FromStr};

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

#[derive(PartialEq, Debug)]
pub struct Action {
    action_type: ActionType,
    attachment: u16,
    from_player: String,
    to_player: String,
}

impl Action {
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

    pub fn action_type(&self) -> &ActionType {
        &self.action_type
    }

    pub fn attachment(&self) -> &u16 {
        &self.attachment
    }

    pub fn from_player(&self) -> &str {
        &self.from_player
    }

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
    /// Format: `"RES,NAME,{NAME}"`.
    Name(String),
    /// Format `"RES,ACT,{SYMBOL},{ATTATCHMENT},{FROM_PLAYER},{TO_PLAYER}"`.
    /// Types of actions are `K(ing), Q(ueen), J(ack), N(umber), B(lack Ace), R(ed Ace),
    /// (Turn) S(tart), (Turn) E(nd)`.
    PlayerAction(Action),
}

impl FromStr for ResponseType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NAME" => Ok(ResponseType::Name(String::default())),
            "ACT" => Ok(ResponseType::PlayerAction(Action::default())),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Response {
    response_type: ResponseType,
}

impl Response {
    pub fn new(response_type: ResponseType) -> Response {
        Response { response_type }
    }

    pub fn response_type(&self) -> &ResponseType {
        &self.response_type
    }
}

impl Display for Response {
    /// Format: `RES,{RESPONSE_TYPE},{...ARGUMENTS}`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let response = match &self.response_type {
            ResponseType::Name(name) => format!("RES,NAME,{name}"),
            ResponseType::PlayerAction(action) => format!(
                "RES,ACT,{},{},{},{}",
                action.action_type.to_symbol(),
                action.attachment,
                action.from_player,
                action.to_player
            ),
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
    UnableToParseAction,
}

impl FromStr for Response {
    type Err = ResponseParseError;

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
                        response_type: ResponseType::Name(name.to_string()),
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
                        response_type: ResponseType::PlayerAction(Action {
                            action_type: action.action_type,
                            attachment: action.attachment,
                            from_player: action.from_player,
                            to_player: action.to_player,
                        }),
                    }),
                    Err(_) => Err(ResponseParseError::UnableToParseAction),
                }
            }
        }
    }
}
