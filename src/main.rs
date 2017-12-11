extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;
extern crate viewport;
extern crate texture;

mod hydro;
mod sokoban;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{PressEvent, RenderEvent, UpdateArgs, UpdateEvent};
use graphics::context::Context;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use viewport::Viewport;
use hydro::{GameObject, Provider};
use sokoban::Game;

fn main() {
    let mut events = Events::new(EventSettings::new());
    let mut window: Window = WindowSettings::new("Rust Sokoban", [800, 600])
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut provider = Provider::new();
    let mut graphics = GlGraphics::new(OpenGL::V3_2);
    let mut game = Game::new();

    game.load(&mut provider);

    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            let viewport: Viewport = render_args.viewport();

            graphics.draw(viewport, |context: Context, gl: &mut GlGraphics| {
              game.render(&mut provider, &context, gl);
            });
        }

        if let Some(UpdateArgs {dt, ..}) = event.update_args() {
            game.update(dt);
        }

        if let Some(button) = event.press_args() {
            game.on_press_button(button);
        }
    }
}
