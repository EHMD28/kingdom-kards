use std::panic;

use crate::game::card::{Card, Suit, Value};

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::card::Color;

pub struct Player {
    name: String,
    hand: Vec<Card>,
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
    points: u16,
}

impl Player {
    pub fn new(name: String) -> Self {
        let mut player = Player {
            name,
            hand: Vec::new(),
            deck: Vec::new(),
            discard_pile: Vec::new(),
            points: 100,
        };

        Player::init_deck(&mut player);

        Player::shuffle_deck(&mut player);

        player.draw_ntimes(5);

        player
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_points(&self) -> u16 {
        self.points
    }

    pub fn get_hand_size(&self) -> u8 {
        self.hand.len() as u8
    }

    pub fn get_card_from_hand(&self, n: u8) -> &Card {
        if n >= self.get_hand_size() {
            panic!("Invalid index of hand");
        }

        self.hand.get(n as usize).unwrap()
    }

    /* TODO: change to non-static method */
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

    /* TODO: change to non-static method */
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

    pub fn play_card(&mut self, pos: u8) {
        if pos >= self.hand.len() as u8 {
            panic!("Can't access index of hand");
        }

        let card = self.hand.get(pos as usize).unwrap();

        match card.get_value() {
            Value::King => todo!(),
            Value::Queen => todo!(),
            Value::Jack => todo!(),
            Value::Two
            | Value::Three
            | Value::Four
            | Value::Five
            | Value::Six
            | Value::Seven
            | Value::Eight
            | Value::Nine
            | Value::Ten => {
                todo!()
            }
            Value::Ace => match card.get_color() {
                Color::Red => todo!(),
                Color::Black => todo!(),
            },
        }
    }

    pub fn play_king(&self, attachment: u16, target: &mut Player) {
        target.points -= 10 + attachment;
    }

    pub fn play_queen(&self, attachment: u16, target: &mut Player) {
        target.points += 10 + attachment;
    }

    pub fn _print_deck(&self) {
        for card in &self.deck {
            card._print_self();
        }
    }

    pub fn _print_hand(&self) {
        for (i, card) in self.hand.iter().enumerate() {
            print!("{}. ", i + 1);
            card._print_self();
        }
    }

    pub fn _print_hand_unicode(&self) {
        for card in &self.hand {
            card._print_self_unicode();
        }
    }

    pub fn _print_self(&self) {
        println!("Player Name: {}", self.name);
        println!("Points: {}", self.points);
        self._print_hand();
    }
}
