extern crate piston;
extern crate graphics;
extern crate texture;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use graphics::{Image, Transformed, clear};
use graphics::rectangle::square;
use graphics::draw_state::DrawState;
use texture::TextureSettings;
use std::path::Path;
use piston::input::*;

struct Provider {
  graphics: GlGraphics,
  texture_settings: TextureSettings
}

struct Character {
  x: f64,
  y: f64,
  image: Image,
  texture: Texture,
  draw_state: DrawState
}

struct Game {
  provider: Provider,
  character: Character
}

impl Provider {

  fn new() -> Provider {
    Provider {
      graphics: GlGraphics::new(OpenGL::V3_2),
      texture_settings: TextureSettings::new()
    }
  }

  fn load_texture(&mut self, name: &str) -> Texture {
    let path_name = format!("assets/{}.png", name);
    let path = Path::new(&path_name);

    Texture::from_path(path, &self.texture_settings).unwrap()
  }

}

impl Character {

  fn load(provider: &mut Provider) -> Character {
    Character {
      x: 0.0,
      y: 0.0,
      draw_state: DrawState::new_alpha(),
      image: Image::new().rect(square(0.0, 0.0, 50.0)),
      texture: provider.load_texture("Character1")
    }
  }

  fn update(&mut self, dt: f64) {
    self.x += 5.0 * dt;
  }

  fn render(&self, args: &RenderArgs, provider: &mut Provider) {
    provider.graphics.draw(args.viewport(), |context, gl| {
      self.image.draw(
        &self.texture,
        &self.draw_state,
        context.transform.trans(self.x, self.y),
        gl
      );
    });

  }

}

impl Game {

  fn load() -> Game {
    let mut provider = Provider::new();
    let character = Character::load(&mut provider);

    Game {provider, character}
  }

  fn update(&mut self, args: &UpdateArgs) {
    let UpdateArgs {dt, ..} = *args;

    self.character.update(dt);
  }

  fn clear(&mut self, args: &RenderArgs) {
    self.provider.graphics.draw(args.viewport(), |_, gl| {
      clear([0.0, 0.0, 0.0, 1.0], gl);
    });
  }

  fn render(&mut self, args: &RenderArgs) {
    self.clear(args);
    self.character.render(args, &mut self.provider);
  }

}


fn main() {
  let opengl = OpenGL::V3_2;
  let mut events = Events::new(EventSettings::new());
  let mut window: Window = WindowSettings::new("Rust Sokoban", [200, 200])
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  let mut game = Game::load();

  while let Some(event) = events.next(&mut window) {
    if let Some(render_args) = event.render_args() {
      game.render(&render_args);
    }

    if let Some(update_args) = event.update_args() {
      game.update(&update_args);
    }
  }
}
