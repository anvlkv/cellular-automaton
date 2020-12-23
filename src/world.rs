use crate::cell::Cell;
use nalgebra::{Dynamic, Matrix, Point2, Point6, VecStorage};

pub type WPoint = Point6<f64>;
pub type MPoint = Point2<usize>;
type XMatrix<T> = Matrix<T, Dynamic, Dynamic, VecStorage<T, Dynamic, Dynamic>>;
type WMatrix = XMatrix<WPoint>;
type MMatrix = XMatrix<MPoint>;

pub struct World {
    matrix: WMatrix,
    surroundings_matrix: MMatrix,
    edge_width: usize,
    width: usize,
    height: usize,
}

impl World {
    pub fn new(width: usize, height: usize, cell_size: f64) -> Self {
        let mut instance = Self {
            matrix: WMatrix::from_element(height, width, WPoint::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
            surroundings_matrix: MMatrix::from_element(height + 2, width + 2, MPoint::new(0, 0)),
            edge_width: 1,
            width,
            height,
        };

        instance.resize_cells(cell_size);

        instance
    }

    fn locations_matrix(
        &self,
    ) -> Matrix<MPoint, Dynamic, Dynamic, VecStorage<MPoint, Dynamic, Dynamic>> {
        self.matrix.map_with_location(|x, y, _p| MPoint::new(x, y))
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
        (w_point, x_index, y_index).into()
    }

    pub fn find_cell_at(&self, x_index: usize, y_index: usize) -> Option<Cell> {
        if self.width > y_index && self.height > x_index {
            Some(self.cell_at(x_index, y_index))
        } else {
            None
        }
    }

    pub fn write(&mut self, cell: Cell) {
        let (w_point, x, y) = cell.into();
        self.matrix[(x, y)] = w_point;
    }

    pub fn mirror_edge(&mut self, edge_width: usize) {
        let locations_matrix = self.locations_matrix();

        let top_slice = locations_matrix
            .slice((self.height - edge_width, 0), (edge_width, self.width))
            .map_with_location(|row, col, loc| Point2::new(Point2::new(row, col + edge_width), loc));
        let bottom_slice = locations_matrix
            .slice((0, 0), (edge_width, self.width))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row + self.height + edge_width, col + edge_width), loc)
            });

        let left_slice = locations_matrix
            .slice((0, self.width - edge_width), (self.height, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row + edge_width, col), loc)
            });
        let right_slice = locations_matrix
            .slice((0, 0), (self.height, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row + edge_width, col + self.width + edge_width), loc)
            });
        let top_left_slice = locations_matrix
            .slice(
                (self.height - edge_width, self.width - edge_width),
                (edge_width, edge_width),
            )
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row, col), loc)
            });
        let top_right_slice = locations_matrix
            .slice((self.height - edge_width, 0), (edge_width, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row, col + self.width + edge_width), loc)
            });
        let bottom_left_slice = locations_matrix
            .slice((0, self.width - edge_width), (edge_width, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row + self.height + edge_width, col), loc)
            });
        let bottom_right_slice = locations_matrix
            .slice((0, 0), (edge_width, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(
                    Point2::new(
                        row + self.height + edge_width,
                        col + self.width + edge_width,
                    ),
                    loc,
                )
            });

        let zero_point = MPoint::new(0, 0);
        let mut mirrored_matrix = locations_matrix.clone();
        mirrored_matrix = mirrored_matrix.insert_columns(0, edge_width, zero_point);
        mirrored_matrix =
            mirrored_matrix.insert_columns(self.width + edge_width , edge_width, zero_point);
        mirrored_matrix = mirrored_matrix.insert_rows(0, edge_width, zero_point);
        mirrored_matrix =
            mirrored_matrix.insert_rows(self.height + edge_width, edge_width, zero_point);

        let slices = [
            top_left_slice,
            top_slice,
            top_right_slice,
            right_slice,
            bottom_right_slice,
            bottom_slice,
            bottom_left_slice,
            left_slice,
        ];

        for slice in slices.iter() {
            for point in slice.iter() {
                let insert_at = (point[0][0], point[0][1]);
                let location = point[1];

                mirrored_matrix[insert_at] = location;
            }
        }

        self.surroundings_matrix = mirrored_matrix;
        self.edge_width = edge_width;
    }

    fn get_surroundings(&self, (x, y): (usize, usize)) -> Vec<Cell> {
        let side = self.edge_width * 2 + 1;

        let surroundings = self.surroundings_matrix.slice((x, y), (side, side));

        surroundings
            .iter()
            .filter(|at| at[0] != x || at[1] != y)
            .map(|at| self.cell_at(at[0], at[1]))
            .collect()
    }

    pub fn next<F>(&self, func: F) -> Vec<Cell>
    where
        F: Fn(Vec<Cell>, Cell) -> Option<Cell>,
    {
        let cells_vec = self.get_cells();
        let mut cells = cells_vec.iter();
        let mut write_cells = Vec::new();

        while let Some(cell) = cells.next() {
            let surroundings = self.get_surroundings(cell.at);
            match func(surroundings.clone(), *cell) {
                Some(c) => {
                    write_cells.push(c)
                },
                None => {}
            }
            // std::thread::spawn(|| {
            // });
        }

        write_cells
    }
}
