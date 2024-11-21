mod game;

use game::player::Player;

fn main() {
    let player_one = Player::new();
    println!("Deck Size: {}", player_one.get_deck_size());
    println!("Points: {}", player_one.get_points());
    player_one._print_hand();
}
