extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::context::Context;
use graphics::math::{identity, transform_pos};
use hydro::{GameObject, Provider, Sprite};

pub struct Character {
    sprite: Sprite
}

impl Character {

    pub fn new(position: (f64, f64)) -> Character {
        let (x, y) = position;
        let sprite = Sprite::new(identity().trans(x, y), "Character1");

        Character {sprite}
    }

    pub fn move_up(&mut self) {
        self.sprite.transform(|transform| {
            let moved = transform.trans(0.0, -50.0);
            let pos = transform_pos(moved.clone(), [0.0, 0.0]);

            return if pos[1] >= 0.0 { Some(moved) } else { None };
        });
    }

    pub fn move_right(&mut self) {
        self.sprite.transform(|transform| {
            let moved = transform.trans(50.0, 0.0);
            let pos = transform_pos(moved.clone(), [0.0, 0.0]);

            return if pos[0] < 800.0 { Some(moved) } else { None };
        });
    }

    pub fn move_down(&mut self) {
        self.sprite.transform(|transform| {
            let moved = transform.trans(0.0, 50.0);
            let pos = transform_pos(moved.clone(), [0.0, 0.0]);

            return if pos[1] < 600.0 { Some(moved) } else { None };
        });
    }

    pub fn move_left(&mut self) {
        self.sprite.transform(|transform| {
            let moved = transform.trans(-50.0, 0.0);
            let pos = transform_pos(moved.clone(), [0.0, 0.0]);

            return if pos[0] >= 0.0 { Some(moved) } else { None };
        });
    }

}

impl GameObject for Character {

    fn load(&self, provider: &mut Provider) {
        self.sprite.load(provider);
    }

    fn render(&self, provider: &Provider, context: &Context, gl: &mut GlGraphics) {
        self.sprite.render(provider, context, gl);
    }

}
