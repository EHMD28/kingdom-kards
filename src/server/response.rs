use std::fmt::Display;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use crate::server::utils::is_zeroed;

use super::ServerError;

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

impl Display for ResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let response_type = match self {
            ResponseType::Name(_) => "NAME",
            ResponseType::PlayerAction(_) => "ACT",
        };

        write!(f, "{response_type}")
    }
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

pub fn send_response(stream: &mut TcpStream, response: Response) -> std::io::Result<()> {
    let response = response.to_string();
    let response = response.as_bytes();
    stream.write_all(response)?;
    Ok(())
}

pub fn await_response(
    stream: &mut TcpStream,
    response_type: ResponseType,
) -> Result<Response, ServerError> {
    let mut buffer = [0u8; 512];

    while is_zeroed(&buffer) {
        if let Err(e) = stream.read(&mut buffer) {
            if e.kind() != ErrorKind::Interrupted {
                return Err(ServerError::IoError(e));
            }
        }

        thread::sleep(Duration::from_millis(500));
    }

    let received = String::from_utf8_lossy(&buffer);
    let received = received.trim_matches('\0');
    let response = Response::from_str(received);

    match response {
        Ok(response) => {
            if *response.response_type() != response_type {
                Err(ServerError::ExpectedResponseType(ResponseType::Name(
                    "".to_string(),
                )))
            } else {
                Ok(response)
            }
        }
        Err(err) => match err {
            ResponseParseError::TooFewArguments => Err(ServerError::ReponseError(err)),
            ResponseParseError::NotAResponse => Err(ServerError::ReponseError(err)),
            ResponseParseError::InvalidType => Err(ServerError::ReponseError(err)),
            ResponseParseError::ExpectedName => Err(ServerError::ReponseError(err)),
            ResponseParseError::UnableToParseAction => Err(ServerError::ReponseError(err)),
        },
    }
}
