use crate::player::Player;
use crate::lib::{
    GameState
};

pub struct Game {
    game_state: GameState,
    players: [Player; 2],
    round: u16,
    turn: Option<u8>,
    winner: Option<u8>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            game_state: GameState::Preparation,
            players: [Player::new(), Player::new()],
            round: 0,
            turn: None,
            winner: None,
        }
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }

    pub fn get_player(&self, id: u32) -> &Player {
        todo!();
    }

    pub fn get_round(&self) -> u16 {
        self.round
    }

    pub fn get_turn(&self) -> Option<u8> {
        self.turn
    }

    pub fn get_winner(&self) -> Option<u8> {
        self.winner
    }

    pub fn check_for_winner(&self) -> Option<u8> {
        todo!();
    }

    pub fn start_game(&mut self) {
        todo!();
    }

    pub fn end_game(&mut self) {
        todo!();
    }

    pub fn place_ship(&mut self) -> Result<bool, bool> {
        todo!();
    }

    pub fn shoot(&mut self) -> Result<bool, bool> {
        todo!();
    }
}