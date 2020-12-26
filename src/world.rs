use crate::cell::Cell;
use nalgebra::{Dynamic, Matrix, Point2, Point6, VecStorage};
use conv::{ApproxFrom};

pub type WPoint = Point6<f64>;
pub type MPoint = Point2<usize>;
type XMatrix<T> = Matrix<T, Dynamic, Dynamic, VecStorage<T, Dynamic, Dynamic>>;
type WMatrix = XMatrix<WPoint>;
type MMatrix = XMatrix<MPoint>;

pub struct World {
    matrix: WMatrix,
    surroundings_matrix: MMatrix,
    locations_matrix: MMatrix,
    dead_pool: Vec<MPoint>,
    edge_width: usize,
    cols: usize,
    rows: usize,
}

impl World {
    pub fn new(rows: usize, cols: usize, cell_size: f64) -> Self {

        let matrix = WMatrix::from_element(rows, cols, WPoint::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
        let locations_matrix = matrix.map_with_location(|row, col, _p| MPoint::new(row, col)).clone();
        let dead_pool: Vec<MPoint> = locations_matrix.clone().into_iter().map(|p| *p).collect();

        let mut instance = Self {
            matrix,
            surroundings_matrix: MMatrix::from_element(rows + 2, cols + 2, MPoint::new(0, 0)),
            locations_matrix,
            dead_pool,
            edge_width: 1,
            cols,
            rows,
        };

        instance.resize_cells(cell_size);

        instance
    }

    pub fn reset(&self, cell_size: f64) -> Self {
        World::new(self.rows, self.cols, cell_size)
    }

    pub fn resize_cells(&mut self, cell_size: f64) {
        let mut locations_iter = self.locations_matrix.iter();

        while let Some(location) = locations_iter.next() {
            let row = location[0];
            let col = location[1];
            let w_point = &mut self.matrix[(row, col)];
            let x: f64 = ApproxFrom::<usize>::approx_from(col).unwrap();
            let y: f64 = ApproxFrom::<usize>::approx_from(row).unwrap();
            w_point[4] = x * cell_size;
            w_point[5] = y * cell_size;
        }
    }

    pub fn get_cells(&self) -> Vec<Cell> {
        self.locations_matrix
            .iter()
            .filter_map(|location| {
                if self.dead_pool.contains(&location) {
                    None
                }
                else {
                    let row = location[0];
                    let col = location[1];
                    Some(self.cell_at(row, col))
                }
            })
            .collect()
    }

    fn cell_at(&self, row: usize, col: usize) -> Cell {
        let w_point = self.matrix[(row, col)];
        (w_point, row, col).into()
    }

    pub fn find_cell_at(&self, row: usize, col: usize) -> Option<Cell> {
        if self.cols > col && self.rows > row {
            Some(self.cell_at(row, col))
        } else {
            None
        }
    }

    pub fn write(&mut self, cell: Cell) {
        let (w_point, row, col) = cell.into();
        self.matrix[(row, col)] = w_point;
        
        let location = self.locations_matrix[(row, col)];
        let dead_pool_index = self.dead_pool.iter().position(|p| p == &location);

        if w_point[0]==0.0 && w_point[1]==0.0 && w_point[2]==0.0&& w_point[3]==0.0 {
            if dead_pool_index.is_none() {
                self.dead_pool.push(location);
            }
        }
        else if let Some(i) = dead_pool_index {
            self.dead_pool.remove(i);
        }
    }

    pub fn mirror_edge(&mut self, edge_width: usize) {
        let top_slice = self.locations_matrix
            .slice((self.rows - edge_width, 0), (edge_width, self.cols))
            .map_with_location(|row, col, loc| Point2::new(Point2::new(row, col + edge_width), loc));
        let bottom_slice = self.locations_matrix
            .slice((0, 0), (edge_width, self.cols))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row + self.rows + edge_width, col + edge_width), loc)
            });

        let left_slice = self.locations_matrix
            .slice((0, self.cols - edge_width), (self.rows, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row + edge_width, col), loc)
            });
        let right_slice = self.locations_matrix
            .slice((0, 0), (self.rows, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row + edge_width, col + self.cols + edge_width), loc)
            });
        let top_left_slice = self.locations_matrix
            .slice(
                (self.rows - edge_width, self.cols - edge_width),
                (edge_width, edge_width),
            )
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row, col), loc)
            });
        let top_right_slice = self.locations_matrix
            .slice((self.rows - edge_width, 0), (edge_width, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row, col + self.cols + edge_width), loc)
            });
        let bottom_left_slice = self.locations_matrix
            .slice((0, self.cols - edge_width), (edge_width, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(Point2::new(row + self.rows + edge_width, col), loc)
            });
        let bottom_right_slice = self.locations_matrix
            .slice((0, 0), (edge_width, edge_width))
            .map_with_location(|row, col, loc| {
                Point2::new(
                    Point2::new(
                        row + self.rows + edge_width,
                        col + self.cols + edge_width,
                    ),
                    loc,
                )
            });

        let zero_point = MPoint::new(0, 0);
        let mut mirrored_matrix = self.locations_matrix.clone();
        mirrored_matrix = mirrored_matrix.insert_columns(0, edge_width, zero_point);
        mirrored_matrix =
            mirrored_matrix.insert_columns(self.cols + edge_width , edge_width, zero_point);
        mirrored_matrix = mirrored_matrix.insert_rows(0, edge_width, zero_point);
        mirrored_matrix =
            mirrored_matrix.insert_rows(self.rows + edge_width, edge_width, zero_point);

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

    pub fn get_surroundings(&self, (row, col): (usize, usize)) -> Vec<Cell> {
        let side = self.edge_width * 2 + 1;

        let surroundings = self.surroundings_matrix.slice((row, col), (side, side));

        surroundings
            .iter()
            .filter_map(|location| {
                if self.dead_pool.contains(&location)
                || location[0] != row 
                || location[1] != col {
                    None
                }
                else {
                    Some(self.cell_at(location[0], location[1]))
                }
            })
            .collect()
    }

    pub fn next<F>(&self, func: F) -> Vec<Cell>
    where
        F: Fn(Vec<Cell>, Cell) -> Option<Cell>
    {
        let cells_vec = self.get_cells();
        let mut cells = cells_vec.iter();
        let mut write_cells = Vec::new();
        
        while let Some(cell) = cells.next() {
            let surroundings = self.get_surroundings(cell.at);
            match &func(surroundings, *cell) {
                Some(c) => {
                    write_cells.push(*c);
                },
                None => {}
            }
        }
        write_cells
    }
}
