use crate::ship::Ship;
use crate::lib::{ Space };

pub const ROWS: usize = 10;
pub const COLS: usize = 10;

pub struct Player {
    board: Vec<Vec<Space>>,
    ships: Vec<Ship>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            board: vec![vec![Space::Empty; COLS]; ROWS],
            ships: Ship::get_ships_vec(),
        }
    }

    pub fn get_board(&self) -> &Vec<Vec<Space>> {
        &self.board
    }

    pub fn get_ships(&self) -> &Vec<Ship> {
        &self.ships
    }
}