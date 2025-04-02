//! This module contains the code for handling player state client side.

use std::cmp::max;
use std::panic;

use crate::game::card::{Card, Suit, Value};
use crate::server::constants::DECK_SIZE;
use crate::server::response::{Action, ActionType};
use crate::server::utils::get_num_input;

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::card::Color;

pub struct Player {
    name: String,
    points: u16,
    hand: Vec<Card>,
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Player {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Player {
        let mut player = Player {
            name: String::new(),
            hand: Vec::with_capacity(DECK_SIZE),
            deck: Vec::with_capacity(DECK_SIZE),
            discard_pile: Vec::with_capacity(DECK_SIZE),
            points: 100,
        };

        player.init_deck();
        player.shuffle_deck();
        player.draw_ntimes(5);

        player
    }

    pub fn with_name(name: String) -> Player {
        let mut player = Player {
            name,
            hand: Vec::with_capacity(DECK_SIZE),
            deck: Vec::with_capacity(DECK_SIZE),
            discard_pile: Vec::with_capacity(DECK_SIZE),
            points: 100,
        };

        player.init_deck();
        player.shuffle_deck();
        player.draw_ntimes(5);

        player
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn points(&self) -> u16 {
        self.points
    }

    pub fn hand(&self) -> &Vec<Card> {
        &self.hand
    }

    pub fn hand_size(&self) -> usize {
        self.hand.len()
    }

    pub fn card_in_hand(&self, n: usize) -> &Card {
        if n >= self.hand_size() {
            panic!("Invalid index of hand");
        }

        self.hand.get(n as usize).unwrap()
    }

    fn init_deck(&mut self) {
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
                self.deck.push(Card::new(suit, value));
            }
        }
    }

    fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut thread_rng());
    }

    pub fn deck_size(&self) -> u16 {
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

    pub fn get_action(&self) -> Action {
        self.print_hand();
        let choosen_card = get_num_input("Choose a card", 0, self.hand_size() as i32);
        let choosen_card = self.card_in_hand(choosen_card as usize);
        let action_type = ActionType::from_card(choosen_card);

        todo!()
    }

    pub fn _print_deck(&self) {
        for card in &self.deck {
            println!("{card}")
        }
    }

    pub fn print_hand(&self) {
        for (i, card) in self.hand.iter().enumerate() {
            println!("{}. {card}", i + 1);
        }
    }

    pub fn _print_hand_unicode(&self) {
        for card in &self.hand {
            println!("{}", card.to_unicode());
        }
    }

    pub fn _print_self(&self) {
        println!("Player Name: {}", self.name);
        println!("Points: {}", self.points);
        self.print_hand();
    }
}
