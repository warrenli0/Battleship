pub struct Settings {
    num_rows: usize,
    num_cols: usize,
}

impl Settings {
    pub fn new(num_rows: usize, num_cols: usize) -> Settings {
        Settings {
            num_rows,
            num_cols
        }
    }

    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn get_num_cols(&self) -> usize {
        self.num_cols
    }
}