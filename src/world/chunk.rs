use std::collections::HashMap;

use cgmath::{Point3, Vector3};
use glium::{glutin::surface::WindowSurface, uniforms::Sampler};
use noise::{NoiseFn, Perlin};

use crate::graphics::cube::{create_single_tx_cube_vertices, FaceUVs, Vertex};

const CHUNK_SIZE: i32 = 16;
const OVERLAP: i32 = 1; // Amount of overlap with neighboring chunks

enum Biome {
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

     // Generate the flat base layer
     for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..=flat_height {  // Ensure the flat terrain is generated up to the specified height
                let offset = Vector3::new(x as f32, y as f32, z as f32);
                let cube_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), offset, 0);

                let base_index = vertices.len() as u32;
                vertices.extend_from_slice(&cube_vertices);

                let cube_indices: Vec<u32> = CUBE_INDICES.iter()
                    .map(|&idx| idx as u32 + base_index)
                    .collect();
                indices.extend_from_slice(&cube_indices);
            }
        }
    }

    // Generate the mountainous terrain
    generate_mountainous_terrain(chunk_position, flat_height, &mut vertices, &mut indices, 2,  perlin, scale, height_scale, extended_size, 0);

    match biome {
        Biome::Plains => {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    for y in 0..=flat_height {  // Ensure the flat terrain is generated up to the specified height
                        let offset = Vector3::new(x as f32, y as f32, z as f32);
                        let cube_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), offset, 0);
        
                        let base_index = vertices.len() as u32;
                        vertices.extend_from_slice(&cube_vertices);
        
                        let cube_indices: Vec<u32> = CUBE_INDICES.iter()
                            .map(|&idx| idx as u32 + base_index)
                            .collect();
                        indices.extend_from_slice(&cube_indices);
                    }
                }
            }
        
        }
        Biome::Mountains => {
            generate_mountainous_terrain(chunk_position, flat_height, &mut vertices, &mut indices, 2,  perlin, 0.05, 20.0, extended_size, 0);
        }    
        Biome::Desert => {
            generate_mountainous_terrain(chunk_position, flat_height, &mut vertices, &mut indices, 2,  perlin, 0.05, 20.0, extended_size, 0);
        }
    }
    
    ChunkData {
        indices,
        vertices,
    }
}

pub fn generate_mountainous_terrain(
    chunk_position: Point3<i32>,
    flat_height: i32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    mountain_width: i32, 
    perlin: Perlin,
    scale: f64,
    height_scale: f64,
    extended_size: i32,
    texture_id: u32
) {
    let mut height_map = vec![vec![0; extended_size as usize]; extended_size as usize];
    // Generate Perlin noise for the height map
    for x in 0..extended_size {
        for z in 0..extended_size {
            let world_x = (chunk_position.x * CHUNK_SIZE + x - OVERLAP) as f64;
            let world_z = (chunk_position.z * CHUNK_SIZE + z - OVERLAP) as f64;

            let noise_value = perlin.get([world_x * scale, world_z * scale]);
            let height = (noise_value * height_scale).round() as i32 + flat_height; // Added flat terrain height
            height_map[x as usize][z as usize] = height;
        }
    }

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let base_height = height_map[(x + OVERLAP) as usize][(z + OVERLAP) as usize];

            for dy in 0..mountain_width {
                let height = base_height + dy;

                if height > flat_height {
                    for y in (flat_height + 1)..=height {
                        if is_block_exposed(x, y, z, &height_map) {
                            let offset = Vector3::new(x as f32, y as f32, z as f32);
                            let cube_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), offset, texture_id);

                            let base_index = vertices.len() as u32;
                            vertices.extend_from_slice(&cube_vertices);

                            let cube_indices: Vec<u32> = CUBE_INDICES.iter()
                                .map(|&idx| idx as u32 + base_index)
                                .collect();
                            indices.extend_from_slice(&cube_indices);
                        }
                    }
                }
            }
        }
    }
}

fn is_block_exposed(x: i32, y: i32, z: i32, height_map: &Vec<Vec<i32>>) -> bool {
    let check_positions = [
        (x, z), // Current position
        (x, z - 1), (x, z + 1), // North, South
        (x - 1, z), (x + 1, z), // West, East
    ];

    let map_size = height_map.len() as i32;

    for (nx, nz) in check_positions.iter() {
        let hm_x = (nx + OVERLAP).clamp(0, map_size - 1) as usize;
        let hm_z = (nz + OVERLAP).clamp(0, map_size - 1) as usize;

        // Check if the block is exposed from the top or sides
        if y > height_map[hm_x][hm_z] {
            return true;
        }
    }

    false
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
    }

    pub fn new_flat(display: &glium::Display<WindowSurface>, position: Point3<i32>, flat_height: i32) -> Self {
        let chunk_data = generate_flat_chunk(position, flat_height);

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

pub fn generate_biome_for_chunk(chunk_position: Point3<i32>) -> Biome {
    let biome_noise = Perlin::new(100);  // Seed for biome noise
    let scale = 0.05;  // Control size of biome regions
    let noise_value = biome_noise.get([chunk_position.x as f64 * scale, chunk_position.z as f64 * scale]);

    if noise_value < -0.3 {
        Biome::Desert
    } else if noise_value < 0.3 {
        Biome::Plains
    } else {
        Biome::Mountains
    }
}


pub fn generate_flat_chunk(chunk_position: Point3<i32>, flat_height: i32) -> ChunkData {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Create a flat chunk with a specified height for all blocks
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..=flat_height {
                // Only render blocks that are exposed (surface blocks in this case)
                if y == flat_height {
                    let offset = Vector3::new(x as f32, y as f32, z as f32);
                    let cube_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), offset, 0);

                    let base_index = vertices.len() as u32;
                    vertices.extend_from_slice(&cube_vertices);

                    let cube_indices: Vec<u32> = CUBE_INDICES.iter()
                        .map(|&idx| idx as u32 + base_index)
                        .collect();
                    indices.extend_from_slice(&cube_indices);
                }
            }
        }
    }

    ChunkData {
        indices,
        vertices,
    }
}
