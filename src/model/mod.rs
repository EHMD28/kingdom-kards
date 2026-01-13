use std::fmt;

use rand::seq::SliceRandom;

#[derive(Clone)]
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

#[derive(Clone)]
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

pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Card {
        Card { suit, value }
    }

    fn suit(&self) -> &Suit {
        &self.suit
    }

    fn value(&self) -> &Value {
        &self.value
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

struct Deck(Vec<Card>);
impl_card_container!(Deck);

impl Deck {
    /// Returns a standard, 52-card, shuffled playing card deck.
    fn shuffled() -> Deck {
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

    fn shuffle(&mut self) {
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

struct Hand(Vec<Card>);
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

struct DiscardPile(Vec<Card>);
impl_card_container!(DiscardPile);

impl Default for DiscardPile {
    fn default() -> DiscardPile {
        DiscardPile(Vec::with_capacity(52))
    }
}

struct Player {
    name: String,
    points: u16,
    deck: Deck,
    hand: Hand,
    discard_pile: DiscardPile,
}

impl Player {
    fn new(name: &str) -> Player {
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
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::model::{Card, Deck, Player, Suit, Value};

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
        assert_eq!(player.hand.cards().len(), 5);
        assert_eq!(player.deck.cards().len(), 52 - player.hand.cards().len());
        assert_eq!(player.discard_pile.cards().len(), 0);
    }
}
