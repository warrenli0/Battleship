use crate::ship::{ShipPosition};

const MAX_NUM_ROWS: usize = 26;  // max number of rows is 26 because we use an alphanumeric positioning system (with only 1 letter)
pub struct Settings {
    game_impl: GameImpl,
    num_rows: usize,
    num_cols: usize,
}

impl Settings {
    pub fn new(game_impl: GameImpl, num_rows: usize, num_cols: usize) -> Settings {
        assert!(num_rows <= MAX_NUM_ROWS);
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

    pub fn parse_alphanum_pos(&self, pos: &str) -> Result<(usize, usize), &str> {
        let chars: Vec<char> = pos.chars().collect();
        if chars.len() < 2 { return Err("Missing digit or letter. Can only parse the pattern: <digit><letter>"); }

        let first_char: char = *chars.get(0).unwrap();
        let last_char: char = *chars.get(chars.len() - 1).unwrap();
        if !first_char.is_digit(10) || !last_char.is_ascii_alphabetic() {
            return Err("Missing digit or letter. Can only parse the pattern: <digit><letter>");
        }

        let mut row: usize = first_char.to_digit(10).unwrap() as usize;
        for idx in 1..chars.len() - 1 {
            let next_char: char = *chars.get(idx).unwrap();
            if !next_char.is_digit(10) {
                return Err("Too many letters. Can only parse the pattern: <digit><letter>");
            }
            let next_digit = next_char.to_digit(10).unwrap() as usize;
            row = match row.checked_mul(10) {
                Some(row) => match row.checked_add(next_digit) {
                    Some(row) => row,
                    None => return Err("Row number is too large for a usize")
                },
                None => return Err("Row number is too large for a usize")
            };
        }

        let col: usize = last_char.to_ascii_uppercase() as usize;
        if row == 0 || row - 1 >= self.num_rows || col < 65 || col - 65 >= self.num_cols {
            return Err("Position is outside of the board");
        }

        Ok((row - 1, col - 65))
    }
}

pub enum GameImpl {
    CommandLine,
    // Graphics,
}