use core::num;

use crate::lib::{ ShipType, Space };

const NUM_SHIPS: usize = 5;

pub struct Ship {
    ship_type: ShipType,
    size: u8,
    is_horizontal: bool,
}

impl Ship {
    pub fn get_ships_vec() -> Vec<Ship> {
        let mut ships: Vec<Ship> = Vec::with_capacity(NUM_SHIPS);
        ships.push(Ship::new(ShipType::Battleship));  // hard-coded for all ship types, for now
        ships.push(Ship::new(ShipType::Carrier));
        ships.push(Ship::new(ShipType::Destroyer));
        ships.push(Ship::new(ShipType::PatrolBoat));
        ships.push(Ship::new(ShipType::Submarine));
        ships
    }

    fn new(ship_type: ShipType) -> Ship {
        Ship {
            ship_type,
            size: Ship::get_ship_size(ship_type),
            is_horizontal: true,
        }
    }

    fn get_ship_size(ship_type: ShipType) -> u8 {
        match ship_type {
            ShipType::Battleship => 4,
            ShipType::Carrier => 5,
            ShipType::Destroyer => 3,
            ShipType::PatrolBoat => 1,
            ShipType::Submarine => 2,
        }
    }

    pub fn get_type(&self) -> ShipType {
        self.ship_type
    }

    pub fn get_size(&self) -> u8 {
        self.size
    }

    pub fn is_horizontal(&self) -> bool {
        self.is_horizontal
    }

    pub fn change_rotation(&mut self) {
        self.is_horizontal = !self.is_horizontal;
    }
}