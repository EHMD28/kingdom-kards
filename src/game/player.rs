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

        player.shuffle_deck();
        
        for _ in 0..5 {
            player.draw_card();
        }

        player
    }

    // pub fn get_points(&self) -> u16 {
    //     self.points
    // }

    fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut thread_rng());
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

    pub fn _get_points(&self) -> u16 {
        self.points
    }

}
