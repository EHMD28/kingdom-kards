//! This module contains a struct that keeps track of the current game state.
//! The GameState struct should only be used server side, and there should only
//! be one instance of a GameState struct per server.

use std::fmt::Display;

use crate::server::constants::MAX_PLAYERS;

use super::player::Player;

/// This is a struct for representing players server side, since it isn't necessary for the
/// server to know which cards each player has, as long as everything is being validated server
/// side.
#[derive(Debug, PartialEq)]
pub struct PlayerDetails {
    name: String,
    points: u16,
}

impl PlayerDetails {
    pub fn new(name: String, points: u16) -> PlayerDetails {
        PlayerDetails { name, points }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn points(&self) -> u16 {
        self.points
    }
}

impl Display for PlayerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {} | Points: {}", self.name(), self.points())
    }
}

impl From<Player> for PlayerDetails {
    fn from(value: Player) -> Self {
        Self {
            name: String::from(value.name()),
            points: value.points(),
        }
    }
}

impl ToOwned for PlayerDetails {
    type Owned = PlayerDetails;

    fn to_owned(&self) -> Self::Owned {
        PlayerDetails::new(self.name.to_string(), self.points)
    }
}

#[derive(Debug, PartialEq)]
pub struct GameState {
    players: Vec<PlayerDetails>,
    current_player: usize,
}

impl GameState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            players: Vec::with_capacity(MAX_PLAYERS),
            current_player: 0,
        }
    }

    pub fn add_player(&mut self, p: PlayerDetails) {
        // println!("Added player: {p}");
        self.players.push(p);
    }

    pub fn all_players(&self) -> &Vec<PlayerDetails> {
        &self.players
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
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

    pub fn print_all_players(&self) {
        for (index, player) in self.players.iter().enumerate() {
            println!("Game State");
            println!("\t{}. {player}", index + 1);
        }
    }

    pub fn is_unique_name(&self, name: &str) -> bool {
        for player in self.players.iter() {
            if player.name() == name {
                return false;
            }
        }

        true
    }
}
