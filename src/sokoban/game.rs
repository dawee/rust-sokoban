extern crate graphics;
extern crate viewport;

use graphics::clear;
use viewport::Viewport;
use sokoban::{Character, Provider};

pub struct Game {
  provider: Provider,
  character: Character
}

impl Game {

  pub fn load() -> Game {
    let provider = Provider::new();
    let character = Character::load(&provider);

    Game {provider, character}
  }

  pub fn update(&mut self, dt: f64) {
    self.character.update(dt);
  }

  pub fn render(&mut self, viewport: &Viewport) {
    self.clear(viewport);
    self.character.render(viewport, &mut self.provider);
  }

  fn clear(&mut self, viewport: &Viewport) {
    self.provider.graphics.draw(*viewport, |_, gl| {
      clear([0.0, 0.0, 0.0, 1.0], gl);
    });
  }

}
