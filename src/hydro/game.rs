extern crate piston;

use self::piston::input::{PressEvent, RenderEvent, UpdateArgs, UpdateEvent};

use hydro::Window;

pub struct Game {
    window: Window
}

impl Game {

    pub fn new(title: &str, width: u32, height: u32) -> Game {
        let window = Window::new(title, width, height);

        Game {window}
    }

    pub fn run(&mut self) {
        let settings = piston::event_loop::EventSettings::new();
        let mut events = piston::event_loop::Events::new(settings);

        self.window.request_gl(|gl, gl_window| {
            while let Some(event) = events.next(gl_window) {
                if let Some(render_args) = event.render_args() {

                }

                if let Some(UpdateArgs {dt, ..}) = event.update_args() {

                }

                if let Some(button) = event.press_args() {

                }
            }
        });

    }

}
