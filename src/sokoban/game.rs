extern crate graphics;
extern crate viewport;

use graphics::clear;
use graphics::context::Context;
use sokoban::{Character, Provider};
use opengl_graphics::GlGraphics;

pub trait GameObject {
  fn load() -> Self;
  fn update(&mut self, f64) {}
  fn render(&mut self, &Context, &mut GlGraphics) {}
}

pub struct Game {
    character: Character
}

impl GameObject for Game {

    fn load() -> Game {
        let provider = Provider::new();
        let character = Character::load(&provider);

        Game {character}
    }

    fn update(&mut self, dt: f64) {
        self.character.update(dt);
    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
      clear([0.0, 0.0, 0.0, 1.0], gl);
      self.character.render(context, gl);
    }

}
