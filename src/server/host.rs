//! This module contains a set of functions for creating a server and
//! handling clients.

use std::{net::TcpListener, thread, time::Duration};

use rand::{seq::SliceRandom, thread_rng};

use crate::{
    game::game_state::{GameState, PlayerDetails},
    server::constants::{STATUS_REQUEST, STATUS_RESPONSE_YES},
    utils::perror_in_fn,
};

use super::{
    constants::{
        ACTION_REQUEST, GAME_STATE_REQUEST, MAX_PLAYERS, NAME_REQUEST, NAME_RESPONSE,
        STATUS_RESPONSE_NO,
    },
    response::{Response, ResponseType},
    StreamHandler,
};

pub struct ServerInstance {
    game_state: GameState,
    listener: TcpListener,
    clients: Vec<StreamHandler>,
    players: Vec<PlayerDetails>,
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
            clients: Vec::with_capacity(MAX_PLAYERS),
            join_code: "1234".to_string(),
            players: Vec::with_capacity(MAX_PLAYERS),
        }
    }

    /// Returns a reference to the member `game_state` of this struct.
    pub fn game_state(&self) -> &GameState {
        &self.game_state
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

                    self.clients.push(handler);
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
        for handler in self.clients.iter_mut() {
            let mut is_accepted = false;

            while !is_accepted {
                let name = ServerInstance::get_client_name(handler);
                is_accepted =
                    ServerInstance::send_name_status(handler, &mut self.game_state, name.as_str());
            }
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
        for client in self.clients.iter_mut() {
            let game_state_response = Response::from_game_state(self.game_state.clone());
            if let Err(err) =
                client.await_request_send_response(GAME_STATE_REQUEST, &game_state_response)
            {
                perror_in_fn("send_game_state", err);
            }
        }
    }

    fn randomize_players(&mut self) {
        self.clients.shuffle(&mut thread_rng());
    }

    /// Starts core gameplay loop.
    fn start_game_loop(&mut self) {
        // Start turn
        // Get actions (loop)
        // End turn

        self.start_turn();
        self.start_action_loop();
        self.game_state.move_next_player();

        todo!()
    }

    fn start_turn(&mut self) {
        let turn_player = self.game_state.turn_player();
        let turn_start = Response::new_turn_start(turn_player.name().to_string());
        self.response_all(turn_start);
    }

    fn response_all(&mut self, response: Response) {
        for client in self.clients.iter_mut() {
            if let Err(err) = client.await_request_send_response(ACTION_REQUEST, &response) {
                perror_in_fn("response_all", err);
            }
        }
    }

    fn start_action_loop(&mut self) {
        todo!()
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
