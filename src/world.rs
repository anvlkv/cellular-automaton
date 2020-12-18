use graphics::types::Color;
use nalgebra::Point;
use nalgebra::Point2;
use nalgebra::U2;
use nalgebra::{Dynamic, Matrix, VecStorage};
// use piston::input::{Event, GenericEvent};
use std::convert::TryInto;

type XMatrix<T> = Matrix<T, Dynamic, Dynamic, VecStorage<T, Dynamic, Dynamic>>;

pub struct World {
    r_matrix: XMatrix<f32>,
    g_matrix: XMatrix<f32>,
    b_matrix: XMatrix<f32>,
    x_matrix: XMatrix<f64>,
    y_matrix: XMatrix<f64>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Cell {
    pub color: Color,
    pub top_left: Point<f64, U2>,
    pub at: (usize, usize),
}

impl World {
    pub fn new(width: usize, height: usize, cell_size: f64) -> Self {
        let mut instance = Self {
            r_matrix: XMatrix::from_element(width, height, 0.0),
            g_matrix: XMatrix::from_element(width, height, 0.0),
            b_matrix: XMatrix::from_element(width, height, 0.0),
            x_matrix: XMatrix::from_element(width, height, 0.0),
            y_matrix: XMatrix::from_element(width, height, 0.0),
            width,
            height,
        };

        instance.resize_cells(cell_size);

        instance
    }

    pub fn resize_cells(&mut self, cell_size: f64) {
        let mut x_y_col_iter = self
            .x_matrix
            .column_iter_mut()
            .zip(self.y_matrix.column_iter_mut());
        let mut y = 0.0;
        while let Some((mut x_col, mut y_col)) = x_y_col_iter.next() {
            let mut x = 0.0;
            let mut x_y_row_iter = x_col.row_iter_mut().zip(y_col.row_iter_mut());

            while let Some((mut x_cell, mut y_cell)) = x_y_row_iter.next() {
                x_cell[(0, 0)] = x;
                y_cell[(0, 0)] = y;
                x += cell_size;
            }
            y += cell_size;
        }
    }

    // pub fn find_position(&self, x: &f64, y: &f64) -> Option<Cell> {
    //     let mut diag_iter = self
    //         .r_matrix
    //         .map_with_location(|x, y, _| Point2::new(x, y))
    //         .diagonal()
    //         .iter()
    //         .map(|at| self.cell_at(at[0], at[1]))
    //         .filter(|cell_at| {
    //             &cell_at.top_left[0] >= x 
    //             && &cell_at.top_left[1] >= y
    //         });

    //     while let Some(cell) = diag_iter.next() {
    //         for i in cell.at.0..self.width {
    //             if self.cell_at(i, cell.at.1).top_left[0] <= x {

    //             }
    //         }
    //     }

    //     todo!()
    // }

    pub fn get_cells(&self) -> Vec<Cell> {
        self.r_matrix
            .map_with_location(|x, y, _| Point2::new(x, y))
            .iter()
            .map(|at| self.cell_at(at[0], at[1]))
            .collect::<Vec<Cell>>()
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Cell {
        Cell {
            color: [
                self.r_matrix[(x, y)],
                self.g_matrix[(x, y)],
                self.b_matrix[(x, y)],
                1.0,
            ],
            top_left: Point2::new(self.x_matrix[(x, y)], self.y_matrix[(x, y)]),
            at: (x, y),
        }
    }

    pub fn write(
        &mut self,
        Cell {
            at,
            color: [r, g, b, _a],
            top_left,
        }: Cell,
    ) {
        let x = top_left[0];
        let y = top_left[1];

        self.r_matrix[at] = r;
        self.g_matrix[at] = g;
        self.b_matrix[at] = b;
        self.x_matrix[at] = x;
        self.y_matrix[at] = y;
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

    pub fn next(&mut self) -> Vec<Cell> {
        let cells_vec = self.get_cells();
        let mut cells = cells_vec.iter();
        let mut write_cells = Vec::new();

        while let Some(cell) = cells.next() {
            let surroundings = self.get_surroundings(cell.at);
            let mut surroundings_iter = surroundings.iter().flatten();

            while let Some(compare) = surroundings_iter.next() {
                if cell.at != compare.at && compare.color != [0.0, 0.0, 0.0, 1.0] {
                    write_cells.push(Cell {
                        at: cell.at.clone(),
                        top_left: cell.top_left,
                        ..{ compare.clone() }
                    })
                }
            }
        }

        write_cells
    }
}
