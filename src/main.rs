extern crate piston;
extern crate graphics;
extern crate texture;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture};
use graphics::{Image, clear};
use graphics::rectangle::square;
use graphics::draw_state::DrawState;
use texture::{TextureSettings};
use std::path::Path;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64   // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y).rot_rad(rotation).trans(-25.0, -25.0);

            clear(GREEN, gl);
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let UpdateArgs {dt, ..} = *args;

        self.rotation += 2.0 * dt;
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

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    let image = Image::new().rect(square(0.0, 0.0, 200.0));
    let texture = Texture::from_path(Path::new("assets/Character1.png"), &TextureSettings::new()).unwrap();

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);

            app.gl.draw(r.viewport(), |c, gl| {
                image.draw(&texture, &DrawState::new_alpha(), c.transform, gl);
            });
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
