use cgmath::Vector3;

use glium::{glutin::surface::WindowSurface, implement_vertex, index::PrimitiveType, Display, IndexBuffer, VertexBuffer};

use crate::{graphics::cube::Vertex, world::world::World};

// #[derive(Copy, Clone)]
// pub struct Vertex {
//     pub position: [f32; 3],
//     pub normal: [f32; 3],
//     pub tex_coords: [f32; 2],
//     pub texture_id: u32
// }
// implement_vertex!(Vertex, position, normal, tex_coords, texture_id);

pub fn create_cube(world_pos: Vector3<f32>, texture_id: u32) -> (Vec<Vertex>, Vec<u32>) {
    // let mut world = World::new(&display);

    let vertices: [Vertex; 24] = [
         // Front face
        Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0], texture_id },
        Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 0.0], texture_id },
        Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 1.0], texture_id },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0], texture_id },
        
        // Back face
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0], texture_id },
        Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0], texture_id },
        Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0], texture_id },
        Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0], texture_id },
    
        // Top face
        Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0], texture_id },
        Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0], texture_id },
        Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0], texture_id },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0], texture_id },
    
        // Bottom face
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0], texture_id },
        Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0], texture_id },
        Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0], texture_id },
        Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0], texture_id },
    
        // Right face
        Vertex { position: [ 0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0], texture_id },
        Vertex { position: [ 0.5, -0.5,  0.5], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0], texture_id },
        Vertex { position: [ 0.5,  0.5,  0.5], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 1.0], texture_id },
        Vertex { position: [ 0.5,  0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 1.0], texture_id },
    
        // Left face
        Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 0.0], texture_id },
        Vertex { position: [-0.5, -0.5,  0.5], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0], texture_id },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 1.0], texture_id },
        Vertex { position: [-0.5,  0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 1.0], texture_id },
        ];

        let indices: [u32; 36] = [
            0,  1,  2,  2,  3,  0, // front
            4,  5,  6,  6,  7,  4, // back
            8,  9, 10, 10, 11,  8, // top
            12, 13, 14, 14, 15, 12, // bottom
            16, 17, 18, 18, 19, 16, // right
            20, 21, 22, 22, 23, 20  // left
        ];

        // Apply the world position offset to the vertices
       let transformed_vertices: Vec<Vertex> = vertices
       .iter()
       .map(|v| Vertex {
         position: [
            v.position[0] + world_pos.x,
            v.position[1] + world_pos.y,
            v.position[2] + world_pos.z,
         ],
         normal: v.normal,
         tex_coords: v.tex_coords,
         texture_id: v.texture_id,
        })
       .collect();

      (transformed_vertices, indices.to_vec())
}