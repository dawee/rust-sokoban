extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::{Texture, GlGraphics};
use graphics::Image;
use graphics::draw_state::DrawState;
use graphics::rectangle::square;
use graphics::context::Context;
use graphics::math::Matrix2d;
use sokoban::{Provider, GameObject};

pub struct Sprite {
    draw_state: DrawState,
    image: Image,
    transform: Matrix2d
}

impl Sprite {
    pub fn new(transform: Matrix2d) -> Sprite {
        Sprite {
            transform,
            draw_state: DrawState::new_alpha(),
            image: Image::new().rect(square(0.0, 0.0, 50.0)),
        }
    }
}

impl GameObject for Sprite {
    fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
    }
}
