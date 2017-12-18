extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;

use self::opengl_graphics::OpenGL::V2_1 as OPEN_GL_V2;

pub struct Window {
    gl: opengl_graphics::GlGraphics,
    gl_window: glutin_window::GlutinWindow
}

impl Window {

    pub fn new(title: &str, width: u32, height: u32) -> Window {
        let settings = piston::window::WindowSettings::new(title, [width, height]);
        let gl_window = settings.opengl(OPEN_GL_V2).exit_on_esc(true).build().unwrap();
        let gl = opengl_graphics::GlGraphics::new(OPEN_GL_V2);

        Window {gl, gl_window}
    }

    pub fn request_gl<RequestGL>(
        &mut self,
        request_gl: RequestGL
    ) where RequestGL: FnOnce(&mut opengl_graphics::GlGraphics, &mut glutin_window::GlutinWindow) {
        request_gl(&mut self.gl, &mut self.gl_window);
    }

}
