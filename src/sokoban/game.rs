extern crate graphics;
extern crate viewport;

use graphics::clear;
use piston::input::Button;
use piston::input::keyboard::Key;
use graphics::context::Context;
use sokoban::{Character, Provider};
use opengl_graphics::GlGraphics;

pub trait GameObject {
    fn load(&mut self, &mut Provider) {}
    fn update(&mut self, f64) {}
    fn render(&mut self, &mut Provider, &Context, &mut GlGraphics) {}
}

pub trait EventListener {
    fn on_press_button(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) => self.on_press_key(key),
            _ => println!("unmanaged event")
        };
    }

    fn on_release_button(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) => self.on_release_key(key),
            _ => println!("unmanaged event")
        };
    }

    fn on_press_key(&mut self, Key) {}
    fn on_release_key(&mut self, Key) {}
}

pub struct Game {
    character: Character
}

impl Game {

    pub fn new() -> Game {
        let character = Character::new((0.0, 0.0));

        Game {character}
    }

}

impl GameObject for Game {

    fn load(&mut self, provider: &mut Provider) {
        self.character.load(provider);
    }

    fn render(&mut self, provider: &mut Provider, context: &Context, gl: &mut GlGraphics) {
      clear([0.0, 0.0, 0.0, 1.0], gl);
      self.character.render(provider, context, gl);
    }

}

impl EventListener for Game {

    fn on_press_key(&mut self, key: Key) {
        match key {
            Key::Up => self.character.move_up(),
            Key::Right => self.character.move_right(),
            Key::Down => self.character.move_down(),
            Key::Left => self.character.move_left(),
            _ => println!("press key")
        };
    }

}
