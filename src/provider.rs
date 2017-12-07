extern crate texture;
extern crate opengl_graphics;

use opengl_graphics::{GlGraphics, OpenGL, Texture};
use texture::TextureSettings;
use std::path::Path;

pub struct Provider {
  pub graphics: GlGraphics,
  texture_settings: TextureSettings
}

impl Provider {

  pub fn new() -> Provider {
    Provider {
      graphics: GlGraphics::new(OpenGL::V3_2),
      texture_settings: TextureSettings::new()
    }
  }

  pub fn load_texture(&self, name: &str) -> Texture {
    let path_name = format!("assets/{}.png", name);
    let path = Path::new(&path_name);

    Texture::from_path(path, &self.texture_settings).unwrap()
  }

}
