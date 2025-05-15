//! This module contains a struct that keeps track of the current game state.
//! The GameState struct should only be used server side, and there should only
//! be one instance of a GameState struct per server.

use std::fmt::Display;

use crate::{server::constants::MAX_PLAYERS, ui::get_num_input};

use super::player::Player;

/// This is a struct for representing players server side, since it isn't necessary for the
/// server to know which cards each player has, as long as everything is being validated server
/// side.
#[derive(Debug, PartialEq, Clone)]
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

    pub fn set_points(&mut self, points: u16) {
        self.points = points;
    }
}

impl Display for PlayerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} with {} points.", self.name(), self.points())
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

// impl ToOwned for PlayerDetails {
//     type Owned = PlayerDetails;

//     fn to_owned(&self) -> Self::Owned {
//         PlayerDetails::new(self.name.to_string(), self.points)
//     }
// }

#[derive(Debug, PartialEq, Clone)]
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

    pub fn get_player_with_prompt(&self) -> &PlayerDetails {
        self.print_all_players();
        let player_pos = get_num_input("Choose a player: ", 1, self.num_players() as i32);
        self.get_player((player_pos - 1) as usize)
    }

    pub fn player_by_name(&self, name: &str) -> Option<&PlayerDetails> {
        self.players.iter().find(|&player| player.name == name)
    }

    pub fn player_by_name_mut(&mut self, name: &str) -> Option<&mut PlayerDetails> {
        self.players.iter_mut().find(|player| player.name == name)
    }

    pub fn current_player(&self) -> &PlayerDetails {
        self.players.get(self.current_player).unwrap()
    }

    pub fn current_player_index(&self) -> usize {
        self.current_player
    }

    pub fn move_next_player(&mut self) {
        let num_players = self.players.len();
        self.current_player = (self.current_player + 1) % (num_players);
    }

    pub fn print_all_players(&self) {
        println!("Current Players:");
        for (index, player) in self.players.iter().enumerate() {
            println!("\t{}. {player}", index + 1);
        }
    }

    pub fn list_players_with_numbers(&self) {
        for (index, player) in self.players.iter().enumerate() {
            println!("{}. {} with {}", index + 1, player.name, player.points);
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
