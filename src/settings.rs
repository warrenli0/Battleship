use crate::ship::{ShipPosition};

pub struct Settings {
    game_impl: GameImpl,
    num_rows: usize,
    num_cols: usize,
}

impl Settings {
    pub fn new(game_impl: GameImpl, num_rows: usize, num_cols: usize) -> Settings {
        Settings {
            game_impl,
            num_rows,
            num_cols,
        }
    }

    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn get_num_cols(&self) -> usize {
        self.num_cols
    }
}

pub enum GameImpl {
    CommandLine,
    // Graphics,
}