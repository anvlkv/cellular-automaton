extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate glutin;
extern crate conv;

use glutin_window::{GlutinWindow};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use conv::{ApproxFrom, RoundToNearest};

mod world;
use world::World;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    world: World,
    cell_size: usize
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, self.cell_size as f64);
        // let rotation = self.rotation;
        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);


        let cells_by_row = self.world.cells();
        let cell_size = self.cell_size;
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);
            let mut cells_by_row_iter = cells_by_row.iter().enumerate();
    
    
            while let Some((r_index, row)) = cells_by_row_iter.next() {
                let mut cells_iter = row.iter().enumerate();
                let y = r_index * cell_size;
    
                while let Some((c_index, cell)) = cells_iter.next() {
                    let (r, g, b) = cell.clone();
                    let x = c_index * cell_size;
                    let transform = c
                        .transform
                        .trans(x as f64, y as f64);
                        // .scale(0.9, 0.9);

                    let rect = Rectangle::new([r, g, b, 1.0]);

                    rect.draw(square, &c.draw_state, transform, gl);
                }
            }
        });

    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
        self.world.next();
    }
}

pub fn get_cell_size(a: usize, b: usize) -> usize {
    let lesser = {
        if a > b {
            a
        }
        else {
            b
        }
    };

    let mut result = Vec::new();
    for i in 1..lesser {
        if a % i == 0 && b % 1 == 0 {
            result.push(i);
        }
    }

    *result.iter().filter(|n| n > &&15_usize).next().unwrap_or(&15_usize)
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("cellulose", [0, 0])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .fullscreen(true)
        .build()
        .unwrap();

    let monitor_id = window.ctx.window().get_current_monitor();
    let screen_size = monitor_id.get_dimensions();

    let width = ApproxFrom::<f64, RoundToNearest>::approx_from(screen_size.width).unwrap();
    let height = ApproxFrom::<f64, RoundToNearest>::approx_from(screen_size.height).unwrap();
    let cell_size = get_cell_size(width, height);

    let world = World::new((width/cell_size) as usize, (height/cell_size) as usize);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        world,
        cell_size
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // app.world.event(&e);

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}