use crate::ship::Ship;
use crate::lib::{
    Space
};

pub struct Player {
    name: String,
    id: u32,
    board: Vec<Vec<Space>>,
    ships: Vec<Ship>,
}

impl Player {
    pub fn new() -> Player {
        todo!();
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_board(&self) -> &Vec<Vec<Space>> {
        &self.board
    }

    pub fn get_ships(&self) -> &Vec<Ship> {
        &self.ships
    }
}