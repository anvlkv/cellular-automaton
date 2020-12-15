use graphics::types::Color;
use nalgebra::Point;
use nalgebra::Point2;
use nalgebra::U2;
use nalgebra::{Dynamic, Matrix, VecStorage};
use piston::input::{Event, GenericEvent};

type XMatrix<T> = Matrix<T, Dynamic, Dynamic, VecStorage<T, Dynamic, Dynamic>>;

pub struct World {
    r_matrix: XMatrix<f32>,
    g_matrix: XMatrix<f32>,
    b_matrix: XMatrix<f32>,
    x_matrix: XMatrix<f64>,
    y_matrix: XMatrix<f64>,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub color: Color,
    pub top_left: Point<f64, U2>,
}

impl World {
    pub fn new(width: usize, height: usize, cell_size: f64) -> Self {
        let mut instance = Self {
            r_matrix: XMatrix::from_element(width, height, 0.1),
            g_matrix: XMatrix::from_element(width, height, 0.2),
            b_matrix: XMatrix::from_element(width, height, 0.3),
            x_matrix: XMatrix::from_element(width, height, 0.0),
            y_matrix: XMatrix::from_element(width, height, 0.0),
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

    pub fn position(&mut self) {}

    pub fn cells_iter<'a>(&'a self) -> impl Iterator<Item = Cell> + 'a {
        let r_cells = self.r_matrix.iter();
        let g_cells = self.g_matrix.iter();
        let b_cells = self.b_matrix.iter();
        let x_cells = self.x_matrix.iter();
        let y_cells = self.y_matrix.iter();

        r_cells
            .zip(g_cells)
            .zip(b_cells)
            .zip(x_cells)
            .zip(y_cells)
            .map(|((((r, g), b), x), y)| Cell {
                color: [*r, *g, *b, 1.0],
                top_left: Point2::new(*x, *y),
            })
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
        }
    }

    // pub fn find_cell_for_position() {
    // }

    pub fn next(&mut self) {
        // self.r_matrix = self.r_matrix.clone() + self.g_matrix.clone();
        // self.g_matrix = self.b_matrix.clone() - self.r_matrix.clone();
        // self.b_matrix = self.g_matrix.clone() + self.b_matrix.clone();
    }
}
