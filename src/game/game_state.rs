use super::player::Player;

pub struct GameState {
    players: Vec<Player>,
    current_player: u8,
}

impl GameState {
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
        self.current_player = (self.current_player + 1) % (num_players as u8);
    }

    pub fn _print_players(&self) {
        for player in self.players.iter() {
            player._print_self();
            print!("\n");
        }
    }
}
