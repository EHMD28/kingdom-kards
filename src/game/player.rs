use crate::game::card::{Card, Suit, Value};

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Player {
    hand: Vec<Card>,
    deck: Vec<Card>,
    points: u16
}

impl Player {
    pub fn new() -> Self {
        let mut player = Player {
            hand: Vec::new(),
            deck: Vec::new(),
            points: 100
        };

        Player::init_deck(&mut player);        

        Player::shuffle_deck(&mut player);
        
        player.draw_ntimes(5);
        
        player
    }

    pub fn get_points(&self) -> u16 {
        self.points
    }

    pub fn get_hand_size(&self) -> u8 {
        self.hand.len() as u8
    }

    fn init_deck(player: &mut Player) {
        for suit in [Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds] {
            for value in [
                Value::Ace,
                Value::Two,
                Value::Three,
                Value::Four,
                Value::Five,
                Value::Six,
                Value::Seven,
                Value::Eight,
                Value::Nine,
                Value::Ten,
                Value::Jack,
                Value::Queen,
                Value::King,
            ] {
                player.deck.push(Card::new(suit, value));
            }
        }
    }

    fn shuffle_deck(player: &mut Player) {
        player.deck.shuffle(&mut thread_rng());
    }

    pub fn get_deck_size(&self) -> u16 {
        self.deck.len() as u16
    }

    pub fn draw_card(&mut self) {
        let card = self.deck.pop();
        match card {
            None => (),
            Some(value) => self.hand.push(value),
        }
    }
    
    pub fn draw_ntimes(&mut self, n: u8) {
        for _ in 0..n {
            self.draw_card();
        }
    }

    pub fn _print_deck(&self) {
        for card in &self.deck {
            card.print_self();            
        }
    }

    pub fn _print_hand(&self) {
        for card in &self.hand {
            card.print_self();
        }
    }

}
