extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::math::{Matrix2d, identity, transform_pos};
use std::collections::HashMap;
use hydro::{GameObject, Provider, Sprite};

enum Posture {Up, Right, Down, Left}

pub struct Character {
    stand_up_sprite: Sprite,
    stand_right_sprite: Sprite,
    stand_down_sprite: Sprite,
    stand_left_sprite: Sprite,
    posture: Posture,
    transform: Matrix2d
}

impl Character {

    fn use_sprite<F>(&self, use_sprite: F) where F: FnOnce(&Sprite) {
        match self.posture {
            Posture::Up => use_sprite(&self.stand_up_sprite),
            Posture::Right => use_sprite(&self.stand_right_sprite),
            Posture::Down => use_sprite(&self.stand_down_sprite),
            Posture::Left => use_sprite(&self.stand_left_sprite)
        };
    }

    pub fn new(position: (f64, f64)) -> Character {
        let (x, y) = position;

        Character {
            posture: Posture::Right,
            transform: identity().trans(x, y),
            stand_up_sprite: Sprite::new(identity(), "Character7"),
            stand_right_sprite: Sprite::new(identity(), "Character2"),
            stand_down_sprite: Sprite::new(identity(), "Character4"),
            stand_left_sprite: Sprite::new(identity(), "Character1")
        }
    }

    pub fn move_up(&mut self) {
        let moved = self.transform.trans(0.0, -50.0);
        let pos = transform_pos(moved.clone(), [0.0, 0.0]);

        self.posture = Posture::Up;
        self.transform = if pos[1] >= 0.0 { moved } else { self.transform };
    }

    pub fn move_right(&mut self) {
        let moved = self.transform.trans(50.0, 0.0);
        let pos = transform_pos(moved.clone(), [0.0, 0.0]);

        self.posture = Posture::Right;
        self.transform = if pos[0] < 800.0 { moved } else { self.transform };
    }

    pub fn move_down(&mut self) {
        let moved = self.transform.trans(0.0, 50.0);
        let pos = transform_pos(moved.clone(), [0.0, 0.0]);

        self.posture = Posture::Down;
        self.transform = if pos[1] < 600.0 { moved } else { self.transform };
    }

    pub fn move_left(&mut self) {
        let moved = self.transform.trans(-50.0, 0.0);
        let pos = transform_pos(moved.clone(), [0.0, 0.0]);

        self.posture = Posture::Left;
        self.transform = if pos[0] >= 0.0 { moved } else { self.transform };
    }

}

impl GameObject for Character {

    fn load(&self, provider: &mut Provider) {
        self.stand_up_sprite.load(provider);
        self.stand_right_sprite.load(provider);
        self.stand_down_sprite.load(provider);
        self.stand_left_sprite.load(provider);
    }

    fn render(&self, provider: &Provider, parent_transform: &Matrix2d, gl: &mut GlGraphics) {
        let transform = self.transform.prepend_transform(*parent_transform);

        self.use_sprite(|sprite| sprite.render(provider, &transform, gl));
    }

}
