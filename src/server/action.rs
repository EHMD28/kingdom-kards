//! This module is responsible for converting between actions for use with the
//! game engine and actions as they are used by the server. Actions are used
//! for communicating between client and server. Actions are transmitted as
//! strings which can then be turned back into Action structs.

use crate::game::card::{Card, Color, Value};
use std::fmt;

pub trait IsAction {
    fn to_action(&self, attachment: u16, from_player: &str, to_player: &str) -> Action;
}

pub enum ActionType {
    None,
    PlayBlackAce,
    PlayRedAce,
    PlayNumber,
    PlayJack,
    PlayQueen,
    PlayKing,
}

impl ActionType {
    pub fn to_symbol(&self) -> &str {
        match self {
            ActionType::None => panic!("Invalid conversion from type to symbol"),
            ActionType::PlayBlackAce => "B",
            ActionType::PlayRedAce => "R",
            ActionType::PlayNumber => "N",
            ActionType::PlayJack => "J",
            ActionType::PlayQueen => "Q",
            ActionType::PlayKing => "K",
        }
    }
}

pub struct Action {
    pub action: ActionType,
    pub attachment: u16,
    pub from_player: String,
    pub to_player: String,
}

impl Action {
    pub fn card_to_action_type(card: &Card) -> ActionType {
        match card.get_value() {
            Value::Ace => match card.get_color() {
                Color::Black => ActionType::PlayBlackAce,
                Color::Red => ActionType::PlayRedAce,
            },
            Value::Two
            | Value::Three
            | Value::Four
            | Value::Five
            | Value::Six
            | Value::Seven
            | Value::Eight
            | Value::Nine
            | Value::Ten => ActionType::PlayNumber,
            Value::Jack => ActionType::PlayJack,
            Value::Queen => ActionType::PlayQueen,
            Value::King => ActionType::PlayKing,
        }
    }
}

impl fmt::Display for Action {
    /// Converts from an action to a string which is then sent to the server.
    /// The format of the string is: `ACT {SYMBOL} {ATTACHMENT} "{FROM_PLAYER}" "{TO_PLAYER}"`.
    ///
    /// `{SYMBOL}` can be B for Black Ace, R for Red Ace, N for Number, J for Jack, Q for Queen,
    /// or K for King.
    ///
    /// `{ATTACHMENT}` includes any number attachment for Kings and Queens. For
    /// the sake of simplicity, Numbers are treated as a type with their value as an attachment.
    ///
    /// `{FROM_PLAYER}` and `{TO_PLAYER}` are the names of the respective players as strings
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = ActionType::to_symbol(&self.action);
        write!(
            f,
            "ACT {} {} \"{}\" \"{}\"",
            symbol, self.attachment, self.from_player, self.to_player
        )
    }
}
