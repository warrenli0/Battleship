use crate::player::{Player, PlayerNumber};
use crate::settings::Settings;
use crate::ship::{ShipPosition};
use crate::space::{Space};
use crate::lib::{ GameState, PlaceShipError, ShootError };
use crate::utils;

use std::{thread, time};

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

        while self.game_state == GameState::Attack {
            self.attack_handler();
        }

        print!("{esc}c", esc = 27 as char);  // clears console
        self.print();
    }

    fn has_started(&self) -> bool {
        !(self.game_state == GameState::Preparation && self.players[0].empty_board() && self.players[1].empty_board()
        && self.round == 0 && self.turn.is_none() && self.winner.is_none())
    }

    fn preparation_handler(&mut self) {
        let player_num: usize = self.turn.unwrap();

        print!("{esc}c", esc = 27 as char);  // clears console
        self.print_board(player_num, false);
        println!("Player {}'s Turn", player_num + 1);
        print!("Your ships: ");
        self.get_player(player_num).print_ships();
        println!("\nWhere would you like to place a ship? Format: <ship number>, <position>, <'h' for horizontal or 'v' for vertical>");
        if self.get_player(player_num).all_ships_placed() {
            println!("All of your ships are placed. Type 'ready' if you are done placing ships.");
        }
        self.preparation_input_handler();
    }

    fn preparation_input_handler(&mut self) {
        let player_num: usize = self.turn.unwrap();
        loop {
            let input: String = utils::read_input();
            let parsed_input = self.parse_preparation_input(input);
            if parsed_input.is_ok() {
                let parsed_input = parsed_input.unwrap();
                if parsed_input.is_some() {  // if player wants to place a ship
                    let ship_num: usize = parsed_input.unwrap().0;
                    let ship_pos: ShipPosition = parsed_input.unwrap().1;
                    if self.place_ship(player_num, ship_num, ship_pos).is_ok() {
                        break;
                    }
                } else if parsed_input.is_none() && self.get_player(player_num).all_ships_placed() {  // if user is done placing ships
                    if player_num == self.players.len() - 1 {
                        self.game_state = GameState::Attack;
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
            if ship_num < self.settings.get_num_ships() && parsed_pos.is_ok() && (orientation == 'h' || orientation == 'v') {
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

    fn attack_handler(&mut self) {
        let player_num = self.turn.unwrap();

        print!("{esc}c", esc = 27 as char);  // clears console
        println!("Your Board");
        self.print_board(player_num, false);
        println!("Enemy's Board");
        self.print_enemy_board(player_num);
        println!("Player {}'s Turn", player_num + 1);
        println!("Where would you like to shoot? Format: <position>");
        self.attack_input_handler();

        let winner: Option<PlayerNumber> = self.get_winner();
        if winner.is_some() {
            self.end(if winner.unwrap() == PlayerNumber::One { 0 } else { 1 });
        }
    }

    fn attack_input_handler(&mut self) {
        let player_num: usize = self.turn.unwrap();
        loop {
            let input: String = utils::read_input();
            let parsed_input = self.parse_attack_input(input);
            if parsed_input.is_err() {
                println!("Invalid input. Example input: 5D");
                continue;
            }

            let target_row_col: (usize, usize) = parsed_input.unwrap();
            let shoot_result = self.shoot_enemy(player_num, target_row_col.0, target_row_col.1);
            if shoot_result.is_ok() {
                if !shoot_result.unwrap() {
                    println!("Your shot missed! Switching turns in 3 seconds...");
                    self.switch_turn();
                    self.round += 1;
                    thread::sleep(time::Duration::from_secs(3));
                }
                break;
            }
            if shoot_result.err().unwrap() == ShootError::AlreadyTargeted {
                println!("You already targeted that space.");
            }
        };
    }

    fn parse_attack_input(&self, input: String) -> Result<(usize, usize), ()> {
        // Format: <position>
        match self.settings.parse_alphanum_pos(input.as_str()) {
            Ok(pos) => Ok(pos),
            Err(_) => Err(())
        }
    }

    fn shoot_enemy(&mut self, player_num: usize, row: usize, col: usize) -> Result<bool, ShootError> {
        self.get_player_mut(if player_num == 0 { 1 } else { 0 }).shoot(row, col)
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

    fn end(&mut self, winner_player_num: usize) {
        self.game_state = GameState::Complete;
        self.turn = None;
        self.winner = Some(winner_player_num);
    }

    pub fn print(&self) {
        println!("========== GAME DETAILS ==========");
        println!("\nThe board of Player 1:");
        self.print_board(0, false);

        println!("\nThe board of Player 2:");
        self.print_board(1, false);

        println!("\nRounds: {:?}", self.round);

        if self.winner.is_some() {
            println!("Winner: Player {:?}", self.winner.unwrap() + 1);
        }
    }

    fn print_board(&self, player_num: usize, hide_ship_locs: bool) {
        let player: &Player = self.get_player(player_num);
        let board: &Vec<Vec<Space>> = player.get_board();

        for row in 0..self.settings.get_num_rows() {
            print!("{:2} ", row + 1);
            for space in board.get(row).unwrap() {
                match space.get_occupant() {
                    Some(ship_num) => match space.was_targeted() {
                        true => {
                            if hide_ship_locs && player.get_ships().get(ship_num).unwrap().health == 0 {
                                print!(" {} ", ship_num);
                            } else {
                                print!(" X ")
                            }
                        },  // 💥
                        false => if hide_ship_locs { print!(" - ") } else { print!(" {} ", ship_num) }
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
        for _col in 0..self.settings.get_num_cols() {
            print!(" {} ", col_label);
            col_label = char::from_u32(col_label as u32 + 1).unwrap_or(col_label);
        }
        println!();
    }

    fn print_enemy_board(&self, player_num: usize) {
        self.print_board(if player_num == 0 { 1 } else { 0 }, true);
    }
}