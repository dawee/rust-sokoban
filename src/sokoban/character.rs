extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::context::Context;
use graphics::math::{Matrix2d, identity, multiply};
use sokoban::{GameObject, Movable, Provider, Sprite};

pub struct Character {
    sprite: Sprite
}

impl Character {
    pub fn new(position: (f64, f64)) -> Character {
        let (x, y) = position;
        let sprite = Sprite::new(identity().trans(x, y));

        Character {sprite}
    }
}

impl GameObject for Character {

    fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
        self.sprite.render(context, gl);
    }

}

impl Movable for Character {

    fn move_up(&mut self) {
    }

    fn move_right(&mut self) {
    }

    fn move_down(&mut self) {
    }

    fn move_left(&mut self) {
    }

}
