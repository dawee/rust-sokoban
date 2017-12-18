mod hydro;

use hydro::Game;

fn main() {
    let mut game = Game::new("Rust Sokoban", 800, 600);

    game.run();
}
