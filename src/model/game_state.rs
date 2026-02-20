use serde::{Deserialize, Serialize};

use crate::model::player::Player;

#[derive(Default, Serialize, Deserialize)]
pub struct GameState {
    players: Vec<Player>,
    current_player: usize,
}

impl GameState {
    pub fn from_players(players: Vec<Player>) -> GameState {
        GameState {
            players,
            current_player: 0,
        }
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn is_unique_name(&self, name: &str) -> bool {
        !self.players.iter().any(|p| p.name() == name)
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::model::{game_state::GameState, player::Player};

    #[test]
    fn game_state_initalization() {
        let player_names = ["Alice", "Bob", "Charlie"];
        let players: Vec<Player> = player_names.iter().map(|name| Player::new(name)).collect();
        let game_state = GameState::from_players(players);
        let game_state_names: Vec<String> = game_state
            .players
            .iter()
            .map(|player| player.name().to_owned())
            .collect();
        let names = zip(player_names, game_state_names);
        for (p_name, gs_name) in names {
            assert_eq!(p_name, gs_name);
        }
        assert_eq!(game_state.current_player, 0);
    }
}
