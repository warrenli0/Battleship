use crate::player::Player;
use crate::player::ROWS;
use crate::player::COLS;
use crate::lib::{ GameState, Space };

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

    pub fn get_player(&self, player_num: u8) -> &Player {
        &self.players[player_num as usize]
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

        for row in 0..board.len() {
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

        println!("    A  B  C  D  E  F  G  H  I  J");
    }
}