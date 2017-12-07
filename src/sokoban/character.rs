extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::{Texture};
use graphics::{Image, Transformed};
use graphics::draw_state::DrawState;
use graphics::rectangle::square;
use viewport::Viewport;
use sokoban::Provider;

pub struct Character {
    x: f64,
    y: f64,
    image: Image,
    texture: Texture,
    draw_state: DrawState
}

impl Character {

    pub fn load(provider: &Provider) -> Character {
        Character {
            x: 0.0,
            y: 0.0,
            draw_state: DrawState::new_alpha(),
            image: Image::new().rect(square(0.0, 0.0, 50.0)),
            texture: provider.load_texture("Character1")
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.x += 5.0 * dt;
    }

    pub fn render(&self, viewport: &Viewport, provider: &mut Provider) {
        provider.graphics.draw(*viewport, |context, gl| {
            self.image.draw(
                &self.texture,
                &self.draw_state,
                context.transform.trans(self.x, self.y),
                gl
            );
        });
    }

}
