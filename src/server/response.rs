use core::str::Split;
use std::fmt::Display;
use std::io::{BufRead, Write};
use std::str::FromStr;

use crate::game::game_state::{GameState, PlayerDetails};
use crate::utils::{perror_in_fn, variant_eq};

use super::ServerError;

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
    /// Format `RES,GAME,{NUM_PLAYERS},{P1_NAME}:{P1_POINTS},{P2_NAME}:{P2_POINTS}...`.
    GameState(Option<GameState>),
}

impl ToOwned for ResponseType {
    type Owned = ResponseType;

    fn to_owned(&self) -> Self::Owned {
        match self {
            ResponseType::Name(_) => ResponseType::Name(None),
            ResponseType::Status(_) => ResponseType::Status(None),
            ResponseType::PlayerAction(_) => ResponseType::PlayerAction(None),
            ResponseType::GameState(_) => ResponseType::GameState(None),
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
            ResponseType::GameState(_) => "GAME",
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
            "GAME" => Ok(ResponseType::GameState(None)),
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

    pub fn from_action(action: Action) -> Response {
        Response {
            response_type: ResponseType::PlayerAction(Some(action)),
        }
    }

    pub fn from_name(name: String) -> Response {
        Response {
            response_type: ResponseType::Name(Some(name)),
        }
    }

    pub fn from_game_state(game_state: GameState) -> Response {
        Response {
            response_type: ResponseType::GameState(Some(game_state)),
        }
    }

    // pub fn new_player_details(name: String, points: u16) -> Response {
    //     Response {
    //         response_type: ResponseType::Details(Some(PlayerDetails::new(name, points))),
    //     }
    // }

    pub fn new_turn_start(pname: String) -> Response {
        Response {
            response_type: ResponseType::PlayerAction(Some(Action::new(
                ActionType::TurnStart,
                0,
                pname,
                String::new(),
            ))),
        }
    }

    pub fn new_turn_end(pname: String) -> Response {
        Response {
            response_type: ResponseType::PlayerAction(Some(Action::new(
                ActionType::TurnEnd,
                0,
                pname,
                String::new(),
            ))),
        }
    }

    /// Returns a reference to `self.response_type`.
    pub fn response_type(&self) -> &ResponseType {
        &self.response_type
    }

    pub fn validate(response: Result<Response, ServerError>, response_type: ResponseType) {
        match response {
            Ok(response) if variant_eq(response.response_type(), &response_type) => (),
            Err(err) => perror_in_fn("Response::validate", err),
            _ => unreachable!("Received response of invalid type"),
        }
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
            ResponseType::GameState(game_state) => {
                let game_state = game_state.as_ref().unwrap();
                let mut response = String::from("RES,GAME,");
                let num_players = game_state.num_players().to_string();
                response.push_str(&num_players);
                response.push(',');
                for player in game_state.all_players().iter() {
                    let name = player.name();
                    let points = player.points().to_string();
                    let player_details = format!("{name}:{points},");
                    response.push_str(&player_details);
                }
                response.pop(); /* removing trailing comma */
                response
            }
        };

        write!(f, "{response}")
    }
}

#[derive(Debug)]
pub enum ResponseParseError {
    InvalidNumArguments,
    NotAResponse,
    InvalidType,
    ExpectedName,
    ExpectedNumPlayers,
    ExpectedStatus,
    ExpectedPoints,
    UnableToParseAction,
    ParseIntError,
}

impl FromStr for Response {
    type Err = ResponseParseError;

    /// Converts from a string to a `Response`. If the conversion fails,
    /// then this function will return a `ResponseParsError`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");

        validate_parts(&mut parts)?;
        let response_type = validate_response_type(&mut parts)?;

        match response_type {
            ResponseType::Name(_) => parts_to_name(&mut parts),
            ResponseType::PlayerAction(_) => parts_to_action(&mut parts),
            ResponseType::Status(_) => parts_to_status(&mut parts),
            ResponseType::GameState(_) => parts_to_game_state(&mut parts),
        }
    }
}

fn validate_parts(parts: &mut Split<&str>) -> Result<(), ResponseParseError> {
    if parts.clone().count() < 2 {
        return Err(ResponseParseError::InvalidNumArguments);
    }

    let first = parts.next().unwrap();
    if first != "RES" {
        return Err(ResponseParseError::NotAResponse);
    }

    Ok(())
}

fn validate_response_type(parts: &mut Split<&str>) -> Result<ResponseType, ResponseParseError> {
    let response_type = parts.next().unwrap();
    let response_type = ResponseType::from_str(response_type);
    match response_type {
        Ok(response_type) => Ok(response_type),
        Err(_) => Err(ResponseParseError::InvalidType),
    }
}

fn parts_to_name(parts: &mut Split<&str>) -> Result<Response, ResponseParseError> {
    if let Some(name) = parts.next() {
        Ok(Response::from_name(name.to_string()))
    } else {
        Err(ResponseParseError::ExpectedName)
    }
}

fn parts_to_action(parts: &mut Split<&str>) -> Result<Response, ResponseParseError> {
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

fn parts_to_status(parts: &mut Split<&str>) -> Result<Response, ResponseParseError> {
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

fn parts_to_game_state(parts: &mut Split<&str>) -> Result<Response, ResponseParseError> {
    if let Some(num_players) = parts.next() {
        let num_players = match num_players.parse::<u8>() {
            Ok(n) => n,
            Err(_) => return Err(ResponseParseError::ParseIntError),
        };

        let mut game_state = GameState::new();
        for _ in 0..num_players {
            if let Some(player) = parts.next() {
                let player_details = &mut player.split(":");
                if player_details.clone().count() != 2 {
                    return Err(ResponseParseError::InvalidNumArguments);
                }
                let name = player_details.next().unwrap();
                let points = player_details.next().unwrap();
                let points = match points.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return Err(ResponseParseError::ParseIntError),
                };
                let player = PlayerDetails::new(name.to_string(), points);
                game_state.add_player(player);
            } else {
                return Err(ResponseParseError::InvalidNumArguments);
            }
        }

        Ok(Response::from_game_state(game_state))
    } else {
        Err(ResponseParseError::ExpectedNumPlayers)
    }
}

// fn parts_to_details(parts: &mut Split<&str>) -> Result<Response, ResponseParseError> {
//     if let Some(name) = parts.next() {
//         if let Some(points) = parts.next() {
//             let points = points.parse::<u16>();
//             if points.is_err() {
//                 Err(ResponseParseError::ExpectedPoints)
//             } else {
//                 let points = points.unwrap();
//                 Ok(Response::new(ResponseType::Details(Some(
//                     PlayerDetails::new(name.to_string(), points),
//                 ))))
//             }
//         } else {
//             Err(ResponseParseError::ExpectedPoints)
//         }
//     } else {
//         Err(ResponseParseError::ExpectedName)
//     }
// }

impl Default for Response {
    fn default() -> Self {
        Response::new(ResponseType::Name(None))
    }
}
