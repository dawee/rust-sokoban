extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::context::Context;
use graphics::math::{Matrix2d, identity, multiply};
use sokoban::{Drawable, GameObject, Movable, Provider};

pub struct Ground {
    drawable: Drawable,
    transform: Matrix2d
}

impl GameObject for Ground {

    fn load(provider: &Provider) -> Ground {
        Ground {
            drawable: Drawable::new(provider, 50.0, "GroundGravel_Concrete"),
            transform: identity()
        }
    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
      let transform: Matrix2d = multiply(context.transform, self.transform);

      self.drawable.draw(gl, transform);
    }

}

impl Movable for Ground {
    fn set_position(&mut self, x: f64, y: f64) {
        self.transform = self.transform.trans(x, y);
    }
}
