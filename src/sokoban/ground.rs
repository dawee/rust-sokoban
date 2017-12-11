extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::context::Context;
use graphics::math::identity;
use hydro::{GameObject, Provider, Sprite};

pub struct Ground {
    sprite: Sprite
}

impl Ground {

    pub fn new(position: (f64, f64)) -> Ground {
        let (x, y) = position;
        let sprite = Sprite::new(identity().trans(x, y), "Ground_Concrete");

        Ground {sprite}
    }

}

impl GameObject for Ground {

    fn load(&mut self, provider: &mut Provider) {
        self.sprite.load(provider);
    }

    fn render(&mut self, provider: &mut Provider, context: &Context, gl: &mut GlGraphics) {
        self.sprite.render(provider, context, gl);
    }

}
