extern crate opengl_graphics;

use self::opengl_graphics::Texture;

mod game;

pub use self::game::Game;

pub struct Provider {}

impl Provider {
    pub fn load_texture(&mut self, name: &String) {}
    pub fn request_texture<RequestTexture>(&self, name: &String, use_texture: RequestTexture) where RequestTexture: FnOnce(&Texture) {

    }
}

pub trait Scene {}
pub trait Transform {}

pub trait Chunk {
    fn update(&mut self, f64) {}
    fn render(&self, &mut Scene, &Transform);
}
