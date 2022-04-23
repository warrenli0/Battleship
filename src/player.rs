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

    pub fn get_ships(&self) -> &Vec<Ship> {
        &self.ships
    }

    pub fn get_pos(&self, row: usize, col: usize) -> Space {
        return self.board[row][col];
    }

    pub fn set_pos(&mut self, row: usize, col: usize, space: Space) {
        self.board[row][col] = space;
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
            for col in pos.col..pos.col + ship_size as usize {
                let mut space: &mut Space = self.board.get_mut(pos.row).unwrap().get_mut(col).unwrap();
                if *space != Space::Empty {
                    return Err(PlaceShipError::AlreadyOccupied);
                }
                *space = Space::Occupied;
            }
        } else {
            for row in pos.row..pos.row + ship_size as usize {
                let mut space: &mut Space = self.board.get_mut(row).unwrap().get_mut(pos.col).unwrap();
                if *space != Space::Empty {
                    return Err(PlaceShipError::AlreadyOccupied);
                }
                *space = Space::Occupied;
            }
        }

        ship.change_position(pos);
        Ok(())
    }

    pub fn print_ships(&self) {
        for idx in 0..self.ships.len() {
            print!("{:?} ({}), ", self.ships.get(idx).unwrap().get_type(), idx);
        }
    }
}