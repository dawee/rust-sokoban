extern crate graphics;
extern crate viewport;

use graphics::clear;
use piston::input::Button;
use piston::input::keyboard::Key;
use graphics::Transformed;
use graphics::math::{Matrix2d, identity};
use opengl_graphics::GlGraphics;
use hydro::{GameObject, Provider, Sprite};
use sokoban::{Character, Level};

pub struct Game {
    character: Character,
    level: Level,
    tiles: Vec<Sprite>
}

impl Game {

    pub fn new() -> Game {
        let character = Character::new((50.0, 50.0));
        let (rows, cols) = (12, 16);
        let level = Level::new();
        let mut tiles: Vec<Sprite> = (0..(rows * cols)).map(|n| {
            let row = (n as f64 / cols as f64).floor();
            let col = n as f64 - row * cols as f64;

            Sprite::new(identity().trans(col * 50.0, row * 50.0), "GroundGravel_Concrete")
        }).collect();

        level.each_wall(|row, col| {
            tiles.push(Sprite::new(identity().trans(col as f64 * 50.0, row as f64 * 50.0), "Wall_Black"));
        });

        Game {character, level, tiles}
    }

    pub fn on_press_button(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) => self.on_press_key(key),
            _ => println!("unmanaged event")
        };
    }

    pub fn on_press_key(&mut self, key: Key) {
        match key {
            Key::Up => self.character.move_up(),
            Key::Right => self.character.move_right(),
            Key::Down => self.character.move_down(),
            Key::Left => self.character.move_left(),
            _ => println!("press key")
        };
    }

}

impl GameObject for Game {

    fn load(&self, provider: &mut Provider) {
        self.character.load(provider);
        self.tiles.iter().for_each(|ground| ground.load(provider));
    }

    fn render(&self, provider: &Provider, parent_transform: &Matrix2d, gl: &mut GlGraphics) {
      clear([0.0, 0.0, 0.0, 1.0], gl);
      self.tiles.iter().for_each(|ground| ground.render(provider, parent_transform, gl));
      self.character.render(provider, parent_transform, gl);
    }

}
