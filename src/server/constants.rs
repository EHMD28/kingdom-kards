//! This module contains constants for anything server related.

use super::{
    request::{Request, RequestType},
    response::{Response, ResponseType, StatusType},
};

// Game Constants
pub const MAX_PLAYERS: usize = 6;
pub const DECK_SIZE: usize = 52;
pub const MAX_USERNAME_LEN: usize = 25;

// Request Constants
pub const NAME_REQUEST: &Request = &Request::new(RequestType::Name);
pub const ACTION_REQUEST: &Request = &Request::new(RequestType::PlayerAction);
pub const STATUS_REQUEST: &Request = &Request::new(RequestType::Status);
pub const GAME_STATE_REQUEST: &Request = &Request::new(RequestType::GameState);

// Response Constants
pub const NAME_RESPONSE: &Response = &Response::new(ResponseType::Name(None));
pub const ACTION_RESPONSE: &Response = &Response::new(ResponseType::PlayerAction(None));
pub const STATUS_RESPONSE: &Response = &Response::new(ResponseType::Status(None));
pub const STATUS_RESPONSE_YES: &Response =
    &Response::new(ResponseType::Status(Some(StatusType::Yes)));
pub const STATUS_RESPONSE_NO: &Response =
    &Response::new(ResponseType::Status(Some(StatusType::No)));
pub const GAME_STATE_RESPONSE: &Response = &Response::new(ResponseType::GameState(None));
