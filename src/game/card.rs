//! The Card type is responsible for holding all of the data related to a specific
//! playing card, as well as methods for creating and getting information from the
//! playing cards.

use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Suit::Spades => "Spades",
            Suit::Clubs => "Clubs",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
        };
        write!(f, "{suit}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Value {
    pub fn is_number(&self) -> bool {
        matches!(
            self,
            Value::Two
                | Value::Three
                | Value::Four
                | Value::Five
                | Value::Six
                | Value::Seven
                | Value::Eight
                | Value::Nine
                | Value::Ten
        )
    }

    pub fn to_number_value(&self) -> u16 {
        match self {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            _ => unreachable!(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Value::Ace => "Ace",
            Value::Two => "Two",
            Value::Three => "Three",
            Value::Four => "Four",
            Value::Five => "Five",
            Value::Six => "Six",
            Value::Seven => "Seven",
            Value::Eight => "Eight",
            Value::Nine => "Nine",
            Value::Ten => "Ten",
            Value::Jack => "Jack",
            Value::Queen => "Queen",
            Value::King => "King",
        };
        write!(f, "{value}")
    }
}

pub enum Color {
    Black,
    Red,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Color::Black => "Black",
            Color::Red => "Red",
        };
        write!(f, "{color}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

/* used for coloring the cards as Unicode */
// const COLOR_BLACK: &str = "\x1b[47;30m";
// const COLOR_RED: &str = "\x1b[47;31m";
// const COLOR_RESET: &str = "\x1b[0m";

fn color_to_ansi_code(color: &Color) -> &'static str {
    match color {
        Color::Black => "\x1b[0;30m",
        Color::Red => "\x1b[0;31m",
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} of {}", self.color(), self.value, self.suit)
    }
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self { suit, value }
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn color(&self) -> &Color {
        match self.suit {
            Suit::Spades | Suit::Clubs => &Color::Black,
            Suit::Diamonds | Suit::Hearts => &Color::Red,
        }
    }

    pub fn to_colored_text(&self) -> String {
        let color = color_to_ansi_code(self.color());
        let value = self.value;
        let suit = self.suit;
        let color_reset = "\x1b[0m";
        format!("{color}{value} of {suit}{color_reset}")
    }

    pub fn to_unicode(&self) -> &str {
        match self.suit {
            Suit::Spades => match self.value {
                Value::Ace => "🂡",
                Value::Two => "🂢",
                Value::Three => "🂣",
                Value::Four => "🂤",
                Value::Five => "🂥",
                Value::Six => "🂦",
                Value::Seven => "🂧",
                Value::Eight => "🂨",
                Value::Nine => "🂩",
                Value::Ten => "🂪",
                Value::Jack => "🂫",
                Value::Queen => "🂭",
                Value::King => "🂮",
            },
            Suit::Clubs => match self.value {
                Value::Ace => "🃑",
                Value::Two => "🃒",
                Value::Three => "🃓",
                Value::Four => "🃔",
                Value::Five => "🃕",
                Value::Six => "🃖",
                Value::Seven => "🃗",
                Value::Eight => "🃘",
                Value::Nine => "🃙",
                Value::Ten => "🃚",
                Value::Jack => "🃛",
                Value::Queen => "🃝",
                Value::King => "🃞",
            },
            Suit::Hearts => match self.value {
                Value::Ace => "🂱",
                Value::Two => "🂲",
                Value::Three => "🂳",
                Value::Four => "🂴",
                Value::Five => "🂵",
                Value::Six => "🂶",
                Value::Seven => "🂷",
                Value::Eight => "🂸",
                Value::Nine => "🂹",
                Value::Ten => "🂺",
                Value::Jack => "🂻",
                Value::Queen => "🂽",
                Value::King => "🂾",
            },
            Suit::Diamonds => match self.value {
                Value::Ace => "🃁",
                Value::Two => "🃂",
                Value::Three => "🃃",
                Value::Four => "🃄",
                Value::Five => "🃅",
                Value::Six => "🃆",
                Value::Seven => "🃇",
                Value::Eight => "🃈",
                Value::Nine => "🃉",
                Value::Ten => "🃊",
                Value::Jack => "🃋",
                Value::Queen => "🃍",
                Value::King => "🃎",
            },
        }
    }
}
