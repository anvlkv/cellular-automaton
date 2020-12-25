#[macro_use]
extern crate bencher;
use bencher::Bencher;
use cellular_automaton::cell::Cell;
use cellular_automaton::world::WPoint;
use nalgebra::Point2;
use std::mem::size_of_val;

fn from_zero_point(bench: &mut Bencher) {
    let point = WPoint::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    bench.iter(|| {
        let _c: Cell = Cell::from((point, 0, 0));
    });

    bench.bytes = size_of_val(&point) as u64;
}

fn into_zero_point(bench: &mut Bencher) {
    let cell = Cell {
        color: [0.0, 0.0, 0.0, 0.0],
        top_left: Point2::new(0.0, 0.0),
        at: (0, 0)
    };

    bench.iter(|| {
        let _w: WPoint = cell.into();
    });

    let w_point: WPoint = cell.into();
    bench.bytes = size_of_val(&w_point) as u64;
}

fn from_one_point(bench: &mut Bencher) {
    let point = WPoint::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
    bench.iter(|| {
        let _c: Cell = Cell::from((point, 1, 1));
    });

    bench.bytes = size_of_val(&point) as u64;
}

fn into_one_point(bench: &mut Bencher) {
    let cell = Cell {
        color: [1.0, 1.0, 1.0, 1.0],
        top_left: Point2::new(1.0, 1.0),
        at: (1, 1)
    };

    bench.iter(|| {
        let _w: WPoint = cell.into();
    });

    let w_point: WPoint = cell.into();
    bench.bytes = size_of_val(&w_point) as u64;
}

benchmark_group!(zero_point, from_zero_point, into_zero_point);
benchmark_group!(one_point, from_one_point, into_one_point);
benchmark_main!(zero_point, one_point);
