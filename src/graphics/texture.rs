use std::{cell::OnceCell, collections::HashMap, sync::OnceLock};

use glium::{glutin::surface::WindowSurface, Display};

use super::cube::FaceUVs;

pub fn calculate_tile_uvs(tile_x: u32, tile_y: u32) -> [(f32, f32); 4] {
    let tile_size: f32 = 64.0;
    let atlas_size: f32 = 1024.0;
    
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


pub static UVS: OnceLock<HashMap<String, FaceUVs>> = OnceLock::new();

pub fn init_uvs() {
    UVS.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("grass".to_string(), FaceUVs {
            front: calculate_tile_uvs(3, 15),
            back: calculate_tile_uvs(3, 15),
            top: calculate_tile_uvs(0, 16),
            bottom: calculate_tile_uvs(2, 16),
            right: calculate_tile_uvs(3, 15),
            left: calculate_tile_uvs(3, 15),
        });
        map.insert("light_grass".to_string(), FaceUVs {
            front: calculate_tile_uvs(13, 2),
            back: calculate_tile_uvs(13, 2),
            top: calculate_tile_uvs(13, 2),
            bottom: calculate_tile_uvs(13, 2),
            right: calculate_tile_uvs(13, 2),
            left: calculate_tile_uvs(13, 2),
        });
        map.insert("dark_grass".to_string(), FaceUVs {
            front: calculate_tile_uvs(13, 0),
            back: calculate_tile_uvs(13, 0),
            top: calculate_tile_uvs(13, 0),
            bottom: calculate_tile_uvs(13, 0),
            right: calculate_tile_uvs(13, 0),
            left: calculate_tile_uvs(13, 0),
        });
        map
    });
}


pub fn get_uvs(name: &str) -> Option<FaceUVs> {
    UVS.get().and_then(|map| map.get(name)).cloned()
}

pub fn create_block_texture(display: &Display<WindowSurface>, ) -> glium::Texture2d {
let image = image::load(std::io::Cursor::new(&include_bytes!("../../res/blocks/dark-grass.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
  let image_dimensions = image.dimensions();
  let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
  let texture = glium::Texture2d::new(display, image).unwrap();
   
  texture
}