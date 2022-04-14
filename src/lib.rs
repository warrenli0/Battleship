#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    Preparation,
    InProgress,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Space {
    Hit,
    Missed,
    Occupied,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShipType {
    Battleship,
    Carrier,
    Destroyer,
    PatrolBoat,
    Submarine,
}