extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::{Texture, GlGraphics};
use graphics::Image;
use graphics::draw_state::DrawState;
use graphics::rectangle::square;
use graphics::context::Context;
use graphics::math::{Matrix2d, multiply};
use graphics::Transformed;
use sokoban::{Provider, GameObject};

pub struct Sprite {
    texture_name: String,
    draw_state: DrawState,
    image: Image,
    transform: Matrix2d
}

impl Sprite {
    pub fn new(transform: Matrix2d, texture_name: &str) -> Sprite {
        Sprite {
            transform,
            texture_name: String::from(texture_name),
            draw_state: DrawState::new_alpha(),
            image: Image::new().rect(square(0.0, 0.0, 50.0)),
        }
    }

    pub fn translate_x(&mut self, x: f64) {
        self.transform = self.transform.trans(x, 0.0);
    }

    pub fn translate_y(&mut self, y: f64) {
        self.transform = self.transform.trans(0.0, y);
    }
}

impl GameObject for Sprite {
    fn load(&mut self, provider: &mut Provider) {
        provider.load_texture(&self.texture_name);
    }

    fn render(&mut self, provider: &mut Provider, context: &Context, gl: &mut GlGraphics) {
        provider.use_texture(&self.texture_name, |texture: &Texture| {
            let transform: Matrix2d = multiply(context.transform, self.transform);

            self.image.draw(texture, &self.draw_state, transform, gl);
        });
    }
}
