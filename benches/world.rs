#[macro_use]
extern crate bencher;
use bencher::Bencher;
use cellular_automaton::world::World;
use std::mem::size_of_val;

fn one_one_get_cells(bench: &mut Bencher) {
    let mut world = World::new(1, 1, 1.0);
    world.mirror_edge(1);

    bench.iter(|| {
        world.get_cells();
    });

    bench.bytes = size_of_val(&world) as u64;
}

fn hundred_one_get_cells(bench: &mut Bencher) {
    let mut world = World::new(100, 100, 1.0);
    world.mirror_edge(1);

    bench.iter(|| {
        world.get_cells();
    });

    bench.bytes = size_of_val(&world) as u64;
}

fn hundred_three_get_cells(bench: &mut Bencher) {
    let mut world = World::new(100, 100, 1.0);
    world.mirror_edge(3);

    bench.iter(|| {
        world.get_cells();
    });

    bench.bytes = size_of_val(&world) as u64;
}

fn one_one_update(bench: &mut Bencher) {
    let mut world = World::new(1, 1, 1.0);
    world.mirror_edge(1);

    bench.iter(|| {
        world.next(|_, _| None);
    });

    bench.bytes = size_of_val(&world) as u64;
}

fn hundred_one_update(bench: &mut Bencher) {
    let mut world = World::new(100, 100, 1.0);
    world.mirror_edge(1);

    bench.iter(|| {
        world.next(|_, _| None);
    });

    bench.bytes = size_of_val(&world) as u64;
}

fn hundred_three_update(bench: &mut Bencher) {
    let mut world = World::new(100, 100, 1.0);
    world.mirror_edge(3);

    bench.iter(|| {
        world.next(|_, _| None);
    });

    bench.bytes = size_of_val(&world) as u64;
}

benchmark_group!(get_cells, one_one_get_cells, hundred_one_get_cells);
benchmark_group!(mirror_edge_get_cells, hundred_one_get_cells, hundred_three_get_cells);
benchmark_group!(update, one_one_update, hundred_one_update);
benchmark_group!(mirror_edge_update, hundred_one_update, hundred_three_update);
benchmark_main!(get_cells, mirror_edge_get_cells, update, mirror_edge_update);