use super::world::{Cell, World};
use conv::{ApproxInto, RoundToNearest};
use glutin_window::GlutinWindow;
use graphics::color::{BLACK, WHITE};
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use palette::{Gradient, Hsv, LinSrgba};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, ButtonArgs, Input, Key, Motion, MouseButton, RenderArgs, RenderEvent, ResizeArgs,
    UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;
use piston::{ButtonState, Event};
use std::vec::IntoIter;

fn cursor_colors_iter() -> IntoIter<Color> {
    let cursor_gradient: Gradient<Hsv> = Gradient::new(vec![
        Hsv::new(0.0, 1.0, 0.5),
        Hsv::new(90.0, 1.0, 0.5),
        Hsv::new(180.0, 1.0, 0.5),
        Hsv::new(270.0, 1.0, 0.5),
        Hsv::new(360.0, 1.0, 0.5),
        Hsv::new(270.0, 1.0, 0.5),
        Hsv::new(180.0, 1.0, 0.5),
        Hsv::new(90.0, 1.0, 0.5),
        Hsv::new(0.0, 1.0, 0.5),
    ]);

    let (start, mut end) = cursor_gradient.domain();
    let mut colors_vec: Vec<Color> = Vec::new();

    // println!("s: {}, e: {}", start, end);

    while end > start {
        let color = LinSrgba::from(cursor_gradient.get(end));
        let (r, g, b, a) = color.into_components();

        // println!("rgba, {}, {}, {}, {}", r, g, b, a);

        colors_vec.push([r, g, b, a]);

        end -= 0.005;
    }

    colors_vec.into_iter()
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    world: World,
    cell_size: f64,
    window: GlutinWindow,
    cursor: Option<Cell>,
    cursor_colors_iter: IntoIter<Color>,
    cursor_paints: bool,
}

impl App {
    fn size_world(width: f64, height: f64) -> (usize, usize, f64) {
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

        let world = World::new(0, 0, 0.0);

        Self {
            gl: GlGraphics::new(opengl),
            window,
            world,
            cell_size: 0.0,
            cursor: None,
            cursor_colors_iter: cursor_colors_iter(),
            cursor_paints: false,
        }
    }

    fn set_cursor(&mut self, [x, y]: [f64; 2]) {
        // self.world.find_cell_for_position(position)

        let cell_x: usize = (x / self.cell_size) as usize;
        let cell_y: usize = (y / self.cell_size) as usize;

        let cell = self.world.cell_at(cell_x, cell_y);

        let color = match &self.cursor {
            Some(c) => c.color,
            None => WHITE,
        };

        self.cursor = Some(Cell { color, ..cell });

        if (cell_x, cell_y) != cell.at {
            self.cursor_colors_iter = cursor_colors_iter();
        }
    }

    fn flow_cursor_color(&mut self, [x, y]: [f64; 2]) {
        if let Some(mut cell) = self.cursor.as_mut() {
            match self.cursor_colors_iter.next() {
                Some(c) => cell.color = c,
                None => self.cursor_colors_iter = cursor_colors_iter(),
            }
        }
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
                        let (width, height, cell_size) = App::size_world(window_size[0], window_size[1]);
                        self.world = World::new(width, height, cell_size);
                        self.cell_size = cell_size;
                    }
                    Input::Move(motion) => match motion {
                        Motion::MouseCursor(position) => {
                            if self.cursor_paints {
                                self.world.write(self.cursor.unwrap())
                            }
                            self.set_cursor(*position);
                        }
                        Motion::MouseScroll(distance) => self.flow_cursor_color(*distance),
                        _ => {}
                    },
                    Input::Button(ButtonArgs {
                        state,
                        button,
                        scancode: _,
                    }) => match button {
                        Button::Mouse(b) => match b {
                            MouseButton::Left => {
                                self.cursor_paints = state == &ButtonState::Press;
                            }
                            MouseButton::Right => {}
                            _ => {}
                        },
                        Button::Keyboard(k) => match k {
                            Key::Right => {}
                            Key::Left => {}
                            Key::Up => {}
                            Key::Down => {}
                            Key::Space => {
                                self.update();
                            }
                            _ => {}
                        },
                        _ => {}
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
            self.update()
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
                b
            } else {
                a
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

        let square = rectangle::square(0.0, 0.0, self.cell_size as f64);
        let cells_vec = self.world.get_cells();
        let mut cells = cells_vec.iter();
        let cursor = self.cursor.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            while let Some(Cell {
                color,
                top_left,
                at: _,
            }) = cells.next()
            {
                let transform = c.transform.trans(top_left[0], top_left[1]);
                let rect = Rectangle::new(*color);
                rect.draw(square, &c.draw_state, transform, gl)
            }

            if let Some(Cell {
                color,
                top_left,
                at: _,
            }) = cursor
            {
                let transform = c.transform.trans(top_left[0], top_left[1]);
                let rect = Rectangle::new(color);
                rect.draw(square, &c.draw_state, transform, gl)
            }
        });
    }

    fn update(&mut self) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
        let write_cells = self.world.next();
        let mut write_cells_iter = write_cells.iter();

        while let Some(w_c) = write_cells_iter.next() {
            self.world.write(*w_c);
        }
    }
}
