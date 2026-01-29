use std::fmt;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// The suit of a player card.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Suit {
    fn all_suits() -> Vec<Suit> {
        vec![Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds]
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self {
            Suit::Spades => "Spades",
            Suit::Clubs => "Clubs",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
        };
        write!(f, "{suit}")
    }
}

/// The value of a playing card (ace, number, queen, etc.).
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Value {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Value {
    fn all_values() -> Vec<Value> {
        vec![
            Value::Ace,
            Value::Number(2),
            Value::Number(3),
            Value::Number(4),
            Value::Number(5),
            Value::Number(6),
            Value::Number(7),
            Value::Number(8),
            Value::Number(9),
            Value::Number(10),
            Value::Jack,
            Value::Queen,
            Value::King,
        ]
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Value::Ace => "Ace",
            Value::Number(num) => match num {
                2 => "Two",
                3 => "Three",
                4 => "Four",
                5 => "Five",
                6 => "Six",
                7 => "Seven",
                8 => "Eight",
                9 => "Nine",
                10 => "Ten",
                _ => unreachable!(),
            },
            Value::Jack => "Jack",
            Value::Queen => "Queen",
            Value::King => "King",
        };
        write!(f, "{value}")
    }
}

/// Two possible colors of cards: red and black.
enum Color {
    Black,
    Red,
}

impl Color {
    /// Create a color from a suit.
    fn from_suit(suit: &Suit) -> &Color {
        match suit {
            Suit::Spades | Suit::Clubs => &Color::Black,
            Suit::Hearts | Suit::Diamonds => &Color::Red,
        }
    }
}

/// The struct for representing a playing card.
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    /// Create a new card from `suit` and `value`.
    pub fn new(suit: Suit, value: Value) -> Card {
        Card { suit, value }
    }

    /// Returns an immutable reference to this card's suit.
    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    /// Returns an immutable reference to this card's value
    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn as_colored_str(&self) -> String {
        let color = Color::from_suit(self.suit());
        let color = match color {
            Color::Black => "\x1b[1;30m",
            Color::Red => "\x1b[1;31m",
        };
        let reset_color = "\x1b[0m";
        format!("{color}{} of {}{reset_color}", self.value(), self.suit())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = Color::from_suit(self.suit());
        let color = match color {
            Color::Black => "\x1b[1;30m",
            Color::Red => "\x1b[1;30m",
        };
        let reset_color = "\x1b[0m";
        write!(f, "{} of {}", self.value(), self.suit())
    }
}

/// Adds `cards()` and `cards_mut()` methods to a struct of the format StructName(Vec<Card>).
macro_rules! impl_card_container {
    ($struct_name:ident) => {
        impl $struct_name {
            /// Returns an immutable reference to the cards.
            pub fn cards(&self) -> &Vec<Card> {
                &self.0
            }

            /// Returns a mutable reference to the cards.
            pub fn cards_mut(&mut self) -> &mut Vec<Card> {
                &mut self.0
            }
        }
    };
}

pub struct Deck(Vec<Card>);
impl_card_container!(Deck);

impl Deck {
    /// Returns a standard, 52-card, shuffled playing card deck.
    pub fn shuffled() -> Deck {
        let cards = Vec::with_capacity(52);
        let mut deck = Deck(cards);
        deck.init_cards();
        deck.shuffle();
        deck
    }

    /// Initializes all cards (4 suits, 13 values per suit).
    fn init_cards(&mut self) {
        for suit in Suit::all_suits().iter() {
            for value in Value::all_values().iter() {
                self.cards_mut()
                    .push(Card::new(suit.to_owned(), value.to_owned()));
            }
        }
    }

    pub fn shuffle(&mut self) {
        self.cards_mut().shuffle(&mut rand::rng());
    }
}

impl Default for Deck {
    /// Returns a standard, 52-card, unshuffled playing card deck
    fn default() -> Self {
        let cards = Vec::with_capacity(52);
        let mut deck = Deck(cards);
        deck.init_cards();
        deck
    }
}

pub struct Hand(Vec<Card>);
impl_card_container!(Hand);

impl Hand {
    fn draw_cards_from_deck(&mut self, deck: &mut Deck, num_times: u8) {
        if deck.cards().len() < num_times.into() {
            unimplemented!("Discard pile should be shuffled into deck")
        } else {
            for _ in 0..num_times {
                let card = deck.cards_mut().pop().unwrap();
                self.cards_mut().push(card);
            }
        }
    }
}

impl Default for Hand {
    fn default() -> Hand {
        Hand(Vec::with_capacity(5))
    }
}

pub struct DiscardPile(Vec<Card>);
impl_card_container!(DiscardPile);

impl Default for DiscardPile {
    fn default() -> DiscardPile {
        DiscardPile(Vec::with_capacity(52))
    }
}

pub struct Player {
    name: String,
    points: u16,
    deck: Deck,
    hand: Hand,
    discard_pile: DiscardPile,
}

impl Player {
    pub fn new(name: &str) -> Player {
        let mut deck = Deck::shuffled();
        let mut hand = Hand::default();
        hand.draw_cards_from_deck(&mut deck, 5);
        Player {
            name: name.to_owned(),
            points: 100,
            deck,
            hand,
            discard_pile: DiscardPile::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn points(&self) -> u16 {
        self.points
    }

    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn deck_size(&self) -> usize {
        self.deck.cards().len()
    }

    pub fn hand_size(&self) -> usize {
        self.hand.cards().len()
    }

    pub fn discard_pile_len(&self) -> usize {
        self.discard_pile.cards().len()
    }
}

#[derive(Default)]
struct GameState {
    players: Vec<Player>,
    current_player: usize,
}

impl GameState {
    pub fn from_players(players: Vec<Player>) -> GameState {
        GameState {
            players,
            current_player: 0,
        }
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }
}

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

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, iter::zip};

    use crate::model::{Action, ActionType, Card, Deck, GameState, Player, Suit, Value};

    #[test]
    fn correct_deck_content() {
        let deck = Deck::default();
        assert_eq!(deck.cards().len(), 52);
        let card_strings: HashSet<String> =
            deck.cards().iter().map(|card| card.to_string()).collect();
        assert_eq!(card_strings.len(), 52, "Deck contains duplicate cards");
    }

    #[test]
    fn card_display() {
        assert_eq!(
            Card::new(Suit::Spades, Value::Ace).to_string(),
            "Ace of Spades"
        );
        assert_eq!(
            Card::new(Suit::Clubs, Value::Number(2)).to_string(),
            "Two of Clubs"
        );
        assert_eq!(
            Card::new(Suit::Hearts, Value::Jack).to_string(),
            "Jack of Hearts"
        );
        assert_eq!(
            Card::new(Suit::Diamonds, Value::King).to_string(),
            "King of Diamonds"
        );
    }

    #[test]
    fn player_intialization() {
        let player = Player::new("Alice");
        assert_eq!(player.name, "Alice");
        assert_eq!(player.points, 100);
        assert_eq!(player.hand_size(), 5);
        assert_eq!(player.deck_size(), 52 - player.hand_size());
        assert_eq!(player.discard_pile_len(), 0);
    }

    #[test]
    fn game_state_initalization() {
        let player_names = ["Alice", "Bob", "Charlie"];
        let players: Vec<Player> = player_names.iter().map(|name| Player::new(name)).collect();
        let game_state = GameState::from_players(players);
        let game_state_names: Vec<String> = game_state
            .players
            .iter()
            .map(|player| player.name().to_owned())
            .collect();
        let names = zip(player_names, game_state_names);
        for (p_name, gs_name) in names {
            assert_eq!(p_name, gs_name);
        }
        assert_eq!(game_state.current_player, 0);
    }

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
            "Expected {}. Receieved {}",
            charlie_action,
            deserialized
        )
    }
}
