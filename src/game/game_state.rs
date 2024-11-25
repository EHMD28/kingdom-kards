use super::player::Player;

pub struct GameState {
    players: Vec<Player>,
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

    pub fn add_player(&mut self, p: Player) {
        self.players.push(p);
    }

    pub fn move_next_player(&mut self) {
        let num_players = self.players.len();
        self.current_player = (self.current_player + 1) % (num_players);
    }

    pub fn get_current_player(&self) -> &Player {
        self.players.get(self.current_player).unwrap()
    }

    pub fn _print_players(&self) {
        for player in self.players.iter() {
            player._print_self();
            println!();
        }
    }
}
