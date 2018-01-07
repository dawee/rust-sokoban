extern crate graphics;
extern crate viewport;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::math::{Matrix2d, identity, transform_pos};
use hydro::{GameObject, Provider, Sprite};
use sokoban::{Character, Level};

pub struct Block {
    row: i32,
    col: i32,
    sprite: Sprite,
    on_diamond_sprite: Sprite,
    on_diamond: bool
}

impl Block {

    pub fn new(row: i32, col: i32) -> Block {
        let sprite = Sprite::new(identity(), "Crate_Blue");
        let on_diamond_sprite = Sprite::new(identity(), "Crate_Purple");
        let on_diamond = false;

        Block {row, col, on_diamond, on_diamond_sprite, sprite}
    }

    fn is_reachable(&self, row: i32, col: i32, level: &Level) -> bool {
        return row >= 0 && row < 12
            && col >= 0 && col < 16
            && !level.contains_wall(row, col);
    }

    pub fn is_at(&self, row: i32, col: i32) -> bool {
        self.row == row && self.col == col
    }

    pub fn move_left(&mut self, character: &Character, level: &Level) {
        let new_col = self.col - 1;
        let should_move = {
            let character_pushing = character.is_at(self.row, self.col + 1);

            character_pushing && self.is_reachable(self.row, new_col, level)
        };

        self.col = if should_move {new_col} else {self.col};
        self.on_diamond = level.contains_diamond(self.row, self.col);
    }

    pub fn move_right(&mut self, character: &Character, level: &Level) {
        let new_col = self.col + 1;
        let should_move = {
            let character_pushing = character.is_at(self.row, self.col - 1);

            character_pushing && self.is_reachable(self.row, new_col, level)
        };

        self.col = if should_move {new_col} else {self.col};
        self.on_diamond = level.contains_diamond(self.row, self.col);
    }

    pub fn move_up(&mut self, character: &Character, level: &Level) {
        let new_row = self.row - 1;
        let should_move = {
            let character_pushing = character.is_at(self.row + 1, self.col);

            character_pushing && self.is_reachable(new_row, self.col, level)
        };

        self.row = if should_move {new_row} else {self.row};
        self.on_diamond = level.contains_diamond(self.row, self.col);
    }

    pub fn move_down(&mut self, character: &Character, level: &Level) {
        let new_row = self.row + 1;
        let should_move = {
            let character_pushing = character.is_at(self.row - 1, self.col);

            character_pushing && self.is_reachable(new_row, self.col, level)
        };

        self.row = if should_move {new_row} else {self.row};
        self.on_diamond = level.contains_diamond(self.row, self.col);
    }

}

impl GameObject for Block {

    fn load(&self, provider: &mut Provider) {
        self.on_diamond_sprite.load(provider);
        self.sprite.load(provider);
    }

    fn render(&self, provider: &Provider, parent_transform: &Matrix2d, gl: &mut GlGraphics) {
        let transform = parent_transform.trans(self.col as f64 * 50.0, self.row as f64 * 50.0);

        if self.on_diamond {
            self.on_diamond_sprite.render(provider, &transform, gl);
        } else {
            self.sprite.render(provider, &transform, gl);
        }
    }

}
