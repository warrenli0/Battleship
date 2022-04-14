mod game;
mod player;
mod ship;
mod lib;

use game::Game;

fn main() {
    let game: Game = Game::new();
    game.print();
}
