use graphics::types::Color;
use nalgebra::{Point2, Dynamic, Matrix, VecStorage, Point6};
use std::convert::TryInto;
use conv::{ValueFrom, ApproxFrom};


type WPoint = Point6<f64>;
type WMatrix = Matrix<WPoint, Dynamic, Dynamic, VecStorage<WPoint, Dynamic, Dynamic>>;


pub struct World {
    matrix: WMatrix,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Cell {
    pub color: Color,
    pub top_left: Point2<f64>,
    pub at: (usize, usize),
}

impl World {
    pub fn new(width: usize, height: usize, cell_size: f64) -> Self {
        let mut instance = Self {
            matrix: WMatrix::from_element(width, height, WPoint::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
            width,
            height,
        };

        instance.resize_cells(cell_size);

        instance
    }

    fn locations_matrix(&self) -> Matrix<Point2<usize>, Dynamic, Dynamic, VecStorage<Point2<usize>, Dynamic, Dynamic>> {
        self.matrix.map_with_location(|x, y, p| {
            Point2::new(x, y)
        })
    }

    pub fn resize_cells(&mut self, cell_size: f64) {
        let locations_matrix = self.locations_matrix();
        let mut locations_iter = locations_matrix.iter();

        while let Some(location) = locations_iter.next() {
            let x_index = location[0];
            let y_index = location[1];
            let w_point = &mut self.matrix[(x_index, y_index)];
            w_point[4] = (x_index as f64) * cell_size;
            w_point[5] = (y_index as f64) * cell_size;
        }
    }

    pub fn get_cells(&self) -> Vec<Cell> {
        self.locations_matrix()
            .iter()
            .map(|location| {
                let x = location[0];
                let y = location[1];
                self.cell_at(x, y)
            })
            .collect()
    }

    fn cell_at(&self, x_index: usize, y_index: usize) -> Cell {
        let w_point = self.matrix[(x_index, y_index)];
        let r = w_point[0];
        let g = w_point[1];
        let b = w_point[2];
        let a = w_point[3];
        let x = w_point[4];
        let y = w_point[5];

        Cell {
            color: [
                f32::approx_from(r).unwrap(),
                f32::approx_from(g).unwrap(),
                f32::approx_from(b).unwrap(),
                f32::approx_from(a).unwrap(), 
            ],
            top_left: Point2::new(x, y),
            at: (x_index, y_index),
        }
    }

    pub fn find_cell_at(&self, x_index: usize, y_index: usize) -> Option<Cell> {
        if self.width > y_index && self.height > x_index {
            Some(self.cell_at(x_index, y_index))
        }
        else {
            None
        }
    }

    pub fn write(
        &mut self,
        Cell {
            at,
            color: [r, g, b, a],
            top_left,
        }: Cell,
    ) {
        let x = top_left[0];
        let y = top_left[1];

        self.matrix[at] = WPoint::new(
            f64::value_from(r).unwrap(),
            f64::value_from(g).unwrap(),
            f64::value_from(b).unwrap(),
            f64::value_from(a).unwrap(),
            x,
            y
        )
    }

    // pub fn find_cell_for_position() {
    // }

    fn get_surroundings(&self, (x, y): (usize, usize)) -> [[Cell; 3]; 3] {
        // let mut slice_shape = (3, 3);
        let mut add_x_before = None;
        let mut add_x_after = None;
        let mut add_y_before = None;
        let mut add_y_after = None;

        if x == 0 {
            add_x_before = Some(self.width - 1);
        } else if x == self.width - 1 {
            add_x_after = Some(0);
        }

        if y == 0 {
            add_y_before = Some(self.height - 1);
        } else if y == self.height - 1 {
            add_y_after = Some(0);
        }

        let surroundings: [(usize, usize); 9] = {
            let mut cells = [(0, 0); 9];
            let mut index = 0;
            let cross: [isize; 3] = [-1, 0, 1];
            for i_x in &cross {
                for i_y in &cross {
                    cells[index] = (
                        {
                            if i_x > &0 && add_x_after.is_some() {
                                add_x_after.unwrap()
                            } else if i_x < &0 && add_x_before.is_some() {
                                add_x_before.unwrap()
                            } else {
                                (i_x + x as isize).try_into().unwrap()
                            }
                        },
                        {
                            if i_y > &0 && add_y_after.is_some() {
                                add_y_after.unwrap()
                            } else if i_y < &0 && add_y_before.is_some() {
                                add_y_before.unwrap()
                            } else {
                                (i_y + y as isize).try_into().unwrap()
                            }
                        },
                    );
                    index += 1;
                }
            }

            cells
        };

        let mut result: Vec<Cell> = Vec::new();

        for (x, y) in surroundings.iter() {
            result.push(self.cell_at(*x, *y));
        }

        [
            [result[0], result[3], result[6]],
            [result[1], result[4], result[7]],
            [result[2], result[5], result[8]],
        ]
    }

    pub fn next<F>(&self, func: F) -> Vec<Cell>
        where F: Fn([[Cell;3];3])->Option<Cell>
    {
        let cells_vec = self.get_cells();
        let mut cells = cells_vec.iter();
        let mut write_cells = Vec::new();

        while let Some(cell) = cells.next() {
            let surroundings = self.get_surroundings(cell.at);
            match func(surroundings) {
                Some(c) => write_cells.push(c),
                None => {}
            }
        }

        write_cells
    }
}
