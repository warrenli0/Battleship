mod game;
mod player;
mod ship;
mod lib;
mod settings;
mod space;

use game::Game;
use settings::{Settings, GameImpl};

const NUM_ROWS: usize = 12;
const NUM_COLS: usize = 11;

fn main() {
    let settings: Settings = Settings::new(GameImpl::CommandLine, NUM_ROWS, NUM_COLS);
    let mut game: Game = Game::new(settings);
    game.init();
    game.print();
}
