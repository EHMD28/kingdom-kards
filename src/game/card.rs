//! The Card type is responsible for holding all of the data related to a specific
//! playing card, as well as methods for creating and getting information from the
//! playing cards.

use std::fmt::Display;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

/* used for coloring the cards as Unicode */
const COLOR_BLACK: &str = "\x1b[47;30m";
const COLOR_RED: &str = "\x1b[47;31m";
const COLOR_RESET: &str = "\x1b[0m";

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self { suit, value }
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_value(&self) -> Value {
        self.value
    }

    pub fn get_color(&self) -> Color {
        match self.suit {
            Suit::Spades | Suit::Clubs => Color::Black,
            Suit::Diamonds | Suit::Hearts => Color::Red,
        }
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

    pub fn _print_self(&self) {
        println!("{} {} of {}", self.get_color(), self.value, self.suit);
    }

    pub fn _print_self_unicode(&self) {
        match self.get_color() {
            Color::Black => {
                print!("{COLOR_BLACK}{} {COLOR_RESET}", self.to_unicode());
            }
            Color::Red => {
                print!("{COLOR_RED}{} {COLOR_RESET}", self.to_unicode());
            }
        }
    }
}
