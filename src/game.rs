use crate::cs128h_fiffffnal;

pub struct Game {
    game_state: GameState,
    players: [Player; 2],
    round: u16,
    turn: Option<&Player>,
    winner: Option<&Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            game_state: GameState::Preparation,
            players: (Player::new(), Player::new()),
            round: 0,
            turn: None,
            winner: None,
        }
    }

    pub fn get_game_state(&self) -> GameState {
        game_state
    }

    pub fn get_player(&self, id: u32) -> &Player {
        todo!();
    }

    pub fn get_round(&self) -> u16 {
        round
    }

    pub fn get_turn(&self) -> Option<&Player> {
        turn
    }

    pub fn get_winner(&self) -> Option<&Player> {
        winner
    }

    pub fn check_for_winner(&self) -> Option<&Player> {
        todo!();
    }

    pub fn start_game(&mut self) {
        todo!();
    }

    pub fn end_game(&mut self) {
        todo!();
    }

    pub fn place_ship(&mut self) -> Result<> {
        todo!();
    }

    pub fn shoot(&mut self) -> Result<> {
        todo!();
    }
}