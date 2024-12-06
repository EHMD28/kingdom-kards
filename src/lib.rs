pub mod game;
pub mod server;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        game::player::*,
        server::action::{Action, ActionType},
    };

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

    #[test]
    fn action_to_str_conversion() {
        assert_eq!(
            "ACT,K,0,John Smith,Jane Doe",
            Action::new(
                ActionType::PlayKing,
                0,
                String::from("John Smith"),
                String::from("Jane Doe")
            )
            .to_string()
        );

        assert_eq!(
            "ACT,K,5,John Smith,Jane Doe",
            Action::new(
                ActionType::PlayKing,
                5,
                String::from("John Smith"),
                String::from("Jane Doe")
            )
            .to_string()
        );

        assert_eq!(
            "ACT,Q,0,John Smith,Jane Doe",
            Action::new(
                ActionType::PlayQueen,
                0,
                String::from("John Smith"),
                String::from("Jane Doe")
            )
            .to_string()
        );

        assert_eq!(
            "ACT,J,0,John Smith,Jane Doe",
            Action::new(
                ActionType::PlayJack,
                0,
                String::from("John Smith"),
                String::from("Jane Doe")
            )
            .to_string()
        );

        assert_eq!(
            "ACT,N,10,John Smith,Jane Doe",
            Action::new(
                ActionType::PlayNumber,
                10,
                String::from("John Smith"),
                String::from("Jane Doe")
            )
            .to_string()
        );

        assert_eq!(
            "ACT,N,2,John Smith,Jane Doe",
            Action::new(
                ActionType::PlayNumber,
                2,
                String::from("John Smith"),
                String::from("Jane Doe")
            )
            .to_string()
        );

        assert_eq!(
            "ACT,B,0,John Smith,Jane Doe",
            Action::new(
                ActionType::PlayBlackAce,
                0,
                String::from("John Smith"),
                String::from("Jane Doe")
            )
            .to_string()
        );

        assert_eq!(
            "ACT,R,0,John Smith,Jane Doe",
            Action::new(
                ActionType::PlayRedAce,
                0,
                String::from("John Smith"),
                String::from("Jane Doe")
            )
            .to_string()
        );
    }

    #[test]
    fn str_to_action_conversion() {
        let default_action = Action::new(ActionType::None, 0, String::new(), String::new());
        let action_str = "ACT,K,0,John Smith,Jane Doe";
        let action = Action::from_str(action_str).unwrap_or(default_action);
        assert_eq!(
            action,
            Action::new(
                ActionType::PlayKing,
                0,
                String::from("John Smith"),
                String::from("Jane Doe")
            )
        );
    }
}
