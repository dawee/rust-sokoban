extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::math::{Matrix2d, identity, transform_pos};
use hydro::{GameObject, Provider, Sprite};
use sokoban::Level;

pub struct Block {
    row: i32,
    col: i32,
    sprite: Sprite
}

impl Block {

    pub fn new(row: i32, col: i32) -> Block {
        let sprite = Sprite::new(identity(), "Crate_Blue");

        Block {row, col, sprite}
    }

}

impl GameObject for Block {

    fn load(&self, provider: &mut Provider) {
        self.sprite.load(provider);
    }

    fn render(&self, provider: &Provider, parent_transform: &Matrix2d, gl: &mut GlGraphics) {
        let transform = parent_transform.trans(self.col as f64 * 50.0, self.row as f64 * 50.0);

        self.sprite.render(provider, &transform, gl);
    }

}
