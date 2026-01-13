use crate::model::{Card, Deck, Player, Suit, Value};

mod client;
mod model;
mod server;
mod ui;

fn main() {
    let deck = Deck::shuffled();
    for card in deck.cards().iter() {
        println!("{}", card.as_colored_str());
    }
}
