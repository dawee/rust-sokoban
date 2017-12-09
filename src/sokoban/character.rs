extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::{Texture, GlGraphics};
use graphics::{Image, Transformed};
use graphics::draw_state::DrawState;
use graphics::rectangle::square;
use graphics::context::Context;
use graphics::math::{Matrix2d, identity, multiply};
use sokoban::{GameObject, Provider};

enum Direction {
    Nope,
    Up,
    Right,
    Down,
    Left
}

pub trait Movable {
    fn move_up(&mut self);
    fn move_right(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn stop(&mut self);
}

pub struct Character {
    direction: Direction,
    image: Image,
    texture: Texture,
    draw_state: DrawState,
    transform: Matrix2d
}

impl GameObject for Character {

    fn load(provider: &Provider) -> Character {
        Character {
            direction: Direction::Nope,
            draw_state: DrawState::new_alpha(),
            image: Image::new().rect(square(0.0, 0.0, 50.0)),
            texture: provider.load_texture("Character1"),
            transform: identity()
        }
    }

    fn update(&mut self, dt: f64) {
        self.transform = match self.direction {
            Direction::Up => self.transform.trans(0.0, -100.0 * dt),
            Direction::Right => self.transform.trans(100.0 * dt, 0.0),
            Direction::Down => self.transform.trans(0.0, 100.0 * dt),
            Direction::Left => self.transform.trans(-100.0 * dt, 0.0),
            Direction::Nope => self.transform
        };
    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
      let transform: Matrix2d = multiply(context.transform, self.transform);

      self.image.draw(&self.texture, &self.draw_state, transform, gl);
    }

}

impl Movable for Character {

    fn move_up(&mut self) {
        self.direction = Direction::Up;
    }

    fn move_right(&mut self) {
        self.direction = Direction::Right;
    }

    fn move_down(&mut self) {
        self.direction = Direction::Down;
    }

    fn move_left(&mut self) {
        self.direction = Direction::Left;
    }

    fn stop(&mut self) {
        self.direction = Direction::Nope;
    }

}
