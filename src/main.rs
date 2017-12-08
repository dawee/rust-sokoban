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
use graphics::context::Context;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use viewport::Viewport;
use sokoban::Game;

fn main() {
    let mut events = Events::new(EventSettings::new());
    let mut window: Window = WindowSettings::new("Rust Sokoban", [200, 200])
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut graphics = GlGraphics::new(OpenGL::V3_2);
    let mut game = Game::load();

    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            let viewport: Viewport = render_args.viewport();

            graphics.draw(viewport, |context: Context, gl: &mut GlGraphics| {
              game.render(&context, gl);
            })
        }

        if let Some(UpdateArgs {dt, ..}) = event.update_args() {
            game.update(dt);
        }
    }
}
