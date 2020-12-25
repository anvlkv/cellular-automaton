use crate::world::WPoint;
use conv::{ApproxFrom, ValueFrom};
use graphics::types::Color;
use nalgebra::Point2;

type CellRepresntation = (WPoint, usize, usize);

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Cell {
    pub color: Color,
    pub top_left: Point2<f64>,
    pub at: (usize, usize),
}

impl From<CellRepresntation> for Cell {
    fn from((w_point, row, col): CellRepresntation) -> Self {
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
            at: (row, col),
        }
    }
}

impl Into<WPoint> for Cell {
    fn into(self) -> WPoint {
        let Cell {
            at:_,
            color: [r, g, b, a],
            top_left,
        } = self;
        let x = top_left[0];
        let y = top_left[1];

        WPoint::new(
            f64::value_from(r).unwrap(),
            f64::value_from(g).unwrap(),
            f64::value_from(b).unwrap(),
            f64::value_from(a).unwrap(),
            x,
            y,
        )
    }
}

impl Into<CellRepresntation> for Cell {
    fn into(self) -> CellRepresntation {
        let Cell {
            at,
            color:_,
            top_left:_,
        } = self;

        (
            self.into(),
            at.0,
            at.1,
        )
    }
}
