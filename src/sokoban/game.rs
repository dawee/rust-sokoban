extern crate graphics;
extern crate viewport;

use graphics::clear;
use piston::input::keyboard::Key;
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use hydro::{EventListener, GameObject, Provider};
use sokoban::Character;

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
