#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    Preparation,
    InProgress,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaceShipError {
    InvalidGameState,
    InvalidShipNum,
    AlreadyOccupied,
    OutOfBounds,
}