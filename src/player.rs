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

    pub fn place_ship(&mut self, ship_num: usize, pos: ShipPosition) -> Result<(), PlaceShipError> {
        let ship: &mut Ship = match self.ships.get_mut(ship_num) {
            Some(ship) => ship,
            None => return Err(PlaceShipError::InvalidShip)
        };
        let ship_size: u8 = ship.get_size();

        if !pos.is_in_bounds(self.board.len(), self.board.get(0).unwrap().len(), ship_size) {
            return Err(PlaceShipError::OutOfBounds);
        }

        if pos.is_horizontal {
            for row in pos.row..pos.row + ship_size as usize {
                if *self.board.get(row).unwrap().get(pos.col).unwrap() != Space::Empty {
                    return Err(PlaceShipError::AlreadyOccupied);
                }
            }
        } else {
            for col in pos.col..pos.col + ship_size as usize {
                if *self.board.get(pos.row).unwrap().get(col).unwrap() != Space::Empty {
                    return Err(PlaceShipError::AlreadyOccupied);
                }
            }
        }

        ship.change_position(pos);
        Ok(())
    }
}