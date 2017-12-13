extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::math::{Matrix2d, identity};
use hydro::{GameObject, Provider, Sprite};

pub struct Wall {
    sprite: Sprite,
    transform: Matrix2d
}

impl Wall {

    pub fn new(position: (f64, f64)) -> Wall {
        let (x, y) = position;
        let transform = identity().trans(x, y);
        let sprite = Sprite::new(identity(), "Wall_Black");

        Wall {sprite, transform}
    }

}

impl GameObject for Wall {

    fn load(&self, provider: &mut Provider) {
        self.sprite.load(provider);
    }

    fn render(&self, provider: &Provider, parent_transform: &Matrix2d, gl: &mut GlGraphics) {
        let transform = parent_transform.append_transform(self.transform);

        self.sprite.render(provider, &transform, gl);
    }

}
