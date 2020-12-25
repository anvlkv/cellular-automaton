use crate::cell::Cell;
use crate::world::World;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use palette::{Gradient, Hsv, LinSrgba};
use piston::input::{Button, ButtonArgs, Input, Key, Motion, MouseButton, RenderArgs, ResizeArgs};
use piston::{ButtonState, Event, Loop};
use std::vec::IntoIter;
use conv::{ApproxFrom};

const SUPER_NOVA: [f32;4] = [1.0;4];
const DEAD: [f32;4] = [0.0;4];

enum CursorAction {
    Paint,
    Clear,
}

pub struct WorldController {
    world: World,
    cell_size: f64,
    frame_size: usize,
    cursor: Option<Cell>,
    cursor_colors_iter: IntoIter<Color>,
    cursor_action: Option<CursorAction>,
    paused: bool,
    speed: isize,
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

    while end > start {
        let color = LinSrgba::from(cursor_gradient.get(end));
        let (r, g, b, a) = color.into_components();
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
            cursor_action: None,
            frame_size: 3,
            paused: true,
            speed: 1
        }
    }

    fn size_world(width: f64, height: f64) -> (usize, usize, f64) {
        let cell_size = Self::get_cell_size(width, height);
        let rows: f64 = height / cell_size;
        let cols: f64 = width / cell_size;
        (
            ApproxFrom::<f64>::approx_from(rows).unwrap(), // count rows
            ApproxFrom::<f64>::approx_from(cols).unwrap(),  // count columns
            cell_size,
        )
    }

    fn set_cursor(&mut self, [x, y]: [f64; 2]) {
        let col: usize = ApproxFrom::<f64>::approx_from(x / self.cell_size).unwrap();
        let row: usize = ApproxFrom::<f64>::approx_from(y / self.cell_size).unwrap();

        match self.world.find_cell_at(row, col) {
            Some(cell) => {
                let color = match &self.cursor {
                    Some(c) => c.color,
                    None => SUPER_NOVA,
                };
                self.cursor = Some(Cell { color, ..cell });
                if (row, col) != cell.at {
                    self.cursor_colors_iter = cursor_colors_iter();
                }
            }
            None => {}
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
            Event::Loop(lp) => match lp {
                Loop::Render(args) => {
                    self.render(&args, gl);
                }
                Loop::Update(_) => {
                    if !self.paused {
                        for _i in 0 .. self.speed {
                            self.update();
                        }
                    }
                }
                _ => {}
            },
            Event::Input(input, _ts) => match input {
                Input::Resize(ResizeArgs {
                    window_size,
                    draw_size: _,
                }) => {
                    let (rows, cols, cell_size) =
                        Self::size_world(window_size[0], window_size[1]);
                    self.world = World::new(rows, cols, cell_size);
                    self.cell_size = cell_size;
                    self.world.mirror_edge(self.frame_size % 2);
                }
                Input::Move(motion) => match motion {
                    Motion::MouseCursor(position) => {
                        if let Some(action) = &self.cursor_action {
                            match action {
                                CursorAction::Paint => self.world.write(self.cursor.unwrap()),
                                CursorAction::Clear => {
                                    let cursor = self.cursor.unwrap();
                                    self.world.write(Cell {
                                        color: DEAD,
                                        ..cursor
                                    });
                                }
                            }
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
                            if state == &ButtonState::Press {
                                self.cursor_action = Some(CursorAction::Paint);
                                if let Some(c) = self.cursor {
                                    self.world.write(c);
                                }
                            } else {
                                self.cursor_action = None;
                            }
                        }
                        MouseButton::Right => {
                            if state == &ButtonState::Press {
                                self.cursor_action = Some(CursorAction::Clear);
                                let cursor = self.cursor.unwrap();
                                self.world.write(Cell {
                                    color: DEAD,
                                    ..cursor
                                });
                            } else {
                                self.cursor_action = None
                            }
                        }
                        _ => {}
                    },
                    Button::Keyboard(k) => match k {
                        Key::Right => {
                            self.speed += 1;
                        }
                        Key::Left => {
                            self.speed -= 1;
                        }
                        Key::Up => {}
                        Key::Down => {}
                        Key::Space => {
                            self.paused = state == &ButtonState::Release;
                        }
                        Key::C => {
                            self.world = self.world.reset(self.cell_size);
                            self.world.mirror_edge(self.frame_size % 2);
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Event::Custom(_eid, _arc, _ts) => {}
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

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.cell_size as f64);
        let cells_vec = self.world.get_cells();
        let mut cells = cells_vec.iter();
        let cursor = self.cursor.clone();

        gl.draw(args.viewport(), |c, gl| {
            clear(DEAD, gl);

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
        enum Channels {
            Red,
            Green,
            Blue
        }

        fn is_alive(cell: &Cell) -> bool {
            is_chanel_alive(cell, &Channels::Red)
            || is_chanel_alive(cell, &Channels::Green)
            || is_chanel_alive(cell, &Channels::Blue)
        }

        fn is_super_nova(cell: &Cell) -> bool {
            let [r, g, b, a] = cell.color;
            (r + g + b) * a >= 3.0
        }

        fn is_chanel_alive(cell: &Cell, ch: &Channels) -> bool {
            let [r, g, b, a] = cell.color;
            match ch {
                Red => r * a > 0.0,
                Green => g * a > 0.0,
                Blue => b * a > 0.0
            }
        }

        fn is_chanel_growing(cell: &Cell, ch: &Channels) -> bool {
            let [r, g, b, a] = cell.color;
            match ch {
                Red => r * a > 0.5,
                Green => g * a > 0.5,
                Blue => b * a > 0.5
            }
        }




        let the_rule = |neighbors: Vec<Cell>, t_cell: Cell| {
            const SPLIT_C: f32 = 0.5;
            const LIVING_C: f32 = 0.89;

            let alive = is_alive(&t_cell);
            let neighbors_alive: Vec<&Cell> = neighbors.iter().filter(|n| is_alive(*n)).collect();

            if alive || neighbors_alive.len() > 0 {
                let neighboring_sns: Vec<&Cell> = neighbors_alive.clone().into_iter().filter(|n| is_super_nova(n)).collect();
                if is_super_nova(&t_cell) {
                    Some(Cell {
                        color: DEAD,
                        ..t_cell
                    })
                }
                else if neighboring_sns.len() > 0 {
                    let mut cell = Cell {
                        ..t_cell
                    };
                    
                    for sn in neighboring_sns {
                        let (sn_row, sn_col) = sn.at;
                        let (row, col) = t_cell.at;

                        if sn_row == row {
                            cell.color[0] += sn.color[0] * SPLIT_C;
                            if sn_col > col {
                                cell.color[2] += sn.color[2] * SPLIT_C;
                            }
                            else {
                                cell.color[1] += sn.color[1] * SPLIT_C;
                            }
                        }
                        else if sn_col == col {
                            cell.color[2] += sn.color[2] * SPLIT_C;
                            if sn_row > row {
                                cell.color[2] += sn.color[2] * SPLIT_C;
                            }
                            else {
                                cell.color[1] += sn.color[1] * SPLIT_C;
                            }
                        }
                        else {
                            if sn_row > row {
                                cell.color[2] += sn.color[2] * SPLIT_C;
                            }
                            else if sn_col < col{
                                cell.color[1] += sn.color[1] * SPLIT_C;
                            }
                            else {
                                cell.color[0] += sn.color[0] * SPLIT_C;
                            }
                        }
                    }

                    if !is_alive(&cell) {
                        cell.color[3] = 1.0;
                    }

                    Some(cell)
                }
                else if neighbors_alive.len() > 0 {
                    let chs_arr = [Channels::Red, Channels::Green, Channels::Blue];
                    let chs =  chs_arr.iter().enumerate();
                    let mut cell = Cell {
                        ..t_cell
                    };

                    for neighbor in neighbors_alive {
                        for (at, ch) in chs.clone() {
                            if is_chanel_growing(neighbor, &ch) {
                                cell.color[at] +=  neighbor.color[at] * SPLIT_C
                            }
                            else if is_chanel_alive(neighbor, &ch) && is_chanel_alive(&t_cell, &ch) {
                                cell.color[at] +=  neighbor.color[at] * SPLIT_C * SPLIT_C
                            }
                        }
                    }

                    if alive {
                        for (at, ch) in chs.clone() {
                            if is_chanel_alive(&t_cell, &ch) {
                                cell.color[at] = cell.color[at] * LIVING_C;
                            }
                        }   
                    }

                    for (at, _ch) in chs.clone() {
                        if cell.color[at] > 1.0 {
                            let mut offset_at = at + 1;

                            if at == 3 {
                                offset_at = 0;
                            }
                            else if at == 0 {
                                offset_at = 3;
                            }

                            cell.color[offset_at] += cell.color[offset_at] - 1.0;
                            cell.color[at] = 1.0;
                        }
                    }

                    if !is_alive(&cell) {
                        cell.color[3] = 1.0;
                    }

                    Some(cell)
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

        // println!("{:?}", write_cells);

        while let Some(w_c) = write_cells_iter.next() {
            self.world.write(*w_c);
        }
    }
}
