use cgmath::Point3;
use noise::Perlin;

use crate::graphics::cube::Vertex;

pub struct GenerateMountaniousTerrainParams {
    pub chunk_position: Point3<i32>,
    pub flat_height: i32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub mountain_width: i32, 
    pub perlin: Perlin,
    pub scale: f64,
    pub height_scale: f64,
    pub extended_size: i32,
    pub base_texture_id: u32,
    pub top_texture_id: u32,
    pub top_layer_thickness: i32
}