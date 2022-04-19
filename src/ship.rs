const NUM_SHIPS: usize = 5;

pub struct Ship {
    ship_type: ShipType,
    position: Option<ShipPosition>,
    health: u8,
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
            position: None,
            health: ship_type.get_size(),
        }
    }

    pub fn get_type(&self) -> ShipType {
        self.ship_type
    }

    pub fn get_size(&self) -> u8 {
        self.ship_type.get_size()
    }

    pub fn change_position(&mut self, pos: ShipPosition) {
        self.position = Some(pos);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShipType {
    Battleship,
    Carrier,
    Destroyer,
    PatrolBoat,
    Submarine,
}

impl ShipType {
    fn get_size(&self) -> u8 {
        match self {
            Battleship => 4,
            Carrier => 5,
            Destroyer => 3,
            PatrolBoat => 1,
            Submarine => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShipPosition {
    pub row: usize,
    pub col: usize,
    pub is_horizontal: bool,
}

impl ShipPosition {
    pub fn is_in_bounds(&self, num_rows: usize, num_cols: usize, ship_size: u8) -> bool {
        if self.row >= num_rows || self.col >= num_cols {
            return false;
        }
        if self.is_horizontal && self.col + ship_size as usize - 1 >= num_cols {
            return false;
        }
        if !self.is_horizontal && self.row + ship_size as usize - 1 >= num_rows {
            return false;
        }
        true
    }
}