use crate::player::Player;
use crate::settings::Settings;
use crate::ship::{Ship, ShipPosition};
use crate::lib::{ GameState, PlaceShipError, Space };

pub struct Game {
    settings: Settings,
    game_state: GameState,
    players: [Player; 2],
    round: u16,
    turn: Option<u8>,
    winner: Option<u8>,
}

impl Game {
    pub fn new(settings: Settings) -> Game {
        let num_rows: usize = settings.get_num_rows();
        let num_cols: usize = settings.get_num_cols();
        Game {
            settings,
            game_state: GameState::Preparation,
            players: [Player::new(num_rows, num_cols), Player::new(num_rows, num_cols)],
            round: 0,
            turn: None,
            winner: None,
        }
    }

    fn get_player(&self, player_num: u8) -> &Player {
        &self.players[player_num as usize]
    }

    fn get_player_mut(&mut self, player_num: u8) -> &mut Player {
        &mut self.players[player_num as usize]
    }

    pub fn start_game(&mut self) {
        if self.has_started() {
            println!("The game has already started!");
            return;
        }

        while self.game_state == GameState::Preparation {
            self.preparation_handler();
        }

        while self.game_state == GameState::InProgress {
            self.in_progress_handler();
        }
    }

    fn has_started(&self) -> bool {
        self.game_state == GameState::Preparation && self.players[0].empty_board() && self.players[1].empty_board()
        && self.round == 0 && self.turn.is_none() && self.winner.is_none()
    }

    fn preparation_handler(&mut self) {
        todo!();
        // read input for ship placement
        // when user types in 'ready', the game will move on if all ships were placed
    }

    fn place_ship(&mut self, player_num: u8, ship: &mut Ship, position: ShipPosition) -> Result<(), PlaceShipError> {
        match self.settings.is_in_bounds(ship.get_size(), position) {
            true => match self.game_state {
                GameState::Preparation => self.get_player_mut(player_num).place_ship(ship, position),
                _ => Err(PlaceShipError::InvalidGameState)
            },
            false => Err(PlaceShipError::OutOfBounds)
        }
    }

    fn in_progress_handler(&mut self) {
        todo!();
        // read input for hits
    }

    fn shoot(&mut self) -> Result<bool, bool> {
        todo!();
    }

    fn check_for_winner(&self) -> Option<u8> {
        todo!();
    }

    fn end_game(&mut self) {
        todo!();
    }

    pub fn print(&self) {
        println!("========== GAME DETAILS ==========");
        println!("\nThe board of Player 1:");
        self.print_board(0);

        println!("\nThe board of Player 2:");
        self.print_board(1);

        println!("\nGame State: {:?}", self.game_state);
        println!("Round: {:?}", self.round);
        
        if self.turn.is_none() {
            println!("Turn: None");
        } else {
            println!("Turn: {:?}", self.turn.unwrap());
        }

        if self.turn.is_none() {
            println!("Winner: None");
        } else {
            println!("Winner: {:?}", self.winner.unwrap());
        }
    }

    fn print_board(&self, player_num: u8) {
        let player: &Player = self.get_player(player_num);
        let board: &Vec<Vec<Space>> = player.get_board();

        for row in 0..self.settings.get_num_rows() {
            print!("{:2} ", row + 1);
            for space in board.get(row).unwrap() {
                if *space == Space::Empty {
                    print!(" - ");
                } else if *space == Space::Hit {
                    print!(" H "); // 💥
                } else if *space == Space::Missed {
                    print!(" M "); // 💧
                } else if *space == Space::Occupied {
                    print!(" S ");
                }
            }
            println!();
        }

        print!("   ");
        let mut col_label: char = 'A';
        for col in 0..self.settings.get_num_cols() {
            print!(" {} ", col_label);
            col_label = char::from_u32(col_label as u32 + 1).unwrap_or(col_label);
        }
        println!();
    }
}