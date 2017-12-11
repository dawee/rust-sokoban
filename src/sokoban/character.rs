extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::context::Context;
use graphics::math::identity;
use sokoban::{GameObject, Provider, Sprite};

pub struct Character {
    sprite: Sprite
}

impl Character {

    pub fn new(position: (f64, f64)) -> Character {
        let (x, y) = position;
        let sprite = Sprite::new(identity().trans(x, y), "Character1");

        Character {sprite}
    }

    pub fn move_up(&mut self) {
        self.sprite.translate_y(-50.0);
    }

    pub fn move_right(&mut self) {
        self.sprite.translate_x(50.0);
    }

    pub fn move_down(&mut self) {
        self.sprite.translate_y(50.0);
    }

    pub fn move_left(&mut self) {
        self.sprite.translate_x(-50.0);
    }

}

impl GameObject for Character {

    fn load(&mut self, provider: &mut Provider) {
        self.sprite.load(provider);
    }

    fn render(&mut self, provider: &mut Provider, context: &Context, gl: &mut GlGraphics) {
        self.sprite.render(provider, context, gl);
    }

}
