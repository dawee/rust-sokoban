extern crate graphics;
extern crate viewport;

use graphics::clear;
use piston::input::Button;
use piston::input::keyboard::Key;
use graphics::Transformed;
use graphics::math::{Matrix2d, identity};
use opengl_graphics::GlGraphics;
use hydro::{GameObject, Provider, Sprite};
use sokoban::{Block, Character, Level};

pub struct Game {
    blocks: Vec<Block>,
    character: Character,
    level: Level,
    tiles: Vec<Sprite>
}

impl Game {

    pub fn new() -> Game {
        let character = Character::new(6, 8);
        let (rows, cols) = (12, 16);
        let level = Level::new();
        let mut blocks = Vec::new();
        let mut tiles = Vec::new();

        level.each_ground(|row, col| {
            tiles.push(Sprite::new(identity().trans(col as f64 * 50.0, row as f64 * 50.0), "GroundGravel_Concrete"));
        });

        level.each_wall(|row, col| {
            tiles.push(Sprite::new(identity().trans(col as f64 * 50.0, row as f64 * 50.0), "Wall_Black"));
        });

        level.each_block(|row, col| {
            blocks.push(Block::new(row as i32, col as i32));
        });

        Game {blocks, character, level, tiles}
    }

    pub fn on_press_button(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) => self.on_press_key(key),
            _ => ()
        };
    }

    pub fn on_press_key(&mut self, key: Key) {
        match key {
            Key::Up => {
                for block in self.blocks.iter_mut() {
                    block.move_up(&self.character, &self.level);
                }

                self.character.move_up(&self.blocks, &self.level);
            }
            Key::Right => {
                for block in self.blocks.iter_mut() {
                    block.move_right(&self.character, &self.level);
                }

                self.character.move_right(&self.blocks, &self.level);
            }
            Key::Down => {
                for block in self.blocks.iter_mut() {
                    block.move_down(&self.character, &self.level);
                }

                self.character.move_down(&self.blocks, &self.level);
            }
            Key::Left => {
                for block in self.blocks.iter_mut() {
                    block.move_left(&self.character, &self.level);
                }

                self.character.move_left(&self.blocks, &self.level);
            },
            _ => ()
        };
    }

}

impl GameObject for Game {

    fn load(&self, provider: &mut Provider) {
        self.character.load(provider);
        self.tiles.iter().for_each(|ground| ground.load(provider));
        self.blocks.iter().for_each(|block| block.load(provider));
    }

    fn render(&self, provider: &Provider, parent_transform: &Matrix2d, gl: &mut GlGraphics) {
      clear([0.0, 0.0, 0.0, 1.0], gl);
      self.tiles.iter().for_each(|tile| tile.render(provider, parent_transform, gl));
      self.blocks.iter().for_each(|block| block.render(provider, parent_transform, gl));
      self.character.render(provider, parent_transform, gl);
    }

}
