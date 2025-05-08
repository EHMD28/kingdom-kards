//! This module contains a set of functions for creating a server and
//! handling clients.

use std::{mem::zeroed, net::TcpListener, thread, time::Duration};

use rand::{seq::SliceRandom, thread_rng};

use crate::{
    game::game_state::{GameState, PlayerDetails},
    server::constants::{STATUS_REQUEST, STATUS_RESPONSE_YES},
    utils::{clear_screen, perror_in_fn, variant_eq},
};

use super::{
    client,
    constants::{
        ACTION_REQUEST, ACTION_RESPONSE, GAME_STATE_REQUEST, MAX_PLAYERS, NAME_REQUEST,
        NAME_RESPONSE, STATUS_RESPONSE_NO,
    },
    response::{Action, ActionType, Response, ResponseType},
    StreamHandler,
};

/// Type used for representing clients (`StreamHandler` and `PlayerDetails`) server-side.
struct Client {
    handler: StreamHandler,
    player: Option<PlayerDetails>,
}

impl Client {
    fn new(handler: StreamHandler, player: Option<PlayerDetails>) -> Client {
        Client { handler, player }
    }

    fn handler(&self) -> &StreamHandler {
        &self.handler
    }

    fn handler_mut(&mut self) -> &mut StreamHandler {
        &mut self.handler
    }

    fn set_player(&mut self, player: PlayerDetails) {
        self.player = Some(player);
    }

    fn player(&self) -> &PlayerDetails {
        self.player.as_ref().unwrap()
    }
}

pub struct ServerInstance {
    game_state: GameState,
    listener: TcpListener,
    clients: Vec<Client>,
    current_client: usize,
    // clients: Vec<StreamHandler>,
    // players: Vec<PlayerDetails>,
    join_code: String,
}

impl ServerInstance {
    /// Starts instance of Kingdom Kards server.
    /// Hosted locally on port 5464 because 'king' - phone keypad -> '5464'
    pub fn create() -> ServerInstance {
        let port = "127.0.0.1:5464".to_string();
        let listener = TcpListener::bind(port).expect("Failed to bind to port 127.0.0.1:5464");
        // The maximum amount of players allowed to join a game.

        ServerInstance {
            game_state: GameState::new(),
            listener,
            join_code: "1234".to_string(),
            clients: Vec::with_capacity(MAX_PLAYERS),
            current_client: 0,
        }
    }

    /// Returns a reference to the member `game_state` of this struct.
    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    fn turn_client_mut(&mut self) -> Option<&mut Client> {
        let turn_player = self.game_state.current_player();
        self.clients
            .iter_mut()
            .find(|c| c.player().name() == turn_player.name())
    }

    /// Starts up server operations. First, the server accepts the number
    /// of players (between 2 and 6). Next, players will enter their usernames
    /// and the server will validate that the names are unique. After that, the
    /// core gameplay loop starts.
    pub fn start(&mut self) {
        // let num_players = get_num_input("Enter number of players (min. 2, max. 6): ", 2, 6);
        // TODO: change back to `num_players` for full application
        self.accept_players(1);
        self.reject_extra_players();
        println!("Starting server with join code: {}", self.join_code);
        self.name_players();
        self.game_state.print_all_players();
        self.send_game_state();
        self.randomize_players();
        self.start_game_loop();
    }

    /// Allows `num_players` clients to join, exits after all players have joined.
    fn accept_players(&mut self, num_players: u8) {
        let mut num_connections = 0;
        println!("Accepting players...");
        while num_connections < num_players {
            match self.listener.accept() {
                Ok((stream, _)) => {
                    let mut handler = StreamHandler::new(stream);
                    if let Err(err) = handler.await_request(STATUS_REQUEST) {
                        perror_in_fn("accept_players", err);
                    }
                    /* Accept player. */
                    if let Err(err) = handler.send_response(STATUS_RESPONSE_YES) {
                        perror_in_fn("accept_players", err);
                    }
                    self.clients.push(Client::new(handler, None));
                    num_connections += 1;
                }
                Err(err) => {
                    perror_in_fn("accept_players", err);
                }
            }
        }
    }

    /// Starts a new thread that rejects all players that join
    /// after the room is full.
    fn reject_extra_players(&mut self) {
        let listener = self.listener.try_clone().unwrap();
        thread::spawn(move || {
            for connection in listener.incoming() {
                let handler = &mut StreamHandler::new(connection.unwrap());
                if let Err(err) =
                    /* Client asking if room is full. */
                    handler.await_request_send_response(STATUS_REQUEST, STATUS_RESPONSE_NO)
                {
                    perror_in_fn("reject_extra_players", err);
                }
            }
        });
    }

    /// Prompts every user to enter a username and verifies that each username is unique.
    fn name_players(&mut self) {
        for client in self.clients.iter_mut() {
            let handler = client.handler_mut();
            let mut is_accepted = false;
            let mut name = String::new();
            while !is_accepted {
                name = ServerInstance::get_client_name(handler);
                is_accepted =
                    ServerInstance::send_name_status(handler, &mut self.game_state, name.as_str());
            }
            client.set_player(PlayerDetails::new(name, 100));
        }
    }

    fn get_client_name(handler: &mut StreamHandler) -> String {
        let status = handler.send_request_await_response(NAME_REQUEST, NAME_RESPONSE);
        match status {
            Ok(response) => {
                if let ResponseType::Name(name) = response.response_type() {
                    name.as_ref().unwrap().to_owned()
                } else {
                    unreachable!("Received response of incorrect type");
                }
            }
            Err(err) => {
                perror_in_fn("get_client_name", err);
                "".to_string()
            }
        }
    }

    fn send_name_status(
        handler: &mut StreamHandler,
        game_state: &mut GameState,
        name: &str,
    ) -> bool {
        if let Err(err) = handler.await_request(STATUS_REQUEST) {
            perror_in_fn("send_status", err);
        }
        if game_state.is_unique_name(name) {
            /* Add a new player with 100 points. */
            let new_player = PlayerDetails::new(name.to_owned(), 100);
            game_state.add_player(new_player);
            if let Err(err) = handler.send_response(STATUS_RESPONSE_YES) {
                perror_in_fn("ServerInstance::send_status", err);
            }

            return true;
        } else if let Err(err) = handler.send_response(STATUS_RESPONSE_NO) {
            perror_in_fn("ServerInstance::send_status", err);
        }

        false
    }

    fn send_game_state(&mut self) {
        for Client { handler, .. } in self.clients.iter_mut() {
            let game_state_response = Response::from_game_state(self.game_state.clone());
            if let Err(err) =
                handler.await_request_send_response(GAME_STATE_REQUEST, &game_state_response)
            {
                perror_in_fn("send_game_state", err);
            }
        }
    }

    fn randomize_players(&mut self) {
        self.clients.shuffle(&mut thread_rng());
    }

    fn get_client_by_name(&self, name: &str) -> &Client {
        for client in self.clients.iter() {
            if client.player().name() == name {
                return client;
            }
        }
        unreachable!()
    }

    fn client_by_name_mut(&mut self, name: &str) -> &mut Client {
        for client in self.clients.iter_mut() {
            if client.player().name() == name {
                return client;
            }
        }
        unreachable!()
    }

    /// Starts core gameplay loop.
    fn start_game_loop(&mut self) {
        self.start_current_turn();
        self.start_action_loop();
        self.move_next_player();
        todo!()
    }

    fn start_current_turn(&mut self) {
        let turn_player = self.game_state.current_player().to_owned();
        let client = self.client_by_name_mut(turn_player.name());
        let response = Response::new_turn_start(turn_player.name().to_owned());
        if let Err(err) = client
            .handler
            .await_request_send_response(ACTION_REQUEST, &response)
        {
            perror_in_fn("start_current_turn", err);
        }
    }

    // fn response_all_except(&mut self, name: &str, response: &Response) {
    //     for Client { handler, player } in self.clients.iter_mut() {
    //         if player.as_ref().unwrap().name() != name {
    //             if let Err(err) = handler.await_request_send_response(ACTION_REQUEST, response) {
    //                 perror_in_fn("response_all", err);
    //             }
    //         }
    //     }
    // }

    fn await_player_action(&mut self) -> Action {
        let client = self.current_client_mut();
        let status = client
            .handler
            .send_request_await_response(ACTION_REQUEST, ACTION_RESPONSE);
        let action = match status {
            Ok(response) => match response.response_type() {
                ResponseType::PlayerAction(action) => action.as_ref().unwrap().to_owned(),
                _ => unreachable!(),
            },
            Err(err) => {
                perror_in_fn("host.rs::await_player_action", err);
                Action::default()
            }
        };
        action
    }

    fn start_action_loop(&mut self) {
        loop {
            let action = self.await_player_action();
            if variant_eq(action.action_type(), &ActionType::TurnEnd) {
                break;
            } else {
                self.handle_action(&action);
            }
        }
    }

    fn handle_action(&mut self, action: &Action) {
        match action.action_type() {
            ActionType::PlayKing => self.handle_king(action),
            ActionType::PlayQueen => self.handle_queen(action),
            ActionType::PlayJack => self.handle_jack(),
            ActionType::PlayNumber => self.handle_number(),
            ActionType::PlayBlackAce => self.handle_black_ace(),
            ActionType::PlayRedAce => self.handle_red_ace(),
            ActionType::TurnEnd | ActionType::TurnStart => unreachable!(),
        }
    }

    fn handle_king(&mut self, action: &Action) {
        let to_player = action.to_player();
        let damage = 10 + action.attachment();
        let to_player = self.game_state.player_by_name_mut(to_player).unwrap();
        to_player.set_points(to_player.points() - damage);
        println!(
            "ACTION: '{}' played King with {} against {}. '{}' now has {} points.",
            action.from_player(),
            action.attachment(),
            action.to_player(),
            action.to_player(),
            to_player.points(),
        );
        // self.response_all_except(
        //     action.from_player(),
        //     &Response::from_action(action.to_owned()),
        // );
    }

    fn handle_queen(&mut self, action: &Action) {
        let player = self
            .game_state
            .player_by_name_mut(action.from_player())
            .unwrap();
        let healed_points = 10 + action.attachment();
        player.set_points(player.points() + healed_points);
        println!(
            "ACTION: '{}' played a Queen with {}. '{}' now has {}",
            player.name(),
            action.attachment(),
            player.name(),
            player.points(),
        );
        // self.response_all_except(
        //     action.from_player(),
        //     &Response::from_action(action.to_owned()),
        // );
    }

    fn handle_jack(&mut self) {
        todo!()
    }

    fn handle_number(&mut self) {
        todo!()
    }

    fn handle_black_ace(&mut self) {
        todo!()
    }

    fn handle_red_ace(&mut self) {
        todo!()
    }

    fn move_next_player(&mut self) {
        self.current_client = (self.current_client + 1) % self.clients.len();
    }

    fn current_client_mut(&mut self) -> &mut Client {
        &mut self.clients[self.current_client]
    }

    /// This function is for testing purposes only. It blocks the main thread in an
    /// infinite loop to prevent the program from immediately exiting.
    pub fn _wait(&self) {
        println!("Server is waiting");

        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
