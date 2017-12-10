extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::{Texture, GlGraphics};
use graphics::Image;
use graphics::draw_state::DrawState;
use graphics::rectangle::square;
use graphics::math::Matrix2d;
use sokoban::Provider;

pub struct Drawable {
    draw_state: DrawState,
    image: Image,
    texture: Texture
}

impl Drawable {
    pub fn new(provider: &Provider, size: f64, texture_name: &str) -> Drawable {
        Drawable {
            draw_state: DrawState::new_alpha(),
            image: Image::new().rect(square(0.0, 0.0, size)),
            texture: provider.load_texture(texture_name)
        }
    }

    pub fn draw(&mut self, gl: &mut GlGraphics, transform: Matrix2d) {
        self.image.draw(&self.texture, &self.draw_state, transform, gl);
    }
}
