use std::collections::HashMap;

use cgmath::{Point3, Vector3};
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::graphics::cube::{create_cube_vertices, FaceUVs, Vertex};

pub struct ChunkData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}



pub fn generate_chunk(uvs: &FaceUVs, camera_position: Point3<f32>) -> ChunkData {
    let chunk_size = 16;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Create a Perlin noise generator with a seed
    let seed: u32 = 42; // You can use any value or make it configurable
    let perlin = Perlin::new(seed);

    // Scaling factors for noise
    let scale = 0.05; // Adjust this for larger or smaller features
    let height_scale = 5.0; // Maximum height adjustment

    // Temporary array to hold the heights
    let mut height_map = vec![vec![0.0; chunk_size]; chunk_size];

    // First pass: Generate heights
    for x in 0..chunk_size {
        for z in 0..chunk_size {
            let noise_value = perlin.get([x as f64 * scale, z as f64 * scale]);
            let height = (noise_value * height_scale).round();
            height_map[x][z] = height as f32;
        }
    }

    // Second pass: Generate blocks based on height and fill gaps
    for x in 0..chunk_size {
        for z in 0..chunk_size {
            let height = height_map[x][z];

            for y in 0..height as u32 {
                let offset = Vector3::new(x as f32, y as f32, z as f32);

                // Generate vertices for this cube
                let cube_vertices = create_cube_vertices(uvs, camera_position, offset);

                // Add vertices to the main vector
                let base_index = vertices.len() as u32;
                vertices.extend_from_slice(&cube_vertices);

                // Generate indices for all cubes
                let cube_indices: Vec<u32> = CUBE_INDICES.iter()
                    .map(|&idx| idx as u32 + base_index)
                    .collect();
                indices.extend_from_slice(&cube_indices);
            }
        }
    }

    ChunkData {
        indices,
        vertices
    }
}

pub const CUBE_INDICES: [u16; 36] = [
    0,  1,  2,  2,  3,  0, // front
    4,  5,  6,  6,  7,  4, // back
    8,  9, 10, 10, 11,  8, // top
    12, 13, 14, 14, 15, 12, // bottom
    16, 17, 18, 18, 19, 16, // right
    20, 21, 22, 22, 23, 20  // left
];