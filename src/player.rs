use crate::ship::{ Ship, ShipType, ShipPosition };
use crate::lib::{ PlaceShipError, Space };

pub struct Player {
    board: Vec<Vec<Space>>,
    ships: Vec<Ship>,
}

impl Player {
    pub fn new(rows: usize, cols: usize) -> Player {
        Player {
            board: vec![vec![Space::Empty; cols]; rows],
            ships: Ship::get_ships_vec(),
        }
    }

    pub fn get_board(&self) -> &Vec<Vec<Space>> {
        &self.board
    }

    pub fn empty_board(&self) -> bool {
        for row in self.board.iter() {
            for space in row.iter() {
                if *space != Space::Empty {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_ships(&self) -> &Vec<Ship> {
        &self.ships
    }

    pub fn place_ship(&mut self, ship: &mut Ship, pos: ShipPosition) -> Result<(), PlaceShipError> {
        ship.change_position(pos);
        Ok(())
    }
}