extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::math::{Matrix2d, identity, transform_pos};
use hydro::{GameObject, Provider, Sprite};
use sokoban::Level;

enum Posture {Up, Right, Down, Left}

pub struct Character {
    stand_up_sprite: Sprite,
    stand_right_sprite: Sprite,
    stand_down_sprite: Sprite,
    stand_left_sprite: Sprite,
    posture: Posture,
    row: i32,
    col: i32
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
            row: 0,
            col: 0,
            stand_up_sprite: Sprite::new(identity(), "Character7"),
            stand_right_sprite: Sprite::new(identity(), "Character2"),
            stand_down_sprite: Sprite::new(identity(), "Character4"),
            stand_left_sprite: Sprite::new(identity(), "Character1")
        }
    }

    fn is_reachable(&self, row: i32, col: i32, level: &Level) -> bool {
        return row >= 0 && row < 12
            && col >= 0 && col < 16
            && !level.is_wall(row as u32, col as u32);
    }

    pub fn move_up(&mut self, level: &Level) {
        let new_row = self.row - 1;

        self.row = if self.is_reachable(new_row, self.col, level) {new_row} else {self.row};
        self.posture = Posture::Up;
    }

    pub fn move_right(&mut self, level: &Level) {
        let new_col = self.col + 1;

        self.col = if self.is_reachable(self.row, new_col, level) {new_col} else {self.col};
        self.posture = Posture::Right;
    }

    pub fn move_down(&mut self, level: &Level) {
        let new_row = self.row + 1;

        self.row = if self.is_reachable(new_row, self.col, level) {new_row} else {self.row};
        self.posture = Posture::Down;
    }

    pub fn move_left(&mut self, level: &Level) {
        let new_col = self.col - 1;

        self.col = if self.is_reachable(self.row, new_col, level) {new_col} else {self.col};
        self.posture = Posture::Left;
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
        let transform = parent_transform.trans(
            self.col as f64 * 50.0,
            self.row as f64 * 50.0
        );

        self.use_sprite(|sprite| sprite.render(provider, &transform, gl));
    }

}
