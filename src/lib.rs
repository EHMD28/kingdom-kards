pub mod game;

#[cfg(test)]
mod tests {
    use crate::game::player::*;

    #[test]
    fn player_initialization() {
        let mut player = Player::new();
        assert_eq!(player.get_points(), 100);
        assert_eq!(player.get_deck_size(), 52 - 5);
        assert_eq!(player.get_hand_size(), 5);
        
        player.draw_card();
        assert_eq!(player.get_deck_size(), 52 - 6);
        assert_eq!(player.get_hand_size(), 6);
    }
}
