use cgmath::{Point3, Vector3};

use crate::graphics::cube::{create_cube_vertices, FaceUVs, Vertex};

pub fn generate_world(num_cubes: usize, uvs: &FaceUVs, camera_position: Point3<f32>) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..num_cubes {
        // Calculate offset for each cube
        let offset = Vector3::new(i as f32 * 2.0 - (num_cubes as f32 - 1.0), -3.0, 0.0);
        
        // Generate vertices for this cube
        let cube_vertices = create_cube_vertices(uvs, camera_position, offset);
        
        // Add vertices to the main vector
        vertices.extend_from_slice(&cube_vertices);

        // Generate indices for this cube
        let cube_indices: Vec<u16> = CUBE_INDICES.iter()
            .map(|&idx| idx + (i * 24) as u16)
            .collect();
        indices.extend_from_slice(&cube_indices);
    }

    (vertices, indices)
}

const CUBE_INDICES: [u16; 36] = [
    0,  1,  2,  2,  3,  0, // front
    4,  5,  6,  6,  7,  4, // back
    8,  9, 10, 10, 11,  8, // top
    12, 13, 14, 14, 15, 12, // bottom
    16, 17, 18, 18, 19, 16, // right
    20, 21, 22, 22, 23, 20  // left
];