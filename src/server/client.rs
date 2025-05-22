//! This module contains a set of functions for client side commmunication
//! with the server.

use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use crate::game::game_state::GameState;
use crate::game::player::Player;
use crate::server::response::{Response, ResponseType, StatusType};
use crate::ui::get_input;
use crate::utils::{perror_in_fn, variant_eq};

use super::constants::{
    ACTION_REQUEST, ACTION_RESPONSE, GAME_STATE_REQUEST, GAME_STATE_RESPONSE, MAX_USERNAME_LEN,
    NAME_REQUEST, STATUS_REQUEST, STATUS_RESPONSE,
};
use super::response::{Action, ActionType};
use super::StreamHandler;

/// This is the type used for representing client-side errors.
pub enum ClientError {
    InvalidCharacterFound,
    CommaFound,
}

/// This is the type used for representing a single client instance.
/// There should only be one `ClientInstance` per running process of
/// `kingdom-kards`.
pub struct ClientInstance {
    /// `StreamHandler` for `TcpStream` connect to server.
    handler: Option<StreamHandler>,
    /// Name of current player.
    player: Player,
}

impl ClientInstance {
    /// Creates a new client instance with parameters uninitialized.
    #[allow(clippy::new_without_default)]
    pub fn new() -> ClientInstance {
        ClientInstance {
            handler: None,
            player: Player::new(),
        }
    }

    /// This function connects client instance to server at `port`. If the client is unable to
    /// connect to the server initially, then the program will prompt the user to try again. If
    /// they answer no, then the program will exit. If they answer yes, then the client will
    /// attempt to connect to the server again.
    ///
    /// ## Returns
    ///
    /// This function will return `None` if the server fails to connect. Otherwise, it will return
    /// `Some(())`
    pub fn connect_to_server(&mut self, port: &str) -> Option<()> {
        loop {
            if let Ok(stream) = TcpStream::connect(port) {
                let mut handler = StreamHandler::new(stream);
                if ClientInstance::is_room_open(&mut handler) {
                    self.handler = Some(handler);
                    println!("Joined Room.");
                    break Some(());
                } else {
                    println!("Room is full.");
                    break None;
                }
            } else if !ClientInstance::try_again() {
                return None;
            } else {
                thread::sleep(Duration::from_millis(500));
            }
        }
    }

    fn is_room_open(handler: &mut StreamHandler) -> bool {
        let status = handler.send_request_await_response(STATUS_REQUEST, STATUS_RESPONSE);
        match status {
            Ok(response) => match response.response_type() {
                ResponseType::Status(Some(StatusType::Yes)) => true,
                ResponseType::Status(Some(StatusType::No)) => false,
                _ => unreachable!(),
            },
            Err(err) => {
                perror_in_fn("is_room_open", err);
                false
            }
        }
    }

    fn try_again() -> bool {
        println!("Failed to connect to server.");
        io::stdout().flush().expect("Unable to flush stdout");

        const MAX_INPUT_LEN: usize = 5;
        let input = get_input("Try again [y/n]: ", MAX_INPUT_LEN).to_lowercase();

        if input == "n" || input == "no" {
            println!("Okay. Closing application");
            false
        } else {
            println!("Trying again...");
            true
        }
    }

    /// Starts the the gameplay loop client side. First, client must choose a username,
    /// then, they will start the actual game. `connect_to_server()` must be called before
    /// this function is called.
    ///
    /// # Panics
    ///
    /// This function will panic if there is no connection to the server
    /// (i.e. `connect_to_server()`) was not called or it failed.
    pub fn start(&mut self) {
        self.choose_player_name();
        let game_state = self.get_game_state();
        game_state.print_all_players();
        self.start_game_loop(&game_state);
    }

    /// This function is used to initiate communcation with the server so the
    /// player can choose a unique username.
    ///
    /// # Panics
    ///
    /// This function will panic if an invalid type is received from the server,
    /// but that should be impossible because `await_request()` and `await_response()`
    /// both check what type is received.
    pub fn choose_player_name(&mut self) {
        let mut is_accepted = false;
        let handler = self.handler.as_mut().unwrap();
        let mut name = String::new();

        while !is_accepted {
            name = ClientInstance::send_name(handler);
            is_accepted = ClientInstance::get_name_status(handler);
        }
        self.player.set_name(name);
        println!("Joined room as {}", self.player.name());
    }

    /// Sends name request an awaits response, printing any errors that may
    /// occur.
    fn send_name(handler: &mut StreamHandler) -> String {
        let name = ClientInstance::get_name_input();
        let name_response = Response::from_name(name.clone());
        if let Err(err) = handler.await_request_send_response(NAME_REQUEST, &name_response) {
            perror_in_fn("choose_player_name", err);
        }
        name
    }

    /// Sends status requests to the server to check if the name entered is valid.
    /// If the name is invalid, then the function will return false. Otherwise, it
    /// will return true.
    ///
    /// # Panics
    ///
    /// This function panics if it receives any invalid types, which should
    /// be impossible when using `send` and `await` functions.
    fn get_name_status(handler: &mut StreamHandler) -> bool {
        if let Err(err) = handler.send_request(STATUS_REQUEST) {
            perror_in_fn("get_name_status", err);
        }

        let status_response = handler.await_response(STATUS_RESPONSE);
        match status_response {
            Ok(response) => match response.response_type() {
                ResponseType::Status(Some(StatusType::Yes)) => {
                    println!("Name was accepted by server.");
                    true
                }
                ResponseType::Status(Some(StatusType::No)) => {
                    println!("Name was rejected by server.");
                    false
                }
                _ => unreachable!(),
            },
            Err(err) => {
                perror_in_fn("get_name_status", err);
                false
            }
        }
    }

    /// Prompts the user to enter a username until they enter a valid username.
    fn get_name_input() -> String {
        loop {
            let name = get_input("Enter a username: ", MAX_USERNAME_LEN);
            if ClientInstance::validate_name_input(&name) {
                break name;
            }
        }
    }

    /// Checks to see if name is only alphabetical ASCII characters
    fn validate_name_input(name: &str) -> bool {
        if !name.is_ascii() || !name.chars().all(char::is_alphabetic) {
            println!(
                "Error! Username cannot contain special characters or numbers (e.g. 1, $, ç, ♥︎)."
            );
            false
        } else {
            true
        }
    }

    fn get_game_state(&mut self) -> GameState {
        let handler = self.handler.as_mut().unwrap();
        let status = handler.send_request_await_response(GAME_STATE_REQUEST, GAME_STATE_RESPONSE);
        match status {
            Ok(response) => {
                if let ResponseType::GameState(game_state) = response.response_type() {
                    game_state.as_ref().unwrap().to_owned()
                } else {
                    unreachable!()
                }
            }
            Err(err) => {
                perror_in_fn("get_game_state", err);
                GameState::new()
            }
        }
    }

    /// Starts core gameplay loop.
    fn start_game_loop(&mut self, game_state: &GameState) {
        let handler = self.handler.as_mut().unwrap();
        let turn_player = ClientInstance::get_turn_start(handler);
        println!("Staring {turn_player}'s Turn");
        if turn_player == self.player.name() {
            self.start_my_turn(game_state);
        } else {
            self.start_other_turn();
        }
    }

    /// Get action from server signifying the start of a player's turn.
    /// If the turn player is this current player, this function returns true,
    /// else, it returns false.
    fn get_turn_start(handler: &mut StreamHandler) -> String {
        let status = handler.send_request_await_response(ACTION_REQUEST, ACTION_RESPONSE);
        match status {
            Ok(response) => {
                if let ResponseType::PlayerAction(Some(action)) = response.response_type() {
                    if variant_eq(action.action_type(), &ActionType::TurnStart) {
                        /* Turn player is stored as "from_player" in Action. */
                        action.from_player().to_owned()
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
            Err(err) => {
                perror_in_fn("get_turn_start", err);
                "".to_string()
            }
        }
    }

    fn start_my_turn(&mut self, game_state: &GameState) {
        /*
           1. Preform an action
           2. Inform server of action
           3. Repeat 1-2 until turn end
        */

        loop {
            if let Some(action) = self.player.get_action(game_state) {
                self.send_action_to_server(&action);
                if variant_eq(action.action_type(), &ActionType::TurnEnd) {
                    break;
                }
            }
        }
        todo!()
    }

    fn send_action_to_server(&mut self, action: &Action) {
        let handler = self.handler.as_mut().unwrap();
        if let Err(err) = handler
            .await_request_send_response(ACTION_REQUEST, &Response::from_action(action.to_owned()))
        {
            perror_in_fn("send_action_to_server", err);
        }
    }

    fn start_other_turn(&mut self) {
        todo!()
    }

    /// This function is for testing purposes only. It blocks the main thread in an
    /// infinite loop to prevent the program from immediately exiting.
    pub fn _wait(&self) {
        println!("Client is waiting");

        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
