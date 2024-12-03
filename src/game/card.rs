use crate::server::action::Action;
use crate::server::action::Actions;
use crate::server::action::ToAction;

trait Stringable {
    fn to_string(self) -> &'static str;
}

#[derive(Clone, Copy)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Stringable for Suit {
    fn to_string(self) -> &'static str {
        match self {
            Suit::Spades => "Spades",
            Suit::Clubs => "Clubs",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
        }
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

impl Stringable for Value {
    fn to_string(self) -> &'static str {
        match self {
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
        }
    }
}

pub enum Color {
    Black,
    Red,
}

impl Stringable for Color {
    fn to_string(self) -> &'static str {
        match self {
            Color::Black => "Black",
            Color::Red => "Red",
        }
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
        println!(
            "{} {} of {}",
            self.get_color().to_string(),
            self.value.to_string(),
            self.suit.to_string()
        );
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

impl ToAction for Card {
    fn to_action(&self) -> Action {
        /* format: ACT {SYMBOL} {ATTACHMENT?} {FROM_PLAYER_NAME} {TO_PLAYER_NAME} */
        Action(Actions::None)
    }
}
