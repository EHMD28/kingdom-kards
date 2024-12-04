pub mod game;
pub mod server;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::{
        game::{
            card::{Card, Suit, Value},
            player::*,
        },
        server::action::IsAction,
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
        let card = Card::new(Suit::Spades, Value::King);
        let action_str = card.to_action(0, "John Smith", "Jane Doe").to_string();
        assert_eq!(r#"ACT K 0 "John Smith" "Jane Doe""#, action_str);

        let card = Card::new(Suit::Spades, Value::King);
        let action_str = card.to_action(5, "John Smith", "Jane Doe").to_string();
        assert_eq!(r#"ACT K 5 "John Smith" "Jane Doe""#, action_str);

        let card = Card::new(Suit::Spades, Value::Queen);
        let action_str = card.to_action(0, "John Smith", "Jane Doe").to_string();
        assert_eq!(r#"ACT Q 0 "John Smith" "Jane Doe""#, action_str);

        let card = Card::new(Suit::Spades, Value::Jack);
        let action_str = card.to_action(0, "John Smith", "Jane Doe").to_string();
        assert_eq!(r#"ACT J 0 "John Smith" "Jane Doe""#, action_str);

        let card = Card::new(Suit::Spades, Value::Ten);
        let action_str = card.to_action(10, "John Smith", "Jane Doe").to_string();
        assert_eq!(r#"ACT N 10 "John Smith" "Jane Doe""#, action_str);

        let card = Card::new(Suit::Spades, Value::Two);
        let action_str = card.to_action(2, "John Smith", "Jane Doe").to_string();
        assert_eq!(r#"ACT N 2 "John Smith" "Jane Doe""#, action_str);

        let card = Card::new(Suit::Spades, Value::Ace);
        let action_str = card.to_action(0, "John Smith", "Jane Doe").to_string();
        assert_eq!(r#"ACT B 0 "John Smith" "Jane Doe""#, action_str);

        let card = Card::new(Suit::Hearts, Value::Ace);
        let action_str = card.to_action(0, "John Smith", "Jane Doe").to_string();
        assert_eq!(r#"ACT R 0 "John Smith" "Jane Doe""#, action_str);
    }
}
