extern crate piston;
extern crate graphics;
extern crate viewport;

mod provider;
mod sprite;

pub use self::provider::Provider;
pub use self::sprite::Sprite;

use piston::input::Button;
use piston::input::keyboard::Key;
use graphics::context::Context;
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
