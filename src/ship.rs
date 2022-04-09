pub struct Ship {
    ship_type: ShipType,
    size: u8,
    is_horizontal: bool,
    occupied_spaces: Vec<&BattleShipSpaces>,
}

impl Ship {
    pub fn new() {
        todo!();
    }

    pub fn get_type(&self) -> ShipType {
        ship_type
    }

    pub fn get_size(&self) -> u8 {
        size
    }

    pub fn is_horizontal(&self) -> bool {
        is_horizontal
    }

    pub fn get_occupied_spaces(&self) -> Vec<BattleShipSpaces> {
        &occupied_spaces
    }
}