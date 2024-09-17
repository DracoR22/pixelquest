extern crate criterion;
extern crate pixelquest;
extern crate cgmath;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cgmath::{Point3, Vector3};
use pixelquest::{camera::camera::Camera, graphics::cube::create_single_tx_cube_vertices};

fn bench_block_vertices_generation(c: &mut Criterion) {
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 3.0),
        Vector3::new(0.0, 1.0, 0.0),
        -90.0,
        0.0,
    );

    let offset = Vector3::new(0.0, -3.0, 0.0);

    c.bench_function("block vertices generation", |b| {
        b.iter(|| create_single_tx_cube_vertices(black_box(camera.position), black_box(offset)))
    });
}

criterion_group!(benches, bench_block_vertices_generation);
criterion_main!(benches);