extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::{Texture, GlGraphics};
use graphics::{Image, Transformed};
use graphics::draw_state::DrawState;
use graphics::rectangle::square;
use graphics::context::Context;
use sokoban::Provider;

pub struct Character {
    x: f64,
    y: f64,
    image: Image,
    texture: Texture,
    draw_state: DrawState
}

impl Character {

    pub fn load(provider: &Provider) -> Character {
        Character {
            x: 0.0,
            y: 0.0,
            draw_state: DrawState::new_alpha(),
            image: Image::new().rect(square(0.0, 0.0, 50.0)),
            texture: provider.load_texture("Character1")
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.x += 5.0 * dt;
    }

    pub fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
      self.image.draw(
          &self.texture,
          &self.draw_state,
          context.transform.trans(self.x, self.y),
          gl
      );
    }
}
