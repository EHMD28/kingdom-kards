use serde::{Deserialize, Serialize};

use crate::model::{action::Action, game_state::GameState};

#[derive(Serialize, Deserialize)]
pub enum RequestType {
    Join(String),
    GameState,
    Action,
}

#[derive(Serialize, Deserialize)]
pub struct Request(RequestType);

#[derive(Serialize, Deserialize)]
pub enum ResponseType {
    Join(bool),
    GameState(GameState),
    Action(Action),
}

#[derive(Serialize, Deserialize)]
pub struct Response(ResponseType);
