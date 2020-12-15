use super::world::{Cell, World};
use conv::{ApproxFrom, RoundToNearest};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, 
    ButtonArgs, 
    Input, 
    Motion, 
    MouseButton,
    RenderArgs, 
    RenderEvent, 
    ResizeArgs, 
    UpdateArgs, 
    UpdateEvent,
    Key,
};
use piston::window::WindowSettings;
use piston::Event;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    world: World,
    cell_size: f64,
    window: GlutinWindow,
    cursor: Option<Cell>
}

impl App {
    fn size_world(window: &GlutinWindow) -> (usize, usize, f64) {
        let monitor_id = window.ctx.window().get_current_monitor();
        let screen_size = monitor_id.get_dimensions();

        let width = ApproxFrom::<f64, RoundToNearest>::approx_from(screen_size.width).unwrap();
        let height = ApproxFrom::<f64, RoundToNearest>::approx_from(screen_size.height).unwrap();
        let cell_size = App::get_cell_size(width, height);

        (
            (width / cell_size) as usize,
            (height / cell_size) as usize,
            cell_size,
        )
    }

    fn new() -> Self {
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let window: GlutinWindow = WindowSettings::new("cellulose", [0, 0])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .fullscreen(true)
            .build()
            .unwrap();

        let (width, height, cell_size) = App::size_world(&window);
        let world = World::new(width, height, cell_size);

        Self {
            gl: GlGraphics::new(opengl),
            window,
            world,
            cell_size,
            cursor: None
        }
    }

    fn set_cursor(&mut self, [x, y]: [f64;2]) {
        // self.world.find_cell_for_position(position)

        let cell_x = (x / self.cell_size) as usize;
        let cell_y = (y / self.cell_size) as usize;

        let cell = self.world.cell_at(cell_x, cell_y);

        self.cursor = Some(cell);

    }

    fn handle_event(&mut self, e: &Event) {
        match e {
            Event::Loop(lp) => {
                // println!("Loop {:?}", lp)
            }
            Event::Input(input, _ts) => {
                // println!("Iput {:?}, {:?}", i, ts)
                match input {
                    Input::Resize(ResizeArgs {
                        window_size,
                        draw_size: _,
                    }) => {
                        if self.cell_size != App::get_cell_size(window_size[0], window_size[1]) {
                            let (width, height, cell_size) = App::size_world(&self.window);
                            self.world = World::new(width, height, cell_size);
                            self.cell_size = cell_size;
                        }
                    }
                    Input::Move(motion) => match motion {
                        Motion::MouseCursor(position) => {
                            self.set_cursor(*position);
                        }
                        Motion::MouseScroll([x, y]) => {}
                        _ => {}
                    },
                    Input::Button(ButtonArgs {
                        state,
                        button,
                        scancode: _,
                    }) => {
                        // println!("s: {:?}", state);

                        match button {
                            Button::Mouse(b) => {
                                match b {
                                    MouseButton::Left => {
                                    },
                                    MouseButton::Right => {
    
                                    },
                                    _ => {}
                                }
                            },
                            Button::Keyboard(k) => {
                                match k {
                                    Key::Right => {
    
                                    },
                                    Key::Left => {
    
                                    },
                                    Key::Up => {
    
                                    },
                                    Key::Down => {
    
                                    },
                                    Key::Space => {
    
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
            Event::Custom(eid, _arc, ts) => {
                // println!("Custom {:?}, {:?}", eid, ts)
            }
        }

        if let Some(args) = e.render_args() {
            self.render(&args);
        }

        if let Some(args) = e.update_args() {
            self.update(&args);
        }
    }

    pub fn start() {
        let mut app = Self::new();

        let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut app.window) {
            app.handle_event(&e);
        }
    }

    fn get_cell_size(a: f64, b: f64) -> f64 {
        let lesser = {
            if a > b {
                a
            } else {
                b
            }
        };

        let mut result = Vec::new();
        for i in 1..(lesser as usize) {
            if a % (i as f64) == 0.0 && b % 1.0 == 0.0 {
                result.push(i as f64);
            }
        }

        *result
            .iter()
            .filter(|n| n > &&15.0_f64)
            .next()
            .unwrap_or(&15.0_f64)
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, self.cell_size as f64);
        let mut cells = self.world.cells_iter();
        let cursor = self.cursor.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            while let Some(Cell { color, top_left }) = cells.next() {
                let transform = c.transform.trans(top_left[0], top_left[1]).scale(0.9, 0.9);
                let rect = Rectangle::new(color);
                rect.draw(square, &c.draw_state, transform, gl)
            }

            if let Some(Cell { color, top_left }) = cursor {
                let transform = c.transform.trans(top_left[0], top_left[1]).scale(0.9, 0.9);
                let rect = Rectangle::new(WHITE);
                rect.draw(square, &c.draw_state, transform, gl)
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
        self.world.next();
    }
}
