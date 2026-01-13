use crate::model::Player;

mod client;
mod model;
mod server;
mod ui;

fn main() {
    let player = Player::new("Alice");
    println!("Name: {}", player.name());
    println!("Points: {}", player.points());
    println!("Hand Size: {}", player.hand_size());
    println!("Deck Size: {}", player.deck_size());
}
