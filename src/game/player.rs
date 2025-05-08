//! This module contains the code for handling player state client side.

use std::panic;

use crate::game::card::{Card, Suit, Value};
use crate::server::constants::DECK_SIZE;
use crate::server::response::{Action, ActionType};
use crate::ui::{get_bool_input, get_num_input};

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::game_state::GameState;

pub struct Player {
    name: String,
    points: u16,
    hand: Vec<Card>,
    deck: Vec<Card>,
    _discard_pile: Vec<Card>,
}

impl Player {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Player {
        let mut player = Player {
            name: String::new(),
            hand: Vec::with_capacity(DECK_SIZE),
            deck: Vec::with_capacity(DECK_SIZE),
            _discard_pile: Vec::with_capacity(DECK_SIZE),
            points: 100,
        };

        player.init_deck();
        // player.shuffle_deck();
        player.draw_ntimes(5);

        player
    }

    pub fn with_name(name: String) -> Player {
        let mut player = Player {
            name,
            hand: Vec::with_capacity(DECK_SIZE),
            deck: Vec::with_capacity(DECK_SIZE),
            _discard_pile: Vec::with_capacity(DECK_SIZE),
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

    pub fn get_card_in_hand(&self, n: usize) -> &Card {
        if n >= self.hand_size() {
            panic!("Invalid index of hand");
        }
        self.hand.get(n).unwrap()
    }

    fn init_deck(&mut self) {
        self.deck.push(Card::new(Suit::Hearts, Value::Seven));
        self.deck.push(Card::new(Suit::Spades, Value::Five));
        self.deck.push(Card::new(Suit::Hearts, Value::Eight));
        self.deck.push(Card::new(Suit::Spades, Value::Queen));
        self.deck.push(Card::new(Suit::Spades, Value::King));
        // for suit in [Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds] {
        //     for value in [
        //         Value::Ace,
        //         Value::Two,
        //         Value::Three,
        //         Value::Four,
        //         Value::Five,
        //         Value::Six,
        //         Value::Seven,
        //         Value::Eight,
        //         Value::Nine,
        //         Value::Ten,
        //         Value::Jack,
        //         Value::Queen,
        //         Value::King,
        //     ] {
        //         self.deck.push(Card::new(suit, value));
        //     }
        // }
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

    pub fn get_action(&self, game_state: &GameState) -> Action {
        println!("0. End Turn");
        self.print_hand();
        let choosen_action = get_num_input("Choose an action: ", 0, self.hand_size() as i32);
        if choosen_action == 0 {
            return Action::new(ActionType::TurnEnd, 0, self.name.clone(), String::new());
        }
        let choosen_card = self.get_card_in_hand((choosen_action - 1) as usize);
        let action_type = ActionType::from_card(choosen_card);
        let attachment: u16 = match action_type {
            ActionType::PlayKing | ActionType::PlayQueen => {
                self.get_attachment().unwrap_or_default()
            }
            ActionType::PlayJack => todo!(),
            ActionType::PlayNumber => todo!(),
            ActionType::PlayBlackAce => todo!(),
            ActionType::PlayRedAce => todo!(),
            _ => unreachable!(),
        };

        game_state.list_players_with_numbers();
        let to_player = get_num_input("Choose a player: ", 1, game_state.num_players() as i32);
        let to_player = game_state.get_player((to_player - 1) as usize);

        Action::new(
            action_type,
            attachment,
            self.name().to_owned(),
            to_player.name().to_owned(),
        )
    }

    fn get_attachment(&self) -> Option<u16> {
        let use_attachment = get_bool_input("Attachment ['yes' or 'no']? ", "yes", "no");
        if use_attachment {
            loop {
                let choosen_card =
                    get_num_input("Choose a number: ", 1, (self.hand_size() + 1) as i32) as usize;
                let chosen_card = self.get_card_in_hand(choosen_card - 1);
                if chosen_card.value().is_number() {
                    let attachment = chosen_card.value().to_number_value();
                    return Some(attachment);
                }
            }
        } else {
            None
        }
    }

    pub fn _print_deck(&self) {
        for card in &self.deck {
            println!("{card}")
        }
    }

    pub fn print_hand(&self) {
        for (i, card) in self.hand.iter().enumerate() {
            println!("{}. {}", i + 1, card.to_colored_text());
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
