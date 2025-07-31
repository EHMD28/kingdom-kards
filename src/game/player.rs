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
        player.draw_n_times(6);

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
        player.draw_n_times(5);

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

    pub fn get_card_with_prompt(&self) -> &Card {
        todo!()
    }

    // pub fn remove_card_in_hand(&mut self, n: usize) -> Option<()> {
    //     todo!()
    // }

    pub fn remove_card_from_hand(&mut self, card: &Card) -> Option<()> {
        if let Some(index) = self.hand.iter().position(|c| c == card) {
            self.hand.remove(index);
            Some(())
        } else {
            None
        }
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
        self.deck.push(Card::new(Suit::Hearts, Value::Seven));
        self.deck.push(Card::new(Suit::Spades, Value::Five));
        self.deck.push(Card::new(Suit::Hearts, Value::Eight));
        self.deck.push(Card::new(Suit::Spades, Value::Two));
        self.deck.push(Card::new(Suit::Hearts, Value::Queen));
        self.deck.push(Card::new(Suit::Spades, Value::King));
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

    pub fn draw_n_times(&mut self, n: u8) {
        for _ in 0..n {
            self.draw_card();
        }
    }

    /// Prompts the player to play a card in their hand or end their turn. Returns None if no
    /// playable card is selected.
    pub fn get_action(&mut self, game_state: &GameState) -> Option<Action> {
        // Prompts the player to choose a card from their hand.
        if let Some(action_card) = self.choose_card() {
            // If the card is a number, the player discards the amount of that number then draws the
            // same amount as that number.
            if action_card.value().is_number() {
                self.handle_number(&action_card)
            }
            // If the card is a King or Queen, then the player is prompted to add an attachment.
            else if matches!(action_card.value(), Value::King | Value::Queen) {
                Some(self.handle_king_queen(&action_card, game_state))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn handle_number(&mut self, action_card: &Card) -> Option<Action> {
        if self.play_number(action_card).is_some() {
            Some(Action::new(
                ActionType::PlayNumber,
                action_card.value().to_number_value(),
                self.name.to_owned(),
                String::new(),
            ))
        } else {
            None
        }
    }

    fn handle_king_queen(&mut self, action_card: &Card, game_state: &GameState) -> Action {
        let attachment = self.play_king_queen();
        let action_type = ActionType::from_card(action_card);
        let to_player = game_state.get_player_with_prompt();
        self.remove_card_from_hand(action_card);
        Action::new(
            action_type,
            attachment,
            self.name.to_owned(),
            to_player.name().to_owned(),
        )
    }

    fn play_number(&mut self, card: &Card) -> Option<()> {
        self.remove_card_from_hand(card);
        let num_value = card.value().to_number_value();
        if (num_value as usize) > self.hand().len() {
            println!("Invalid action! Number exceeds hand size.");
            None
        } else {
            let mut num_discarded = 0;
            while num_discarded < num_value {
                if let Some(chosen) = self.choose_card() {
                    self.remove_card_from_hand(&chosen);
                    num_discarded += 1;
                }
            }
            self.draw_n_times(num_value as u8);
            Some(())
        }
    }

    fn play_king_queen(&mut self) -> u16 {
        if let Some(attachment_card) = self.choose_attacment() {
            let value = attachment_card.value().to_number_value();
            self.remove_card_from_hand(&attachment_card);
            value
        }
        // Player did not choose an attachment.
        else {
            0
        }
    }

    fn print_options(&self) {
        println!("0. End Turn");
        self.print_hand();
    }

    fn choose_card(&self) -> Option<Card> {
        self.print_options();
        let choosen_action = get_num_input("Choose a card: ", 0, self.hand_size() as i32);
        if choosen_action == 0 {
            None
        } else {
            Some(
                self.get_card_in_hand((choosen_action - 1) as usize)
                    .to_owned(),
            )
        }
    }

    // fn choose_card_with_prompt(&self, prompt: &str) -> Card {
    //     let choosen_action = get_num_input(prompt, 0, self.hand_size() as i32);
    //     self.get_card_in_hand((choosen_action - 1) as usize)
    //         .to_owned()
    // }

    // fn get_player_action_attachment(&self, action_card: &Card) -> Option<(Card, u16)> {
    //     let action_type = ActionType::from_card(action_card);
    //     let attachment_card = self.choose_attacment();
    //     if matches!(action_type, ActionType::PlayKing | ActionType::PlayQueen)
    //         && attachment_card.is_some()
    //     {
    //         let card = attachment_card.unwrap();
    //         let attachment = card.value.to_number_value();
    //         Some((card, attachment))
    //     } else {
    //         None
    //     }
    // }

    fn choose_attacment(&self) -> Option<Card> {
        let use_attachment = get_bool_input("Attachment? ['yes' or 'no']: ", "yes", "no");
        if use_attachment {
            loop {
                self.print_hand();
                let choosen_card =
                    get_num_input("Choose a number: ", 1, (self.hand_size() + 1) as i32) as usize;
                let chosen_card = self.get_card_in_hand(choosen_card - 1);
                if chosen_card.value().is_number() {
                    return Some(chosen_card.to_owned());
                }
            }
        } else {
            None
        }
    }

    // fn choose_number(&self) -> Card {
    //     self.print_hand();
    //     loop {
    //         let card = self.choose_card_with_prompt("Choose card to discard: ");
    //         if card.value().is_number() {
    //             return card;
    //         }
    //     }
    // }

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

impl Default for Player {
    fn default() -> Self {
        Player::new()
    }
}
