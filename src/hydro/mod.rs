extern crate opengl_graphics;

use self::opengl_graphics::Texture;

/*
 * Hydro
 */

mod game;

pub use self::game::Game;

pub trait Provider {
    fn load_texture(&mut self, name: &String);
    fn request_texture<RequestTexture>(
        &self,
        &String,
        RequestTexture
    ) where RequestTexture: FnOnce(&Texture);
}
