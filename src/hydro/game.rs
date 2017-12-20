extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;

use self::piston::input::{PressEvent, RenderEvent, UpdateArgs, UpdateEvent};
use self::opengl_graphics::OpenGL::V2_1 as OPEN_GL_V2;

pub struct Game {
    events: piston::event_loop::Events,
    gl: opengl_graphics::GlGraphics,
    gl_window: glutin_window::GlutinWindow
}

impl Game {

    pub fn new(title: &str, width: u32, height: u32) -> Game {
        let events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
        let window_settings = piston::window::WindowSettings::new(title, [width, height]);
        let gl_window = window_settings.opengl(OPEN_GL_V2).exit_on_esc(true).build().unwrap();
        let gl = opengl_graphics::GlGraphics::new(OPEN_GL_V2);

        Game {events, gl, gl_window}
    }

    pub fn run(&mut self) {
        while let Some(event) = self.events.next(&mut self.gl_window) {
            if let Some(UpdateArgs {dt, ..}) = event.update_args() {
            }

            if let Some(render_args) = event.render_args() {
            }
        }
    }

}
