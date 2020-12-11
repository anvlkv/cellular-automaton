


use nalgebra::{Matrix, VecStorage, Dynamic};
use piston::input::{GenericEvent, Event};

type XMatrix = Matrix<f32, Dynamic, Dynamic, VecStorage<f32, Dynamic, Dynamic>>;

pub struct World {
    r_matrix: XMatrix,
    g_matrix: XMatrix,
    b_matrix: XMatrix,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            r_matrix: XMatrix::from_element(width, height, 0.0),
            g_matrix: XMatrix::from_element(width, height, 0.0),
            b_matrix: XMatrix::from_element(width, height, 0.0),
        }
    }

    pub fn cells(&self) -> Vec<Vec<(f32, f32, f32)>> {
        let r_rows = self.r_matrix.row_iter();
        let g_rows = self.g_matrix.row_iter();
        let b_rows = self.b_matrix.row_iter();

        r_rows.zip(g_rows).zip(b_rows).map(|((r, g), b)| {
            r.column_iter().zip(g.column_iter()).zip(b.column_iter()).map(|((r, g), b)| {
                (r[0], g[0], b[0])
            }).collect::<Vec<(f32, f32, f32)>>()
        }).collect()
    }

    pub fn next(&mut self) {
        // self.r_matrix = self.r_matrix.clone() + self.g_matrix.clone();
        // self.g_matrix = self.b_matrix.clone() - self.r_matrix.clone();
        // self.b_matrix = self.g_matrix.clone() + self.b_matrix.clone();
    }
}