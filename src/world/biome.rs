use cgmath::{Point3, Vector3};
use noise::{NoiseFn, Perlin};

use crate::{constants::world::CUBE_INDICES, graphics::cube::{create_single_tx_cube_vertices, Vertex}};

use super::chunk::{Biome, ChunkData};

pub struct BiomeGenerator {
   pub biome_noise: Perlin,
   pub elevation_noise: Perlin,
}

impl BiomeGenerator {
    pub fn new(seed: u32) -> Self {
        BiomeGenerator {
            biome_noise: Perlin::new(seed),
            elevation_noise: Perlin::new(seed + 1),
        }
    }

    pub fn get_biome_and_elevation(&self, x: f64, z: f64) -> (Biome, f64) {
        let biome_scale = 0.005;
        let elevation_scale = 0.01;

        let biome_value = self.biome_noise.get([x * biome_scale, z * biome_scale]);
        let elevation_value = self.elevation_noise.get([x * elevation_scale, z * elevation_scale]);

        let biome = if biome_value < -0.3 {
            Biome::Desert
        } else if biome_value < 0.3 {
            Biome::Plains
        } else {
            Biome::Mountains
        };

        let base_elevation = match biome {
            Biome::Desert => 64.0 + elevation_value * 10.0,
            Biome::Plains => 68.0 + elevation_value * 15.0,
            Biome::Mountains => 80.0 + elevation_value * 50.0,
        };

        (biome, base_elevation)
    }
}

pub fn generate_chunk(chunk_position: Point3<i32>, flat_height: i32) -> ChunkData {
    let biome_generator = BiomeGenerator::new(142);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate height map for the chunk
    let mut height_map = vec![vec![0; 16]; 16];
    for x in 0..16 {
        for z in 0..16 {
            let world_x = (chunk_position.x * 16 as i32 + x as i32) as f64;
            let world_z = (chunk_position.z * 16 as i32 + z as i32) as f64;
            let (_, elevation) = biome_generator.get_biome_and_elevation(world_x, world_z);
            height_map[x][z] = elevation.max(flat_height as f64) as i32;
        }
    }

    // Generate terrain based on height map
    for x in 0..16 {
        for z in 0..16 {
            let height = height_map[x][z];
            for y in 0..=height {
                if is_block_exposed(x, y, z, &height_map) {
                    let world_x = (chunk_position.x * 16 as i32 + x as i32) as f64;
                    let world_z = (chunk_position.z * 16 as i32 + z as i32) as f64;
                    let (biome, _) = biome_generator.get_biome_and_elevation(world_x, world_z);
                    
                    let texture_id = match biome {
                        Biome::Desert => 2,
                        Biome::Plains => 0,
                        Biome::Mountains => if y > flat_height + 10 { 3 } else { 0 },
                    };

                    add_block(x, y, z, texture_id, &mut vertices, &mut indices);
                }
            }
        }
    }

    ChunkData {
        indices,
        vertices,
    }
}

pub fn is_block_exposed(x: usize, y: i32, z: usize, height_map: &Vec<Vec<i32>>) -> bool {
    if y == 0 { return false; }  // Bottom layer is never exposed
    if y > height_map[x][z] { return false; }  // Above the surface
    if x > 0 && height_map[x-1][z] > y { return true; }
    if x < 16-1 && height_map[x+1][z] > y { return true; }
    if z > 0 && height_map[x][z-1] > y { return true; }
    if z < 16-1 && height_map[x][z+1] > y { return true; }
    y == height_map[x][z]  // Top block is always exposed
}

pub fn add_block(x: usize, y: i32, z: usize, texture_id: u32, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let offset = Vector3::new(x as f32, y as f32, z as f32);
    let cube_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), offset, texture_id);

    let base_index = vertices.len() as u32;
    vertices.extend_from_slice(&cube_vertices);

    let cube_indices: Vec<u32> = CUBE_INDICES.iter()
        .map(|&idx| idx as u32 + base_index)
        .collect();
    indices.extend_from_slice(&cube_indices);
}