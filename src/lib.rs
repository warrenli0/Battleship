enum BattleShipSpaces {
    Hit,
    Missed,
    Occupied,
    Empty,
}

enum GameState {
    Preparation,
    InProgress,
    Complete,
}

enum ShipType {
    Destroyer, 
    Submarine,
    Cruiser,
    Battleship,
    Carrier,
}

struct Board {
    space: BattleShipSpaces,
    p1_turn: bool,
    p2_turn: bool,
}