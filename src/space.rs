#[derive(Clone)]
pub struct Space {
    hit_or_missed: bool,
    occupant: Option<usize>,
}

impl Space {
    pub fn new_board(rows: usize, cols: usize) -> Vec<Vec<Space>> {
        vec![vec![Space::new(); cols]; rows]
    }

    fn new() -> Space {
        Space {
            hit_or_missed: false,
            occupant: None,
        }
    }

    pub fn was_targeted(&self) -> bool {
        self.hit_or_missed
    }

    pub fn shoot(&mut self) {
        self.hit_or_missed = true;
    }

    pub fn is_occupied(&self) -> bool {
        self.occupant.is_some()
    }

    pub fn get_occupant(&self) -> Option<usize> {
        self.occupant
    }

    pub fn set_occupant(&mut self, occupant: Option<usize>) {
        self.occupant = occupant
    }
}
