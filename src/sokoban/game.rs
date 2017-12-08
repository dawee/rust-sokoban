extern crate graphics;
extern crate viewport;

use graphics::clear;
use graphics::context::Context;
use sokoban::{Character, Provider};
use opengl_graphics::GlGraphics;

pub struct Game {
    character: Character
}

impl Game {

    pub fn load() -> Game {
        let provider = Provider::new();
        let character = Character::load(&provider);

        Game {character}
    }

    pub fn update(&mut self, dt: f64) {
        self.character.update(dt);
    }

    pub fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
      clear([0.0, 0.0, 0.0, 1.0], gl);
      self.character.render(context, gl);
    }

}
