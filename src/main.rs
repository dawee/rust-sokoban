extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;
extern crate viewport;
extern crate texture;

mod sokoban;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use viewport::Viewport;
use sokoban::Game;

fn main() {
    let mut events = Events::new(EventSettings::new());
    let mut window: Window = WindowSettings::new("Rust Sokoban", [200, 200])
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::load();

    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            let viewport: Viewport = render_args.viewport();

            game.render(&viewport);
        }

        if let Some(UpdateArgs {dt, ..}) = event.update_args() {
            game.update(dt);
        }
    }
}
