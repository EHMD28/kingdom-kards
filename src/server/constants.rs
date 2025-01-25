//! This module contains constants for anything server related.

use super::{
    request::{Request, RequestType},
    response::{Response, ResponseType, StatusType},
};

pub const NAME_REQUEST: Request = Request::new(RequestType::Name);
pub const ACTION_REQUEST: Request = Request::new(RequestType::PlayerAction);
pub const STATUS_REQUEST: Request = Request::new(RequestType::Status);
pub const DETAILS_REQUEST: Request = Request::new(RequestType::Details);

pub const NAME_RESPONSE: Response = Response::new(ResponseType::Name(None));
pub const ACTION_RESPONSE: Response = Response::new(ResponseType::PlayerAction(None));
pub const STATUS_RESPONSE: Response = Response::new(ResponseType::Status(None));
pub const STATUS_RESPONSE_YES: Response =
    Response::new(ResponseType::Status(Some(StatusType::Yes)));
pub const STATUS_RESPONSE_NO: Response = Response::new(ResponseType::Status(Some(StatusType::No)));
pub const DETAILS_RESPONSE: Response = Response::new(ResponseType::Details(None));
