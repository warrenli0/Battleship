use crate::player::Player;
use crate::settings::Settings;
use crate::ship::{ShipPosition};
use crate::space::{Space};
use crate::lib::{ GameState, PlaceShipError };

use std::io::{self, Read};

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

    pub fn init(&mut self) {
        if self.has_started() {
            println!("The game has already started!");
            return;
        }

        self.turn = Some(0);
        while self.game_state == GameState::Preparation {
            self.preparation_handler();
        }

        while self.game_state == GameState::InProgress {
            self.in_progress_handler();
        }
    }

    fn has_started(&self) -> bool {
        !(self.game_state == GameState::Preparation && self.players[0].empty_board() && self.players[1].empty_board()
        && self.round == 0 && self.turn.is_none() && self.winner.is_none())
    }

    fn preparation_handler(&mut self) {
        let player_num: u8 = self.turn.unwrap();

        print!("{esc}c", esc = 27 as char);
        self.print_board(player_num);
        println!("Player {}'s Turn", player_num);
        print!("Your ships: ");
        self.get_player(player_num).print_ships();
        println!("\nWhere would you like to place a ship? Format: <ship number>, <position>, <'h' for horizontal or 'v' for vertical>");

        self.place_ship_from_input(player_num, self.get_player(player_num).get_ships().len());
    }

    fn place_ship_from_input(&mut self, player_num: u8, num_ships: usize) {
        loop {
            let mut buffer: String = String::new();
            io::stdin().read_line(&mut buffer).expect("Unable to read from standard input; try again");
            let parts: Vec<&str> = buffer.as_str().split(',').collect();
            if parts.len() == 3 {
                let ship_num = (*parts.get(0).unwrap()).trim().parse::<usize>();
                let pos_str = (*parts.get(1).unwrap()).trim();
                let orientation = (*parts.get(2).unwrap()).trim().parse::<char>();

                if ship_num.is_ok() && orientation.is_ok() {
                    let ship_num: usize = ship_num.unwrap();
                    let orientation: char = orientation.unwrap().to_ascii_lowercase();
                    let parsed_pos = self.settings.parse_alphanum_pos(pos_str);
                    if ship_num < num_ships && parsed_pos.is_ok() && (orientation == 'h' || orientation == 'v') {
                        let (row, col) = parsed_pos.unwrap();
                        if self.place_ship(player_num, ship_num, ShipPosition {row, col, is_horizontal: orientation == 'h'}).is_ok() {
                            break;
                        }
                    }
                }
            }
            println!("Invalid input. Example input: 2, 3B, H (place ship 2 at position 3B horizontally)");
        };
    }

    fn place_ship(&mut self, player_num: u8, ship_num: usize, pos: ShipPosition) -> Result<(), PlaceShipError> {
        match self.game_state {
            GameState::Preparation => self.get_player_mut(player_num).place_ship(ship_num, pos),
            _ => Err(PlaceShipError::InvalidGameState)
        }
    }

    fn in_progress_handler(&mut self) {
        todo!();
        // read input for hits
    }

    fn shoot(&mut self, player_num: u8, row: usize, col: usize) -> Result<bool, PlaceShipError> {
        let mut player: &mut Player = self.get_player_mut(player_num);
        if row >= player.get_board().len() || row < 0 || col >= player.get_board()[0].len() || col < 0 {
            return Err(PlaceShipError::OutOfBounds);
        }
        player.get_space_mut(row,col).shoot();
        if player.get_space(row,col).is_occupied() {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn check_for_winner(&self) -> Option<u8> {
        todo!();
    }

    fn end(&mut self) {
        self.game_state = GameState::Complete;
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
                match space.get_occupant() {
                    Some(ship_num) => match space.was_targeted() {
                        true => print!(" X "),  // 💥
                        false => print!(" {} ", ship_num)
                    },
                    None => match space.was_targeted() {
                        true => print!(" ~ "),  // 💧
                        false => print!(" - ")
                    }
                };
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