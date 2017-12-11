extern crate texture;
extern crate opengl_graphics;

use opengl_graphics::Texture;
use texture::TextureSettings;
use std::path::Path;
use std::collections::HashMap;
use std::clone::Clone;

pub struct Provider {
    texture_settings: TextureSettings,
    textures: HashMap<String, Texture>
}

impl Provider {

    pub fn new() -> Provider {
        let texture_settings = TextureSettings::new();
        let textures = HashMap::new();

        Provider {texture_settings, textures}
    }

    pub fn load_texture(&mut self, name: &String) {
        if !self.textures.contains_key(name) {
            let path_name = format!("assets/{}.png", name);
            let path = Path::new(&path_name);
            let texture = Texture::from_path(path, &self.texture_settings).unwrap();

            self.textures.insert(name.clone(), texture);
        }
    }

    pub fn use_texture<F>(&self, name: &String, use_texture: F) where F: FnOnce(&Texture) {
        if let Some(texture) = self.textures.get(name) {
            use_texture(texture);
        } else {
            println!("Try to use unloaded texture '{}'", name);
        }
    }

}
