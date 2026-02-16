use crate::model::{action::Action, game_state::GameState};

pub enum RequestType {
    Join(String),
    GameState,
    Action,
}

pub struct Request(RequestType);

pub enum ResponseType {
    Join(bool),
    GameState(GameState),
    Action(Action),
}

pub struct Response(ResponseType);
