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

    pub fn print_self(&self) {
        println!("{} {} of {}", self.get_color().to_string(),
                                self.value.to_string(),
                                self.suit.to_string());
    }
}
