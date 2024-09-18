use cgmath::{Point3, Vector3};
use noise::{Perlin, NoiseFn};

use crate::{constants::world::{CHUNK_SIZE, CUBE_INDICES, OVERLAP}, graphics::cube::{create_single_tx_cube_vertices, Vertex}};

pub fn generate_flat_terrain(flat_height: i32, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>, texture_id: u32) {
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..= flat_height {  // Ensure the flat terrain is generated up to the specified height
                let offset = Vector3::new(x as f32, y as f32, z as f32);
                let cube_vertices = create_single_tx_cube_vertices(Point3::new(0.0, 0.0, 0.0), offset, texture_id);

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
    mountain_width: i32, 
    perlin: Perlin,
    scale: f64,
    height_scale: f64,
    extended_size: i32,
    upper_texture_id: u32,
    lower_texture_id: u32,
    lower_portion_height: i32
) {
    let mut height_map = vec![vec![0; extended_size as usize]; extended_size as usize];
    // Generate Perlin noise for the height map
    for x in 0..extended_size {
        for z in 0..extended_size {
            let world_x = (chunk_position.x * CHUNK_SIZE + x - OVERLAP) as f64;
            let world_z = (chunk_position.z * CHUNK_SIZE + z - OVERLAP) as f64;

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
                    
                    // Determine which texture to use
                    let texture_id = if y <= flat_height + lower_portion_height {
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

pub fn generate_unbalanced_terrain(
    chunk_position: Point3<i32>,
    flat_height: i32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    mountain_width: i32, 
    perlin: Perlin,
    scale: f64,
    height_scale: f64,
    extended_size: i32,
    texture_id_1: u32, // First texture ID
    texture_id_2: u32, // Second texture ID
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

    // Iterate over the chunk and alternate the textures
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let base_height = height_map[(x + OVERLAP) as usize][(z + OVERLAP) as usize];

            for dy in 0..mountain_width {
                let height = base_height + dy;

                if height > flat_height {
                    for y in (flat_height + 1)..=height {
                        if is_block_exposed(x, y, z, &height_map) {
                            let offset = Vector3::new(x as f32, y as f32, z as f32);

                            // Alternate textures based on position (e.g., checkerboard pattern)
                            let texture_id = if (x + z) % 2 == 0 { 
                                texture_id_1  // Use first texture for even sums of x + z
                            } else {
                                texture_id_2  // Use second texture for odd sums of x + z
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
