extern crate conv;
extern crate glutin;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod app;
pub mod world;

use app::App;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    App::start();
}
