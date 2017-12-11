extern crate piston;
extern crate graphics;
extern crate viewport;

mod provider;
mod sprite;

pub use self::provider::Provider;
pub use self::sprite::Sprite;

use graphics::context::Context;
use opengl_graphics::GlGraphics;

pub trait GameObject {
    fn load(&self, &mut Provider) {}
    fn update(&mut self, f64) {}
    fn render(&self, &Provider, &Context, &mut GlGraphics) {}
}
