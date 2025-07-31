use crate::{game::game_state::GameState, server::response::Action};

pub struct Commentator;

impl Commentator {
    pub fn turn_start(name: &str) {
        println!("{name} is starting their turn.");
    }

    pub fn turn_end(name: &str) {
        println!("{name} has ended their turn.")
    }

    pub fn play_king(action: &Action, game_state: &GameState) {
        let to_player = game_state.player_by_name(action.to_player());
        let to_player = to_player.unwrap();
        if action.attachment() != 0 {
            println!(
                "{} played a King with {} against {}.",
                action.from_player(),
                action.attachment(),
                action.to_player()
            );
        } else {
            println!(
                "{} played a King against {}.",
                action.from_player(),
                action.to_player(),
            );
        }
        println!(
            "{} now has {} points.",
            to_player.name(),
            to_player.points()
        );
    }

    pub fn play_queen(action: &Action, game_state: &GameState) {
        let to_player = game_state.player_by_name(action.to_player());
        let to_player = to_player.unwrap();
        if action.attachment() != 0 {
            println!(
                "{} played a Queen with {} against {}.",
                action.from_player(),
                action.attachment(),
                action.to_player()
            );
        } else {
            println!(
                "{} played a Queen against {}.",
                action.from_player(),
                action.to_player(),
            );
        }
        println!(
            "{} now has {} points.",
            to_player.name(),
            to_player.points()
        );
    }

    pub fn play_number(action: &Action) {
        println!(
            "{} played a Number {} and drew {} cards.",
            action.from_player(),
            action.attachment(),
            action.attachment()
        );
    }
}
