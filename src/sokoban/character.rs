extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::{Texture, GlGraphics};
use graphics::{Image, Transformed};
use graphics::draw_state::DrawState;
use graphics::rectangle::square;
use graphics::context::Context;
use graphics::math::{Matrix2d, identity, multiply};
use sokoban::{GameObject, Provider};

pub struct Character {
    image: Image,
    texture: Texture,
    draw_state: DrawState,
    transform: Matrix2d
}

impl GameObject for Character {

    fn load(provider: &Provider) -> Character {
        Character {
            draw_state: DrawState::new_alpha(),
            image: Image::new().rect(square(0.0, 0.0, 50.0)),
            texture: provider.load_texture("Character1"),
            transform: identity()
        }
    }

    fn update(&mut self, dt: f64) {
        self.transform = self.transform.trans(5.0 * dt, 0.0);
    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
      let transform: Matrix2d = multiply(context.transform, self.transform);

      self.image.draw(&self.texture, &self.draw_state, transform, gl);
    }

}
