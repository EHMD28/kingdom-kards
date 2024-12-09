//! This module is responsible for converting between actions for use with the
//! game engine and actions as they are used by the server. Actions are used
//! for communicating between client and server. Actions are transmitted as
//! strings which can then be turned back into Action structs.

use crate::game::card::{Card, Color, Value};
use std::{fmt, str::FromStr};

pub trait ToAction {
    fn to_action(&self, attachment: u16, from_player: &str, to_player: &str) -> Action;
}

/// This enum contains each possible type of action that the server can handle.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum ActionType {
    None,
    PlayBlackAce,
    PlayRedAce,
    PlayNumber,
    PlayJack,
    PlayQueen,
    PlayKing,
    TurnStart,
    TurnEnd,
    GetDetails,
}

impl ActionType {
    /// ## Panics
    ///
    /// This function will panic if supplied an `ActionType::None` value.
    pub fn to_symbol(&self) -> &str {
        match self {
            ActionType::None => panic!("Invalid conversion from type to symbol"),
            ActionType::PlayBlackAce => "B",
            ActionType::PlayRedAce => "R",
            ActionType::PlayNumber => "N",
            ActionType::PlayJack => "J",
            ActionType::PlayQueen => "Q",
            ActionType::PlayKing => "K",
            ActionType::TurnStart => "S",
            ActionType::TurnEnd => "E",
            ActionType::GetDetails => "D",
        }
    }

    /// Converts from a one character string to ActionType enum.
    ///
    /// ## Panics
    ///
    /// This function will panic if it recieves a symbol that cannot be
    /// converted to an ActionType;
    pub fn symbol_to_type(symbol: &str) -> ActionType {
        match symbol {
            "K" => ActionType::PlayKing,
            "Q" => ActionType::PlayQueen,
            "J" => ActionType::PlayJack,
            "N" => ActionType::PlayNumber,
            "R" => ActionType::PlayRedAce,
            "B" => ActionType::PlayBlackAce,
            "S" => ActionType::TurnStart,
            "E" => ActionType::TurnEnd,
            "D" => ActionType::GetDetails,
            _ => {
                panic!("Cannot convert symbol to type");
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Action {
    pub action: ActionType,
    pub attachment: u16,
    pub from_player: String,
    pub to_player: String,
}

impl Action {
    pub fn new(
        action: ActionType,
        attachment: u16,
        from_player: String,
        to_player: String,
    ) -> Self {
        Action {
            action,
            attachment,
            from_player,
            to_player,
        }
    }

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

    /// Converts an action to an in-game messsage. For example, an `Action` struct with the fields
    /// `{ action: PlayKing, attatchment: 8, "John Smith", "Jane Doe" }` is printed out as
    /// `John Smith plays a King with 8, targeting Jane Doe.`
    /// `Jane Doe takes 8 damage, going from 100 points to 92.`
    pub fn to_pretty_string() -> String {
        todo!()
    }
}

pub enum Error {
    InvalidFirstWord,
    CantParseNum,
}

impl FromStr for Action {
    type Err = Error;

    /// Takes a string in the form `ACT,{SYMBOL},{ATTACHMENT},"{FROM_PLAYER}","{TO_PLAYER}"`
    /// and converts into an `Action` struct.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();

        if parts.len() != 5 {
            panic!("String not formatted properly. Wrong amount of parameters");
        }

        let first = parts.first().unwrap();
        if first != &"ACT" {
            return Err(Error::InvalidFirstWord);
        }

        let symbol = parts.get(1).unwrap();
        let symbol = ActionType::symbol_to_type(symbol);

        let attachment_part = parts.get(2).unwrap();
        let attachment: u16;
        if let Ok(value) = attachment_part.parse::<u16>() {
            attachment = value;
        } else {
            return Err(Error::CantParseNum);
        }

        let from_player = parts.get(3).unwrap();
        let to_player = parts.get(4).unwrap();

        Ok(Action {
            action: symbol,
            attachment,
            from_player: String::from(*from_player),
            to_player: String::from(*to_player),
        })
    }
}

impl fmt::Display for Action {
    /// Converts from an action to a string which is then sent to the server.
    /// The format of the string is: `ACT,{SYMBOL},{ATTACHMENT},"{FROM_PLAYER}","{TO_PLAYER}"`.
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
            "ACT,{},{},{},{}",
            symbol, self.attachment, self.from_player, self.to_player
        )
    }
}
