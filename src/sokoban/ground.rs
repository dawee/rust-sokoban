extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::math::{Matrix2d, identity};
use hydro::{GameObject, Provider, Sprite};

pub struct Ground {
    sprite: Sprite
}

impl Ground {

    pub fn new(position: (f64, f64)) -> Ground {
        let (x, y) = position;
        let sprite = Sprite::new(identity().trans(x, y), "GroundGravel_Concrete");

        Ground {sprite}
    }

}

impl GameObject for Ground {

    fn load(&self, provider: &mut Provider) {
        self.sprite.load(provider);
    }

    fn render(&self, provider: &Provider, parent_transform: &Matrix2d, gl: &mut GlGraphics) {
        self.sprite.render(provider, parent_transform, gl);
    }

}
