//! This module contains a struct that keeps track of the current game state.
//! The GameState struct should only be used server side, and there should only
//! be one instance of a GameState struct per server.

use super::player::Player;

pub struct PlayerDetails {
    name: String,
    points: u16,
}

impl PlayerDetails {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_points(&self) -> u16 {
        self.points
    }
}

impl From<Player> for PlayerDetails {
    fn from(value: Player) -> Self {
        Self {
            name: String::from(value.get_name()),
            points: value.get_points(),
        }
    }
}

pub struct GameState {
    players: Vec<PlayerDetails>,
    current_player: usize,
}

impl GameState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            current_player: 0,
        }
    }

    pub fn add_player(&mut self, p: PlayerDetails) {
        self.players.push(p);
    }

    pub fn get_all_players(&self) -> &Vec<PlayerDetails> {
        &self.players
    }

    pub fn get_player(&self, pos: usize) -> &PlayerDetails {
        let num_players = 0..self.players.len();
        if !num_players.contains(&pos) {
            panic!("Trying to access invalid player index");
        }

        self.players.get(pos).unwrap()
    }

    pub fn move_next_player(&mut self) {
        let num_players = self.players.len();
        self.current_player = (self.current_player + 1) % (num_players);
    }

    // pub fn get_current_player(&self) -> & {
    // }

    // pub fn remove_lost_players(&mut self) {
    //     self.players.retain(|p| p.get_points() != 0);
    // }

    // pub fn _print_players(&self) {
    //     for player in self.players.iter() {
    //         player._print_self();
    //         println!();
    //     }
    // }
}
