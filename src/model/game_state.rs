use std::collections::HashMap;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::model::player::Player;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct GameState {
    /// A hashmap relating the name of the player to the player themselves.
    players: HashMap<String, Player>,
    /// A vector containing the names of the players in the proper order.
    player_order: Vec<String>,
    /// The name of the current player.
    current_player: usize,
}

impl GameState {
    /// Creates a new game state from `players`. The first player is whoever is the first player in
    /// `players`.
    fn from_players(players: Vec<Player>) -> GameState {
        let mut hashmap: HashMap<String, Player> = HashMap::new();
        for player in players {
            hashmap.insert(player.name().to_owned(), player);
        }
        let mut player_names: Vec<String> = hashmap.keys().map(|k| k.to_owned()).collect();
        player_names.shuffle(&mut rand::rng());
        GameState {
            players: hashmap,
            player_order: player_names,
            current_player: 0,
        }
    }

    // /// Creates a `Vec` containing each player. This involves cloning all of the values, so avoid
    // /// calling this method often.
    // pub fn players(&self) -> Vec<Player> {
    //     self.players.values().cloned().collect()
    // }

    /// Checks to see if the name is unique.
    pub fn is_unique_name(&self, name: &str) -> bool {
        !self.players.contains_key(name)
    }

    /// Adds a player to the game.
    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.name().to_owned(), player);
    }

    pub fn randomize_players(&mut self) {}
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::model::{
        game_state::GameState,
        player::{self, Player},
    };

    #[test]
    fn game_state_initalization() {
        let player_names = ["Alice", "Bob", "Charlie"];
        let players: Vec<Player> = player_names.iter().map(|name| Player::new(name)).collect();
        let game_state = GameState::from_players(players);
        for p_name in player_names {
            assert!(game_state.players.contains_key(p_name))
        }
        assert_eq!(game_state.player_order.first().unwrap(), "Alice");
    }
}
