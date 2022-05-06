#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    Preparation,
    Attack,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaceShipError {
    InvalidGameState,
    InvalidShipNum,
    AlreadyOccupied,
    OutOfBounds,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShootError {
    AlreadyTargeted,
    OutOfBounds,
}