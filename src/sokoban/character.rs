extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::context::Context;
use graphics::math::{Matrix2d, identity, multiply};
use sokoban::{Drawable, GameObject, Provider};

pub trait Movable {
    fn move_up(&mut self);
    fn move_right(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
}

pub struct Character {
    drawable: Drawable,
    transform: Matrix2d
}

impl GameObject for Character {

    fn load(provider: &Provider) -> Character {
        Character {
            drawable: Drawable::new(provider, 50.0, "Character1"),
            transform: identity()
        }
    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
      let transform: Matrix2d = multiply(context.transform, self.transform);

      self.drawable.draw(gl, transform);
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
