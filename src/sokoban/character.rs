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

pub trait Movable {
    fn move_up(&mut self);
    fn move_right(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
}

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

    fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
      let transform: Matrix2d = multiply(context.transform, self.transform);

      self.image.draw(&self.texture, &self.draw_state, transform, gl);
    }

}

impl Movable for Character {

    fn move_up(&mut self) {
        self.transform = self.transform.trans(0.0, -50.0);
    }

    fn move_right(&mut self) {
        self.transform = self.transform.trans(50.0, 0.0);
    }

    fn move_down(&mut self) {
        self.transform = self.transform.trans(0.0, 50.0);
    }

    fn move_left(&mut self) {
        self.transform = self.transform.trans(-50.0, 0.0);
    }

}
