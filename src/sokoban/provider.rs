extern crate texture;
extern crate opengl_graphics;

use opengl_graphics::Texture;
use texture::TextureSettings;
use std::path::Path;

pub struct Provider {
    texture_settings: TextureSettings
}

impl Provider {

    pub fn new() -> Provider {
        Provider {texture_settings: TextureSettings::new()}
    }

    pub fn load_texture(&self, name: &str) {
        let path_name = format!("assets/{}.png", name);
        let path = Path::new(&path_name);
        let texture = Texture::from_path(path, &self.texture_settings).unwrap();
    }

}
