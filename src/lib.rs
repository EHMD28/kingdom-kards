pub mod game;
pub mod server;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::game::player::*;

    #[test]
    fn player_initialization() {
        let mut player = Player::new(String::from("John Smith"));
        assert_eq!(player.get_name(), "John Smith");
        assert_eq!(player.get_points(), 100);
        assert_eq!(player.get_deck_size(), 52 - 5);
        assert_eq!(player.get_hand_size(), 5);

        player.draw_card();
        assert_eq!(player.get_deck_size(), 52 - 6);
        assert_eq!(player.get_hand_size(), 6);
    }

    #[test]
    fn card_effects_working() {
        let mut player_one = Player::new(String::from("John Smith"));
        assert_eq!(player_one.get_points(), 100);
        Player::play_king(10, &mut player_one);
        assert_eq!(player_one.get_points(), 80);
        Player::play_queen(0, &mut player_one);
        assert_eq!(player_one.get_points(), 90);
    }
}
