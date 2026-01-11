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

struct Deck(Vec<Card>);

impl Deck {
    pub fn cards(&self) -> &Vec<Card> {
        &self.0
    }

    pub fn cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self.0
    }

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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::model::{Card, Deck, Suit, Value};

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
}
