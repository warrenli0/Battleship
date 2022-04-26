use crate::ship::{ Ship, ShipType, ShipPosition };
use crate::space::{Space};
use crate::lib::{ PlaceShipError };

pub struct Player {
    board: Vec<Vec<Space>>,
    ships: Vec<Ship>,
}

impl Player {
    pub fn new(rows: usize, cols: usize) -> Player {
        Player {
            board: Space::new_board(rows, cols),
            ships: Ship::new_ships_vec(),
        }
    }

    pub fn get_board(&self) -> &Vec<Vec<Space>> {
        &self.board
    }

    pub fn get_ships(&self) -> &Vec<Ship> {
        &self.ships
    }

    pub fn get_space(&self, row: usize, col: usize) -> &Space {
        &self.board[row][col]
    }

    pub fn get_space_mut(&mut self, row: usize, col: usize) -> &mut Space {
        &mut self.board[row][col]
    }

    pub fn empty_board(&self) -> bool {
        for row in self.board.iter() {
            for space in row.iter() {
                if space.is_occupied() {
                    return false;
                }
            }
        }
        true
    }

    pub fn place_ship(&mut self, ship_num: usize, pos: ShipPosition) -> Result<(), PlaceShipError> {
        // find ship with ship_num
        let ship_size: u8 = match self.ships.get(ship_num) {
            Some(ship) => ship.get_size(),
            None => return Err(PlaceShipError::InvalidShipNum)
        };

        // check if ship's new position is valid
        {let new_ship_spaces = match self.get_board_spaces_from_pos(pos, ship_size as usize) {
            Ok(spaces) => spaces,
            Err(()) => return Err(PlaceShipError::OutOfBounds)
        };
        for space in new_ship_spaces {
            if space.is_occupied() && space.get_occupant().unwrap() != ship_num {
                return Err(PlaceShipError::AlreadyOccupied);
            }
        }}

        // remove ship's former position (if exists)
        let prev_pos: Option<ShipPosition> = self.ships.get(ship_num).unwrap().get_pos();
        if prev_pos.is_some() {
            let prev_ship_spaces = self.get_board_spaces_from_pos(prev_pos.unwrap(), ship_size as usize).unwrap();
            for space in prev_ship_spaces {
                space.set_occupant(None);
            }
        }

        // set ship's new position
        let new_ship_spaces = self.get_board_spaces_from_pos(pos, ship_size as usize).unwrap();
        for space in new_ship_spaces {
            space.set_occupant(Some(ship_num));
        }
        self.ships.get_mut(ship_num).unwrap().change_position(pos);

        Ok(())
    }

    fn get_board_spaces_from_pos(&mut self, pos: ShipPosition, size: usize) -> Result<Vec<&mut Space>, ()> {
        if !pos.is_in_bounds(self.board.len(), self.board[0].len(), size) {
            return Err(());
        }
        if pos.is_horizontal {
            let mut spaces: Vec<&mut Space> = Vec::with_capacity(size);
            let mut idx: usize = 0;
            for space in self.board[pos.row].iter_mut() {
                if (idx >= pos.col && idx < pos.col + size) { spaces.push(space); }
                idx += 1;
            }
            return Ok(spaces);
        }
        let spaces: Vec<&mut Space> = self.board[pos.row..pos.row + size].iter_mut().map(|row| &mut row[pos.col]).collect();
        Ok(spaces)
    }

    pub fn print_ships(&self) {
        for idx in 0..self.ships.len() {
            let ship: &Ship = self.ships.get(idx).unwrap();
            print!("[{}] {:?}{}, ", idx, ship.get_type(), if ship.get_pos().is_some() {" (Placed)"} else {""});
        }
    }
}