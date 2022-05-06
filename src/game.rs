use crate::player::{Player, PlayerNumber};
use crate::settings::Settings;
use crate::ship::{ShipPosition};
use crate::space::{Space};
use crate::lib::{ GameState, PlaceShipError };
use crate::utils;

pub struct Game {
    settings: Settings,
    game_state: GameState,
    players: [Player; 2],
    round: u16,
    turn: Option<usize>,
    winner: Option<usize>,
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

    fn get_player(&self, player_num: usize) -> &Player {
        &self.players[player_num]
    }

    fn get_player_mut(&mut self, player_num: usize) -> &mut Player {
        &mut self.players[player_num]
    }
    
    fn switch_turn(&mut self) {
        if self.turn.is_some() {
            if self.turn.unwrap() == 0 {
                self.turn = Some(1);
            } else {
                self.turn = Some(0);
            }
        }
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
        let player_num: usize = self.turn.unwrap();

        print!("{esc}c", esc = 27 as char);  // clears console
        self.print_board(player_num);
        println!("Player {}'s Turn", player_num + 1);
        print!("Your ships: ");
        self.get_player(player_num).print_ships();
        if self.get_player(player_num).all_ships_placed() {
            print!("\nAll of your ships are placed. Type 'ready' if you are done placing ships.");
        }
        println!("\nWhere would you like to place a ship? Format: <ship number>, <position>, <'h' for horizontal or 'v' for vertical>");
        self.preparation_input_handler();
    }

    fn preparation_input_handler(&mut self) {;
        let player_num: usize = self.turn.unwrap();
        loop {
            let input: String = utils::read_input();
            let parsed_input = self.parse_preparation_input(input);
            if parsed_input.is_ok() {
                let parsed_input = parsed_input.unwrap();
                if parsed_input.is_some() {  // if player wants to place a ship
                    let ship_num: usize = parsed_input.unwrap().0;
                    let ship_pos: ShipPosition = parsed_input.unwrap().1;
                    if (self.place_ship(player_num, ship_num, ship_pos).is_ok()) {
                        break;
                    }
                } else if parsed_input.is_none() && self.get_player(player_num).all_ships_placed() {  // if user is done placing ships
                    if player_num == self.players.len() - 1 {
                        self.game_state = GameState::InProgress;
                    }
                    self.switch_turn();
                    break;
                }
            };
            println!("Invalid input. Example input: 2, 3B, H (place ship 2 at position 3B horizontally)");
        };
    }

    // returns: None if player is done placing ships; Some if player inputs any valid board position
    fn parse_preparation_input(&self, input: String) -> Result<Option<(usize, ShipPosition)>, ()> {
        if input.trim().to_ascii_lowercase() == "ready" {
            return Ok(None);
        }
        
        // Format: <ship number>, <position>, <'h' for horizontal or 'v' for vertical>
        let parts: Vec<&str> = input.as_str().split(',').collect();
        if parts.len() != 3 {
            return Err(());
        }

        let ship_num = (*parts.get(0).unwrap()).trim().parse::<usize>();
        let pos_str = (*parts.get(1).unwrap()).trim();
        let orientation = (*parts.get(2).unwrap()).trim().parse::<char>();

        if ship_num.is_ok() && orientation.is_ok() {
            let ship_num: usize = ship_num.unwrap();
            let orientation: char = orientation.unwrap().to_ascii_lowercase();
            let parsed_pos = self.settings.parse_alphanum_pos(pos_str);
            if ship_num < self.settings.get_num_ships() && parsed_pos.is_ok() && parsed_pos.is_ok() && (orientation == 'h' || orientation == 'v') {
                let (row, col) = parsed_pos.unwrap();
                return Ok(Some((ship_num, ShipPosition {row, col, is_horizontal: orientation == 'h'})));
            }
        }
        Err(())
    }

    fn place_ship(&mut self, player_num: usize, ship_num: usize, pos: ShipPosition) -> Result<(), PlaceShipError> {
        match self.game_state {
            GameState::Preparation => self.get_player_mut(player_num).place_ship(ship_num, pos),
            _ => Err(PlaceShipError::InvalidGameState)
        }
    }

    fn in_progress_handler(&mut self) {
        todo!();
        // read input for hits
    }

    fn shoot(&mut self, player_num: usize, row: usize, col: usize) -> Result<bool, PlaceShipError> {
        let player: &mut Player = self.get_player_mut(player_num);
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

    fn get_winner(&self) -> Option<PlayerNumber> {
        let p1_loss = self.players[0].loses();
        let p2_loss = self.players[1].loses();
        if p2_loss {
            return Some(PlayerNumber::One);
        }
        if p1_loss {
            return Some(PlayerNumber::Two);
        }
        return None;
    }

    fn player_loses(&self, player_number: usize) -> bool {
        let player = &self.players[player_number];
        let ships = player.get_ships();
        for ship in ships {
            let ship_position = match ship.get_pos() {
                Some(position) => position,
                None => {
                    return false;
                }
            };
            let mut current_row = ship_position.row;
            let mut current_column = ship_position.col;
            for _i in 0..ship.get_size() {
                let space = player.get_space(current_row, current_column);
                if !space.was_targeted() {
                    return false;
                }
                if ship_position.is_horizontal {
                    current_row += 1;
                } else {
                    current_column += 1;
                }
            }
        }
        return true;
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

    fn print_board(&self, player_num: usize) {
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