mod hydro;

use hydro::{Chunk, Game, Provider, Scene, Transform};

struct Sokoban {}

impl Chunk for Sokoban {
    fn render(&self, scene: &mut Scene, origin: &Transform) {}
}

fn main() {
    let mut sokoban = Sokoban {};
    let mut game = Game::new("Rust Sokoban", [800, 600]);

    game.run(&mut sokoban);
}
