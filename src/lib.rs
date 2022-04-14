#[derive(Clone, Copy)]
pub enum GameState {
    Preparation,
    InProgress,
    Complete,
}

#[derive(Clone, Copy)]
pub enum Space {
    Hit,
    Missed,
    Occupied,
    Empty,
}

#[derive(Clone, Copy)]
pub enum ShipType {
    Destroyer, 
    Submarine,
    PatrolBoat,
    Battleship,
    Carrier,
}