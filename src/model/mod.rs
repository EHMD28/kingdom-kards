use std::fmt;

pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
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

pub enum Value {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
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
