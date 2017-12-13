extern crate graphics;
extern crate viewport;

use graphics::clear;
use piston::input::Button;
use piston::input::keyboard::Key;
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;
use hydro::{GameObject, Provider};
use sokoban::{Character, Ground};

pub struct Game {
    character: Character,
    grounds: Vec<Ground>
}

impl Game {

    pub fn new() -> Game {
        let character = Character::new((0.0, 0.0));
        let (rows, cols) = (12, 16);
        let grounds = (0..(rows * cols)).map(|n| {
            let row = (n as f64 / cols as f64).floor();
            let col = n as f64 - row * cols as f64;
            let position = (col * 50.0, row * 50.0);

            Ground::new(position)
        }).collect();

        Game {character, grounds}
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
        self.grounds.iter().for_each(|ground| ground.load(provider));
        self.character.load(provider);
    }

    fn render(&self, provider: &Provider, parent_transform: &Matrix2d, gl: &mut GlGraphics) {
      clear([0.0, 0.0, 0.0, 1.0], gl);
      self.grounds.iter().for_each(|ground| ground.render(provider, parent_transform, gl));
      self.character.render(provider, parent_transform, gl);
    }

}
