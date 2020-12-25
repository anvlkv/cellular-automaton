use crate::world_controller::WorldController;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
// use piston::input::{Input, ResizeArgs};
// use piston::{Event};
// use piston::AdvancedWindow;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    window: GlutinWindow,
    world_controller: WorldController
}

const OPEN_GL: OpenGL = OpenGL::V3_2;

impl App {
    fn new() -> Self {
        let window: GlutinWindow = WindowSettings::new("cellulose", [0, 0])
            .graphics_api(OPEN_GL)
            .exit_on_esc(true)
            .fullscreen(true)
            .build()
            .unwrap();

        let world_controller = WorldController::new();

        Self {
            gl: GlGraphics::new(OPEN_GL),
            window,
            world_controller
        }
    }

    

    pub fn start() {
        let mut app = Self::new();
        
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut app.window) {
            app.world_controller.handle_event(&e, &mut app.gl);
        }
    }


}
