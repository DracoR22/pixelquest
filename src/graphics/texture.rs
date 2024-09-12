pub fn calculate_tile_uvs(tile_x: u32, tile_y: u32) -> [(f32, f32); 4] {
    let tile_size: f32 = 37.51;
    let atlas_size: f32 = 600.0;
    
    // Calculate the UV coordinates for the tile
    let u1: f32 = (tile_x as f32 * tile_size) / atlas_size;
    let v1: f32 = (tile_y as f32 * tile_size) / atlas_size;
    let u2: f32 = u1 + (tile_size / atlas_size);
    let v2: f32 = v1 + (tile_size / atlas_size);

    [
        (u1, v1), // Bottom-left
        (u2, v1), // Bottom-right
        (u2, v2), // Top-right
        (u1, v2), // Top-left
    ]
}