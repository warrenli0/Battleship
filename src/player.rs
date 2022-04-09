pub struct Player {
    name: String,
    id: u32,
    board: Vec<Vec<BattleShipSpaces>>,
    ships: Vec<Ship>,
}

impl Player {
    pub fn new() {
        todo!();
    }

    pub fn get_name(&self) -> &String {
        &name
    }

    pub fn get_id(&self) -> u32 {
        id
    }

    pub fn get_board(&self) -> &Vec<Vec<BattleShipSpaces>> {
        &board
    }

    pub fn get_ships(&self) -> &Vec<Ship> {
        ships
    }
}