use cgmath::{Point3, Vector3};
use pixelquest::{camera::camera::Camera, graphics::cube::create_single_tx_cube_vertices};

extern crate pixelquest;

#[test]
fn test_block_vertices_generation() {
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 3.0),
        Vector3::new(0.0, 1.0, 0.0),
        -90.0,
        0.0,
    );

    let offset = Vector3::new(0.0, -3.0, 0.0);
    
    create_single_tx_cube_vertices(camera.position, offset, 0);
}