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
use graphics::Transformed;
use texture::{TextureSettings};
use std::path::Path;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut events = Events::new(EventSettings::new());
    let mut window: Window = WindowSettings::new("Rust Sokoban", [200, 200])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut x = 0.0;
    let mut graphics = GlGraphics::new(opengl);
    let draw_state = DrawState::new_alpha();
    let image = Image::new().rect(square(0.0, 0.0, 50.0));
    let texture = Texture::from_path(
        Path::new("assets/Character1.png"),
        &TextureSettings::new()
    ).unwrap();

    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            graphics.draw(render_args.viewport(), |context, gl| {
                clear([0.0, 0.0, 0.0, 1.0], gl);
                image.draw(&texture, &draw_state, context.transform.trans(x, 0.0), gl);
            });
        }

        if let Some(update_args) = event.update_args() {
            let UpdateArgs {dt, ..} = update_args;

            x = x + 1.0 * dt;
        }
    }
}
