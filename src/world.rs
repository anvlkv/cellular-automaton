use std::sync::mpsc::channel;
use crate::cell::Cell;
use nalgebra::{Dynamic, Matrix, Point2, Point6, VecStorage};
use conv::{ApproxFrom};

pub type WPoint = Point6<f64>;
pub type MPoint = Point2<usize>;
type XMatrix<T> = Matrix<T, Dynamic, Dynamic, VecStorage<T, Dynamic, Dynamic>>;
type WMatrix = XMatrix<WPoint>;
type MMatrix = XMatrix<MPoint>;

#[derive(Clone)]
pub struct World {
    matrix: WMatrix,
    surroundings_matrix: MMatrix,
    locations_matrix: MMatrix,
    edge_width: usize,
    cell_size: f64,
    cols: usize,
    rows: usize,
}

impl World {
    pub fn new(rows: usize, cols: usize, cell_size: f64) -> Self {
        let mut instance = Self {
            matrix: WMatrix::from_element(rows, cols, WPoint::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
            surroundings_matrix: MMatrix::from_element(rows + 2, cols + 2, MPoint::new(0, 0)),
            locations_matrix: MMatrix::from_element(rows, cols, MPoint::new(0, 0)),
            edge_width: 1,
            cell_size,
            cols,
            rows,
        };

        instance.locations_matrix();
        instance.resize_cells(cell_size);

        instance
    }

    pub fn reset(&mut self) {
        let new_instance = World::new(self.rows, self.cols, self.cell_size);

        self.mirror_edge(self.edge_width);
        
        self.matrix = new_instance.matrix;
        self.surroundings_matrix = new_instance.surroundings_matrix;
    }

    pub fn resize(&mut self, rows: usize, cols: usize, cell_size: f64) {
        self.rows = rows;
        self.cols = cols;
        self.cell_size = cell_size;
        self.reset();
    }

    fn locations_matrix(
        &mut self,
    ) {
       self.locations_matrix = self.matrix.map_with_location(|row, col, _p| MPoint::new(row, col)).clone();
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
            .map(|location| {
                let row = location[0];
                let col = location[1];
                self.cell_at(row, col)
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
            .filter(|at| at[0] != row || at[1] != col)
            .map(|at| self.cell_at(at[0], at[1]))
            .collect()
    }

    pub fn next<'a, F>(&'static mut self, rule: F)
    where
        F: FnOnce(Vec<Cell>, Cell) -> Option<Cell>,
        F: Send + 'static,
        F: Copy
    {
        let mut cells_vec = self.get_cells();
        let mut handles = Vec::new();
        let frame = (self.edge_width * 2 + 1)^2;
        let total_cells = cells_vec.len();
        for t in 0 .. (total_cells / frame) {
            let slc = cells_vec.split_off(total_cells - frame * (t + 1) );
            // let t_rule = rule.clone();
            handles.push(std::thread::spawn(move || {
                let mut cells = slc.into_iter();
                let mut write_cells = Vec::new();
                while let Some(cell) = cells.next() {
                    let surroundings = self.get_surroundings(cell.at);
                    match rule(surroundings, cell) {
                        Some(c) => {
                            write_cells.push(c);
                        },
                        None => {}
                    }
                }
                write_cells
            }));
        }

        
        
        // let mut updated_self = ;

        for h in handles {
            let updates = h.join().unwrap();
            for c in updates {
                self.write(c);
                
            }
        }
        


        
    }
}
