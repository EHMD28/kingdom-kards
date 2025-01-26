pub mod game;
pub mod server;
pub mod utils;

#[cfg(test)]
mod tests {

    use core::panic;
    use std::str::FromStr;

    use crate::game::game_state::GameState;
    use crate::game::game_state::PlayerDetails;
    use crate::game::player::*;
    use crate::server::request::*;
    use crate::server::response::*;

    #[test]
    fn player_initialization() {
        let mut player = Player::with_name(String::from("John Smith"));
        assert_eq!(player.name(), "John Smith");
        assert_eq!(player.points(), 100);
        assert_eq!(player.get_deck_size(), 52 - 5);
        assert_eq!(player.get_hand_size(), 5);

        player.draw_card();
        assert_eq!(player.get_deck_size(), 52 - 6);
        assert_eq!(player.get_hand_size(), 6);
    }

    #[test]
    fn card_effects_working() {
        let mut player_one = Player::with_name(String::from("John Smith"));
        assert_eq!(player_one.points(), 100);
        Player::play_king(10, &mut player_one);
        assert_eq!(player_one.points(), 80);
        Player::play_queen(0, &mut player_one);
        assert_eq!(player_one.points(), 90);
    }

    #[test]
    fn str_to_request() {
        let test_one = Request::from_str("REQ,NAME");
        match test_one {
            Ok(request) => assert_eq!(request, Request::new(RequestType::Name)),
            Err(e) => unreachable!("{:?}", e),
        }

        let test_two = Request::from_str("REQ,ACT");
        match test_two {
            Ok(request) => assert_eq!(request, Request::new(RequestType::PlayerAction)),
            Err(e) => unreachable!("{:?}", e),
        }

        let test_three = Request::from_str("REQ,STATUS");
        match test_three {
            Ok(request) => assert_eq!(request, Request::new(RequestType::Status)),
            Err(e) => unreachable!("{e:?}"),
        };

        let test_four = Request::from_str("REQ,GAME");
        match test_four {
            Ok(request) => assert_eq!(request, Request::new(RequestType::GameState)),
            Err(e) => unreachable!("{e:?}"),
        }
    }

    #[test]
    fn request_to_str() {
        let test_one = Request::new(RequestType::Name).to_string();
        assert_eq!(test_one, "REQ,NAME");
        let test_two = Request::new(RequestType::PlayerAction).to_string();
        assert_eq!(test_two, "REQ,ACT");
        let test_three = Request::new(RequestType::Status).to_string();
        assert_eq!(test_three, "REQ,STATUS");
        let test_four = Request::new(RequestType::Status).to_string();
        assert_eq!(test_four, "REQ,STATUS");
    }

    #[test]
    fn str_to_response() {
        let test_one = Response::from_str("RES,NAME,John Smith");
        match test_one {
            Ok(response) => assert_eq!(
                response,
                Response::new(ResponseType::Name(Some("John Smith".to_string())))
            ),
            Err(e) => panic!("{:?}", e),
        }

        let test_two = Response::from_str("RES,ACT,K,10,John Smith,Jane Doe");
        let test_two_action = Action::new(
            ActionType::PlayKing,
            10,
            "John Smith".to_string(),
            "Jane Doe".to_string(),
        );
        match test_two {
            Ok(response) => assert_eq!(
                response,
                Response::new(ResponseType::PlayerAction(Some(test_two_action)))
            ),
            Err(e) => panic!("{:?}", e),
        }

        let test_three = Response::from_str("RES,STATUS,Y");
        match test_three {
            Ok(response) => assert_eq!(
                response,
                Response::new(ResponseType::Status(Some(StatusType::Yes)))
            ),
            Err(e) => panic!("{e:?}"),
        }

        let test_four = Response::from_str("RES,GAME,2,ABC:90,DEF:20");
        let mut game_state = GameState::new();
        game_state.add_player(PlayerDetails::new("ABC".to_string(), 90));
        game_state.add_player(PlayerDetails::new("DEF".to_string(), 20));
        let response_four = Response::new(ResponseType::GameState(Some(game_state)));
        match test_four {
            Ok(response) => assert_eq!(response, response_four),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn response_to_str() {
        let test_one =
            Response::new(ResponseType::Name(Some("John Smith".to_string()))).to_string();
        assert_eq!(test_one, "RES,NAME,John Smith");

        let test_two_action = Action::new(
            ActionType::PlayKing,
            10,
            "John Smith".to_string(),
            "Jane Doe".to_string(),
        );
        let test_two = Response::new(ResponseType::PlayerAction(Some(test_two_action))).to_string();
        assert_eq!(test_two, "RES,ACT,K,10,John Smith,Jane Doe");

        let test_three = Response::new(ResponseType::Status(Some(StatusType::Yes))).to_string();
        assert_eq!(test_three, "RES,STATUS,Y");

        let mut game_state = GameState::new();
        game_state.add_player(PlayerDetails::new("ABC".to_string(), 90));
        game_state.add_player(PlayerDetails::new("DEF".to_string(), 20));
        let test_four = Response::new(ResponseType::GameState(Some(game_state))).to_string();
        assert_eq!(test_four, "RES,GAME,2,ABC:90,DEF:20");
    }
}
