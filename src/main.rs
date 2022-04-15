mod game;
mod player;
mod ship;
mod lib;
mod settings;

use game::Game;
use settings::Settings;

const NUM_ROWS: usize = 12;
const NUM_COLS: usize = 11;

fn main() {
    let settings: Settings = Settings::new(NUM_ROWS, NUM_COLS);
    let game: Game = Game::new(settings);
    game.print();
}
