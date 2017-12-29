extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;

use self::opengl_graphics::Texture;
use self::piston::input::{PressEvent, RenderEvent, UpdateArgs, UpdateEvent};
use self::opengl_graphics::OpenGL::V2_1 as OPEN_GL_V2;
use hydro::{Chunk, Provider, Scene, Transform};

struct PistonScene {}
struct PistonTransform {}

impl Scene for PistonScene {}
impl Transform for PistonTransform {}

pub struct Game {
    events: piston::event_loop::Events,
    gl: opengl_graphics::GlGraphics,
    gl_window: glutin_window::GlutinWindow,
    provider: Provider,
    scene: PistonScene,
    transform: PistonTransform
}

impl Game {
    pub fn new(title: &str, size: [u32; 2]) -> Game {
        let events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
        let window_settings = piston::window::WindowSettings::new(title, size);
        let gl_window = window_settings.opengl(OPEN_GL_V2).exit_on_esc(true).build().unwrap();
        let gl = opengl_graphics::GlGraphics::new(OPEN_GL_V2);
        let provider = Provider {};
        let scene = PistonScene {};
        let transform = PistonTransform {};

        Game {events, gl, gl_window, provider, scene, transform}
    }

    pub fn run(&mut self, root: &mut Chunk) {
        while let Some(event) = self.events.next(&mut self.gl_window) {
            if let Some(UpdateArgs {dt, ..}) = event.update_args() {
                root.update(dt);
            }

            if let Some(render_args) = event.render_args() {
                root.render(&mut self.scene, &self.transform);
            }
        }
    }
}
