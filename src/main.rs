use kingdom_kards::game::player::*;
use kingdom_kards::game::game_state::GameState;

fn main() {
    let player_one = Player::new(String::from("John Smith"));
    let player_two = Player::new(String::from("Jane Doe"));
    let mut g_state = GameState::new();

    g_state.add_player(player_one);
    g_state.add_player(player_two);

    println!("\nGame State\n");
    g_state._print_players();
}
