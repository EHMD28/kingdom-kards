use std::fmt;

use serde::{Deserialize, Serialize};

use crate::model::card::Card;

#[derive(Serialize, Deserialize)]
pub struct Action {
    from_player: String,
    to_player: Option<String>,
    action_type: ActionType,
}

impl Action {
    pub fn new(from_player: &str, to_player: Option<&str>, action_type: ActionType) -> Action {
        let to_player = to_player.map(|s| s.to_owned());
        Action {
            from_player: from_player.to_owned(),
            to_player,
            action_type,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_string(self).unwrap();
        write!(f, "{s}")
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.from_player == other.from_player
            && self.to_player == other.to_player
            && self.action_type == other.action_type
    }
}

impl Eq for Action {}

/// The type of action between the client and server (join, game state, playing card, etc.).
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionType {
    /// A client tries to join the server.
    Join,
    /// A client requests the current game state (player names and points).
    GetGameState,
    /// A player plays a King. The inner value is the attachment.
    PlayKing(u8),
    /// A player plays a Queen. The inner value is the attachment.
    PlayQueen(u8),
    /// A player plays a Jack.
    PlayJack,
    /// A player plays an Ace. The inner value is the card they added back.
    PlayAce(Card),
}

#[cfg(test)]
mod tests {
    use crate::model::{
        action::{Action, ActionType},
        card::{Card, Suit, Value},
    };

    const ALICE_SERIALIZED_STRING: &str =
        r#"{"from_player":"Alice","to_player":null,"action_type":"Join"}"#;
    const BOB_SERIALIZED_STRING: &str =
        r#"{"from_player":"Bob","to_player":"Alice","action_type":{"PlayKing":8}}"#;
    const CHARLIE_SERIALIZED_STRING: &str = r#"{"from_player":"Charlie","to_player":null,"action_type":{"PlayAce":{"suit":"Spades","value":{"Number":2}}}}"#;

    #[test]
    fn action_serialization() {
        let alice_action = Action::new("Alice", None, ActionType::Join);
        assert_eq!(alice_action.to_string(), ALICE_SERIALIZED_STRING);

        let bob_action = Action::new("Bob", Some("Alice"), ActionType::PlayKing(8));
        assert_eq!(bob_action.to_string(), BOB_SERIALIZED_STRING);

        let charlie_action = Action::new(
            "Charlie",
            None,
            ActionType::PlayAce(Card::new(Suit::Spades, Value::Number(2))),
        );
        assert_eq!(charlie_action.to_string(), CHARLIE_SERIALIZED_STRING)
    }

    #[test]
    fn action_deserialization() {
        let alice_action = Action::new("Alice", None, ActionType::Join);
        let deserialized: Action = serde_json::from_str(ALICE_SERIALIZED_STRING).unwrap();
        assert!(
            deserialized == alice_action,
            "Expected {}. Received {}",
            alice_action,
            deserialized
        );

        let bob_action = Action::new("Bob", Some("Alice"), ActionType::PlayKing(8));
        let deserialized: Action = serde_json::from_str(BOB_SERIALIZED_STRING).unwrap();
        assert!(
            deserialized == bob_action,
            "Expected {}. Received {}",
            bob_action,
            deserialized
        );

        let charlie_action = Action::new(
            "Charlie",
            None,
            ActionType::PlayAce(Card::new(Suit::Spades, Value::Number(2))),
        );
        let deserialized: Action = serde_json::from_str(CHARLIE_SERIALIZED_STRING).unwrap();
        assert!(
            deserialized == charlie_action,
            "Expected {}. Received {}",
            charlie_action,
            deserialized
        )
    }
}
