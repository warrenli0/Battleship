enum BattleShipSpaces {
    Hit,
    Missed,
    Occupied,
    Empty,
}

enum ShipType {
    Destroyer, 
    Submarine,
    Cruiser,
    Battleship,
    Carrier,
}

struct Board {
    BattleShipSpaces space,
    bool p1_turn,
    bool p2_turn,
}