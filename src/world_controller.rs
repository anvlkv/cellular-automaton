use opengl_graphics::GlGraphics;
use super::world::{Cell, World};
use conv::{ApproxInto, RoundToNearest};
use graphics::color::{BLACK, WHITE};
use graphics::types::Color;
use palette::{Gradient, Hsv, LinSrgba};
use piston::input::{
    Button, ButtonArgs, Input, Key, Motion, MouseButton, RenderArgs, RenderEvent, ResizeArgs,
    UpdateArgs, UpdateEvent,
};
use piston::{ButtonState, Event, Loop};
use std::vec::IntoIter;
use std::time::Instant;

pub struct WorldController {
    world: World,
    cell_size: f64,
    cursor: Option<Cell>,
    cursor_colors_iter: IntoIter<Color>,
    cursor_paints: bool,
    paused: bool
}


fn cursor_colors_iter() -> IntoIter<Color> {
    let cursor_gradient: Gradient<Hsv> = Gradient::new(vec![
        Hsv::new(0.0, 1.0, 0.5),
        Hsv::new(90.0, 1.0, 0.5),
        Hsv::new(180.0, 1.0, 0.5),
        Hsv::new(270.0, 1.0, 0.5),
        Hsv::new(360.0, 1.0, 0.5),
        Hsv::new(275.0, 1.0, 0.5),
        Hsv::new(165.0, 1.0, 0.5),
        Hsv::new(85.0, 1.0, 0.5),
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

impl WorldController {
    pub fn new() -> Self {
        let world = World::new(0, 0, 0.0);

        Self {
            world,
            cell_size: 0.0,
            cursor: None,
            cursor_colors_iter: cursor_colors_iter(),
            cursor_paints: false,
            paused: true
        }
    }

    fn size_world(width: f64, height: f64) -> (usize, usize, f64) {
        let cell_size = Self::get_cell_size(width, height);

        (
            (width / cell_size) as usize,
            (height / cell_size) as usize,
            cell_size,
        )
    }

    fn set_cursor(&mut self, [x, y]: [f64; 2]) {
        // self.world.find_cell_for_position(position)

        let cell_x: usize = (x / self.cell_size) as usize;
        let cell_y: usize = (y / self.cell_size) as usize;

        println!("{}, {}", cell_x, cell_y);
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

    pub fn handle_event(&mut self, e: &Event, gl: &mut GlGraphics) {
        match e {
            Event::Loop(lp) => {
                match lp {
                    Loop::Render(args) => {
                        self.render(&args, gl);
                    },
                    Loop::Update(_) => {
                        if !self.paused {
                            self.update()
                        }
                    },
                    _ => {}
                }
                // println!("Loop {:?}", lp)
            }
            Event::Input(input, _ts) => {
                // println!("Iput {:?}, {:?}", i, ts)
                match input {
                    Input::Resize(ResizeArgs {
                        window_size,
                        draw_size: _,
                    }) => {
                        let (width, height, cell_size) = Self::size_world(window_size[0], window_size[1]);
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
                                if let Some(c) = self.cursor {
                                    self.world.write(c);
                                }
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
                                self.paused = state == &ButtonState::Release;
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

        

        // if let Some(args) = e.update_args() {
        //     self.update()
        // }
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

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.cell_size as f64);
        let cells_vec = self.world.get_cells();
        let mut cells = cells_vec.iter();
        let cursor = self.cursor.clone();

        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

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

    pub fn update(&mut self) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
        fn is_alive(cell: Cell) -> bool {
            let [r,g,b,_] = cell.color;

            r + g + b > 0.0
        }

        

        // fn is_alive(cell: Cell) -> bool {
        //     let [r,g,b,a] = cell.color;

        //     r + g + b > 0.0
        // }

        let the_rule = |[
            [c11, c12, c13],
            [c21, trg, c23],
            [c31, c32, c33]
        ]: [[Cell;3];3]| {
            let alive = is_alive(trg);
            let neighbors = [c11, c12, c13,c21, c23, c31, c32, c33];
            let neighbors_alive = neighbors.iter().filter(|n|is_alive(**n));

            if neighbors_alive.clone().count() >= 4 {
                if alive {
                    Some(Cell{
                        color: BLACK,
                        ..trg
                    })
                    // None
                }
                else {
                    None
                }
            }
            else if neighbors_alive.clone().count() >= 3 {
                if !alive {
                    Some(Cell{
                        color: WHITE,
                        ..trg
                    })
                }
                else {
                    None
                }
            }
            else if neighbors_alive.count() < 2 {
                if alive {
                    Some(Cell{
                        color: BLACK,
                        ..trg
                    })
                }
                else {
                    None
                }
            }
            else {
                None
            }


        };
        let write_cells = self.world.next(the_rule);
        let mut write_cells_iter = write_cells.iter();

        while let Some(w_c) = write_cells_iter.next() {
            self.world.write(*w_c);
        }
    }

    
}