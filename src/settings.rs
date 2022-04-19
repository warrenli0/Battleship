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

    pub fn is_in_bounds(&self, ship_size: u8, pos: ShipPosition) -> bool {
        if pos.row < 0 || pos.row >= self.num_rows || pos.col < 0 || pos.col >= self.num_cols {
            return false;
        }
        if pos.is_horizontal && pos.col + ship_size as usize - 1 >= self.num_cols {
            return false;
        }
        if !pos.is_horizontal && pos.row + ship_size as usize - 1 >= self.num_rows {
            return false;
        }
        true
    }
}

pub enum GameImpl {
    CommandLine,
    // Graphics,
}