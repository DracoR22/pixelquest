use cgmath::{Point3, Vector3};
use noise::{Perlin, NoiseFn};
use rand::Rng;

use crate::{constants::world::{CHUNK_SIZE, CUBE_INDICES, OVERLAP}, graphics::cube::{create_single_tx_cube_vertices, Vertex}};

pub struct Terrain {
    pub chunk_position: Point3<i32>,
    pub flat_height: i32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

}

impl Terrain {
    pub fn new(flat_height: i32, vertices: Vec<Vertex>, indices: Vec<u32>, chunk_position: Point3<i32>) -> Self {
       Terrain {
        flat_height,
        indices, 
        vertices,
        chunk_position
       }
    }

    pub fn generate_flat_terrain(&mut self, add_height: Option<i32>, texture_id: u32) {
        let height = self.flat_height + add_height.unwrap_or(0);
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 0..= height {  // Ensure the flat terrain is generated up to the specified height
                    let offset = Vector3::new(x as f32, y as f32, z as f32);
                    let cube_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), offset, texture_id);
    
                    // generate vertices and indices for each vertex
                    let base_index = self.vertices.len() as u32;
                    self.vertices.extend_from_slice(&cube_vertices);
    
                    let cube_indices: Vec<u32> = CUBE_INDICES.iter()
                        .map(|&idx| idx as u32 + base_index)
                        .collect();
                    self.indices.extend_from_slice(&cube_indices);
                }
            }}
    }
}

pub fn generate_flat_terrain(flat_height: i32, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>, texture_id: u32) {
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..= flat_height {  // Ensure the flat terrain is generated up to the specified height
                let offset = Vector3::new(x as f32, y as f32, z as f32);
                let cube_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), offset, texture_id);

                // generate vertices and indices for each vertex
                let base_index = vertices.len() as u32;
                vertices.extend_from_slice(&cube_vertices);

                let cube_indices: Vec<u32> = CUBE_INDICES.iter()
                    .map(|&idx| idx as u32 + base_index)
                    .collect();
                indices.extend_from_slice(&cube_indices);
            }
        }}
}

pub fn generate_mountainous_terrain(
    chunk_position: Point3<i32>,
    flat_height: i32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    perlin: Perlin,
    scale: f64,
    height_scale: f64,
    extended_size: i32,
    upper_texture_id: u32,
    lower_texture_id: u32,
    _lower_portion_height: i32, // No longer used for texture selection
) {
    let mut height_map = vec![vec![0; extended_size as usize]; extended_size as usize];
    let mut rng = rand::thread_rng(); // Initialize random number generator

    // Generate Perlin noise for the height map
    for x in 0..extended_size {
        for z in 0..extended_size {
            let world_x = (chunk_position.x * CHUNK_SIZE + x) as f64;
            let world_z = (chunk_position.z * CHUNK_SIZE + z) as f64;

            let noise_value = perlin.get([world_x * scale, world_z * scale]);
            let height = (noise_value * height_scale).round() as i32 + flat_height;
            height_map[x as usize][z as usize] = height;
        }
    }

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let base_height = height_map[(x + OVERLAP) as usize][(z + OVERLAP) as usize];

            for y in (flat_height + 1)..=base_height {
                if is_block_exposed(x, y, z, &height_map) {
                    let offset = Vector3::new(x as f32, y as f32, z as f32);

                    // Randomly determine which texture to use
                    let random_chance: f32 = rng.gen(); // Generate a random float between 0.0 and 1.0
                    let texture_id = if random_chance < 0.2 {
                        // 20% chance to apply the lower texture
                        lower_texture_id
                    } else {
                        upper_texture_id
                    };

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

pub fn generate_spiral_mountain_terrain(
    chunk_position: Point3<i32>,
    flat_height: i32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    mountain_width: i32,
    scale: f64,
    height_scale: f64,
    extended_size: i32,
    texture_id_1: u32,  // First texture ID
    texture_id_2: u32,  // Second texture ID
    spiral_factor: f64, // Control how tight the spiral is
    num_rings: f64      // Control the number of spiral rings
) {
    let mut height_map = vec![vec![0; extended_size as usize]; extended_size as usize];

    // First loop: generate height map using Perlin noise and spiral calculation
    for x in 0..extended_size {
        for z in 0..extended_size {
            let world_x = (chunk_position.x * CHUNK_SIZE + x - OVERLAP) as f64;
            let world_z = (chunk_position.z * CHUNK_SIZE + z - OVERLAP) as f64;

            // Convert to polar coordinates to create a spiral effect
            let radius = (world_x.powi(2) + world_z.powi(2)).sqrt() * scale;

            // Introduce the num_rings factor to control how many spirals occur
            let angle = world_x.atan2(world_z) + (spiral_factor * radius) / num_rings;

            // Control the height using a sinusoidal function based on the angle (creates waves)
            let height = (angle.sin() * height_scale).round() as i32 + flat_height;

            height_map[x as usize][z as usize] = height;
        }
    }

    // Second loop: generate cubes using the height map
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let base_height = height_map[(x + OVERLAP) as usize][(z + OVERLAP) as usize];

            for dy in 0..mountain_width {
                let height = base_height + dy;

                if height > flat_height {
                    for y in (flat_height + 1)..=height {
                        if is_block_exposed(x, y, z, &height_map) {
                            let offset = Vector3::new(x as f32, y as f32, z as f32);

                            // Recalculate world_x, world_z, radius, and angle
                            let world_x = (chunk_position.x * CHUNK_SIZE + x - OVERLAP) as f64;
                            let world_z = (chunk_position.z * CHUNK_SIZE + z - OVERLAP) as f64;
                            let radius = (world_x.powi(2) + world_z.powi(2)).sqrt() * scale;
                            let angle = world_x.atan2(world_z) + (spiral_factor * radius) / num_rings;

                            // Alternate textures based on the angle of the spiral
                            let texture_id = if angle.sin() > 0.0 {
                                texture_id_1  // Use first texture for positive angles
                            } else {
                                texture_id_2  // Use second texture for negative angles
                            };

                            // Generate cube vertices with the selected texture
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


pub fn generate_trees(
    chunk_position: Point3<i32>,
    flat_height: i32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    perlin: &Perlin,
    tree_density: f64,
    tree_height: i32,
    trunk_texture_id: u32,
    leaf_texture_id: u32
) {
    let tree_scale = 0.05; // Adjust this to change the distribution of trees

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let world_x = (chunk_position.x * CHUNK_SIZE + x) as f64;
            let world_z = (chunk_position.z * CHUNK_SIZE + z) as f64;

            // Use Perlin noise to determine if a tree should be placed
            let noise_value = perlin.get([world_x * tree_scale, world_z * tree_scale]);

            if noise_value > 1.0 - tree_density {
                // Use flat_height as the base height for all trees
                let base_height = flat_height;

                // We don't need to check if the location is suitable since it's flat terrain
                generate_tree(
                    x as f32,
                    base_height as f32,
                    z as f32,
                    tree_height,
                    vertices,
                    indices,
                    trunk_texture_id,
                    leaf_texture_id
                );
            }
        }
    }
}

fn generate_tree(
    x: f32,
    y: f32,
    z: f32,
    height: i32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    trunk_texture_id: u32,
    leaf_texture_id: u32
) {
    // Generate trunk
    for i in 0..height {
        let offset = Vector3::new(x, y + i as f32, z);
        let cube_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), offset, trunk_texture_id);
        let base_index = vertices.len() as u32;
        vertices.extend_from_slice(&cube_vertices);
        let cube_indices: Vec<u32> = CUBE_INDICES.iter()
            .map(|&idx| idx as u32 + base_index)
            .collect();
        indices.extend_from_slice(&cube_indices);
    }

    // Generate spherical leaves
    let leaf_center = Vector3::new(x, y + height as f32, z);
    let leaf_radius = 3.0;  // Set the radius for the spherical canopy

    // Loop over a cube that encompasses the leaf sphere
    for dx in -leaf_radius as i32..=leaf_radius as i32 {
        for dy in -leaf_radius as i32..=leaf_radius as i32 {
            for dz in -leaf_radius as i32..=leaf_radius as i32 {
                // Calculate the position of the current leaf cube
                let leaf_pos = leaf_center + Vector3::new(dx as f32, dy as f32, dz as f32);

                // Calculate the distance from the leaf center
                let distance = ((dx * dx + dy * dy + dz * dz) as f32).sqrt();

                // Only place leaf cubes if they are within the radius of the sphere
                if distance <= leaf_radius {
                    let leaf_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), leaf_pos, leaf_texture_id);
                    let base_index = vertices.len() as u32;
                    vertices.extend_from_slice(&leaf_vertices);
                    let leaf_indices: Vec<u32> = CUBE_INDICES.iter()
                        .map(|&idx| idx as u32 + base_index)
                        .collect();
                    indices.extend_from_slice(&leaf_indices);
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

pub fn generate_terrain_chunk(
    chunk_position: Point3<i32>,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    perlin: &Perlin,
    scale: f64,
    height_scale: f64,
    base_height: i32,
    texture_id: u32,
) {
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let world_x = (chunk_position.x * CHUNK_SIZE + x) as f64;
            let world_z = (chunk_position.z * CHUNK_SIZE + z) as f64;

            // Get noise value for terrain height at this (x, z)
            let noise_value = perlin.get([world_x * scale, world_z * scale]);
            
            // Map noise value (-1.0 to 1.0) to a terrain height (e.g., 0 to 30 blocks)
            let terrain_height = ((noise_value + 1.0) / 2.0 * height_scale) as i32 + base_height;

            // Generate blocks for terrain
            for y in 0..terrain_height {
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