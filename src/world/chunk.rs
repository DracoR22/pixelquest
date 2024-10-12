use std::collections::HashMap;

use cgmath::{Point3, Vector3};
use glium::{glutin::surface::WindowSurface, uniforms::Sampler};
use noise::{NoiseFn, Perlin};

use crate::graphics::cube::{create_single_tx_cube_vertices, FaceUVs, Vertex};

use super::terrain::{generate_arch_mountain_terrain, generate_flat_terrain, generate_mountainous_terrain, generate_spiral_mountain_terrain, generate_terrain_chunk, generate_trees};

const CHUNK_SIZE: i32 = 16;
const OVERLAP: i32 = 1; // Amount of overlap with neighboring chunks

pub enum Biome {
    Plains,
    Mountains,
    Desert,
    // Forest,
    // Ocean,
    // Tundra,
}


pub struct ChunkData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub fn generate_chunk(chunk_position: Point3<i32>, flat_height: i32) -> ChunkData {
    let biome = generate_biome_for_chunk(chunk_position);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let seed: u32 = 142;
    let perlin = Perlin::new(seed);

    let scale = 0.1; // Adjust the scale for frequency of mountains
    let height_scale = 1.0; // Taller mountains

    // Generate an extended height map for mountainous terrain
    let extended_size = CHUNK_SIZE + 2 * OVERLAP;

    let mut extended_height_map: Vec<Vec<f64>> = vec![vec![0.0; extended_size as usize]; extended_size as usize];

     // Generate the flat base layer
     generate_flat_terrain(flat_height, &mut vertices, &mut indices, 4);

       // Generate the mountainous terrain
    // generate_mountainous_terrain(chunk_position, flat_height, &mut vertices, &mut indices, 2,  perlin, scale, height_scale, extended_size, 0);
    match biome {
        Biome::Plains => {
        //    generate_flat_terrain(flat_height + 1, &mut vertices, &mut indices, 0);
          generate_terrain_chunk(
           chunk_position, 
           &mut vertices,  
           &mut indices,
           &perlin,       
        0.01,          
    10.0,          
    1,              
    0,              
);

// generate_arch_mountain_terrain(chunk_position, flat_height, &mut vertices, &mut indices,  2, 0.01, 0.0, extended_size, 0, 4, 10.0, 1);
// generate_spiral_mountain_terrain(chunk_position, flat_height, &mut vertices, &mut indices, 2, 0.01, 3.0, extended_size, 0, 3,  5.0, 1.0);
        generate_trees(
            chunk_position,
            flat_height,
            &mut vertices,
            &mut indices,
            &perlin,
            0.1, // tree density (adjust as needed)
            13,    // tree height
            4,
            0
        );

        generate_mountainous_terrain(chunk_position, flat_height, &mut vertices, &mut indices,  perlin, 0.01, 60.0, extended_size, 0, 4, 10);
       
        }
        // Biome::Mountains => {
        //     generate_flat_terrain(flat_height + 1, &mut vertices, &mut indices, 0);
        //     generate_terrain_chunk(
        //         chunk_position, // Chunk position in the world
        //         &mut vertices,  // Vertex buffer to store the chunk vertices
        //         &mut indices,   // Index buffer to store the chunk indices
        //         &perlin,        // Perlin noise instance
        //         0.01,            // Lower noise scale for more detailed hills
        //         10.0,           // Higher height scale for more dramatic height differences
        //         1,              // Higher base height to lift the terrain off the ground more
        //         0,              // Texture ID for terrain blocks
        //     );
        //     generate_trees(
        //         chunk_position,
        //         flat_height,
        //         &mut vertices,
        //         &mut indices,
        //         &perlin,
        //         0.1, // tree density (adjust as needed)
        //         13,    // tree height
        //         4,
        //         0
        //     );
        //     generate_mountainous_terrain(chunk_position, flat_height, &mut vertices, &mut indices,  perlin, 0.03, 60.0, extended_size, 0, 3, 10);
        // }    
        // Biome::Desert => {
        //       // Generate the mountainous terrain
        //     generate_flat_terrain(flat_height + 1, &mut vertices, &mut indices, 2);
        //     generate_terrain_chunk(
        //         chunk_position, // Chunk position in the world
        //         &mut vertices,  // Vertex buffer to store the chunk vertices
        //         &mut indices,   // Index buffer to store the chunk indices
        //         &perlin,        // Perlin noise instance
        //         0.01,            // Lower noise scale for more detailed hills
        //         10.0,           // Higher height scale for more dramatic height differences
        //         1,              // Higher base height to lift the terrain off the ground more
        //         2,              // Texture ID for terrain blocks
        //     );
        //     generate_mountainous_terrain(chunk_position, flat_height, &mut vertices, &mut indices,  perlin, 0.02, 20.0, extended_size, 2, 2, 2);
        // }

        _ => ()
    }
    
    ChunkData {
        indices,
        vertices,
    }
}

pub struct Chunk {
   pub vertex_buffer: glium::VertexBuffer<Vertex>,
   pub index_buffer: glium::IndexBuffer<u32>,
   pub position: Point3<i32>, // Chunk position in world space
   pub chunk_data: ChunkData,
}

impl Chunk {
    pub fn new(display: &glium::Display<WindowSurface>, position: Point3<i32>) -> Self {
        let flat_height = 0; // Define a flat terrain height
        let chunk_data = generate_chunk(position, flat_height); // Generate chunk with both flat terrain and mountains

        // create vertex and index buffer we got from chunk data struct
        let vertex_buffer = glium::VertexBuffer::new(display, &chunk_data.vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &chunk_data.indices,
        )
        .unwrap();

        Chunk {
            vertex_buffer,
            index_buffer,
            position,
            chunk_data
        }
    }}

pub fn generate_biome_for_chunk(chunk_position: Point3<i32>) -> Biome {
    let biome_noise = Perlin::new(100);  // Seed for biome noise
    let scale = 0.05;  // Control size of biome regions
    let scaled_x = chunk_position.x as f64 * scale;
    let scaled_z = chunk_position.z as f64 * scale;

    // Generate smooth transitions between biomes using noise
    let noise_value = biome_noise.get([scaled_x, scaled_z]);

    if noise_value < -0.3 {
        Biome::Plains
    }
    // } else if noise_value < 0.3 {
    //     Biome::Plains
    // } 
    else {
        Biome::Plains
    }
}