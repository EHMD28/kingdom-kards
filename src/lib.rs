pub mod game;
pub mod server;
pub mod utils;

#[cfg(test)]
mod tests {

    use core::panic;
    use std::str::FromStr;

    use request_response::{Action, ActionType, Request, RequestType, Response, ResponseType};

    use crate::game::player::*;
    use crate::server::*;

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
    fn str_to_request() {
        let test_one = Request::from_str("REQ,NAME");
        match test_one {
            Ok(request) => assert_eq!(request, Request::new(RequestType::Name)),
            Err(e) => panic!("{:?}", e),
        }

        let test_two = Request::from_str("REQ,ACT");
        match test_two {
            Ok(request) => assert_eq!(request, Request::new(RequestType::PlayerAction)),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn request_to_str() {
        let test_one = Request::new(RequestType::Name).to_string();
        assert_eq!(test_one, "REQ,NAME");
    }

    #[test]
    fn str_to_response() {
        let test_one = Response::from_str("RES,NAME,John Smith");
        match test_one {
            Ok(response) => assert_eq!(
                response,
                Response::new(ResponseType::Name("John Smith".to_string()))
            ),
            Err(e) => panic!("{:?}", e),
        }

        let test_two = Response::from_str("RES,ACT,K,10,John Smith,Jane Doe");
        match test_two {
            Ok(response) => assert_eq!(
                response,
                Response::new(ResponseType::PlayerAction(Action::new(
                    ActionType::PlayKing,
                    10,
                    String::from("John Smith"),
                    String::from("Jane Doe")
                )))
            ),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn response_to_str() {
        let test_one = Response::new(ResponseType::Name("John Smith".to_string())).to_string();
        assert_eq!(test_one, "RES,NAME,John Smith");

        let test_two = Response::new(ResponseType::PlayerAction(Action::new(
            ActionType::PlayKing,
            10,
            String::from("John Smith"),
            String::from("Jane Doe"),
        )))
        .to_string();
        assert_eq!(test_two, "RES,ACT,K,10,John Smith,Jane Doe");
    }
}
