use kingdom_kards::game::game_state::GameState;
use kingdom_kards::game::player::*;

fn main() {
    let mut player_one = Player::new(String::from("John Smith"));
    let player_two = Player::new(String::from("Jane Doe"));
    let mut g_state = GameState::new();

    println!("Player One Points: {}", player_one.get_points());
    Player::play_king(100, &mut player_one);
    println!("Player One Points: {}", player_one.get_points());

    g_state.add_player(player_one);
    g_state.add_player(player_two);
}
