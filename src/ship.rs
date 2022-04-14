use crate::lib::{
    ShipType,
    Space,
};

pub struct Ship {
    ship_type: ShipType,
    size: u8,
    is_horizontal: bool,
}

impl Ship {
    pub fn new() {
        todo!();
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
}