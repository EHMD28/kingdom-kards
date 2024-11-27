use kingdom_kards::game::game_state::GameState;
use kingdom_kards::game::player::*;

fn main() {
    let player_one = Player::new(String::from("John Smith"));
    let player_two = Player::new(String::from("Jane Doe"));
    let mut g_state = GameState::new();

    player_one._print_hand();
    player_one._print_hand_unicode();
    println!();

    g_state.add_player(player_one);
    g_state.add_player(player_two);

    // println!("\nGame State\n");
    // g_state._print_players();
}
