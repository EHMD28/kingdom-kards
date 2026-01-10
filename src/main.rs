use crate::model::{Card, Suit, Value};

mod client;
mod model;
mod server;

fn main() {
    let card = Card::new(Suit::Spades, Value::Number(4));
    println!("{card}");
}
