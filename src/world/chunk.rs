use std::collections::HashMap;

use cgmath::{Point3, Vector3};

use crate::graphics::cube::{create_cube_vertices, FaceUVs, Vertex};

pub fn generate_chunk(uvs:&FaceUVs, camera_position: Point3<f32>) -> (Vec<Vertex>, Vec<u32>) {
    let chunk_size = 16;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for x in 0..chunk_size {
        for z in 0..chunk_size {
            // Calculate offset for each cube in the chunk
            let offset = Vector3::new(x as f32, -3.0, z as f32);
            
            // Generate vertices for this cube
            let cube_vertices = create_cube_vertices(uvs, camera_position, offset);
            
            // Add vertices to the main vector
            let base_index = vertices.len() as u32;
            vertices.extend_from_slice(&cube_vertices);

            // Generate indices for this cube
            let cube_indices: Vec<u32> = CUBE_INDICES.iter()
                .map(|&idx| idx as u32 + base_index)
                .collect();
            indices.extend_from_slice(&cube_indices);
        }
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