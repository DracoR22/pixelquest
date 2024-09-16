use cgmath::{Matrix4, Point3, Vector3};
use glium::{glutin::surface::WindowSurface, uniform, uniforms::Sampler, Surface};

use crate::{camera::camera::Camera, graphics::cube::FaceUVs};

use super::chunk::Chunk;

pub struct World {
    pub chunks: Vec<Chunk>,
    chunk_size: i32,
}

impl World {
    pub fn new(display: &glium::Display<WindowSurface>, uvs: &FaceUVs) -> Self {
        let mut chunks = Vec::new();
        let chunk_size = 16; // Define chunk size

        // Generate a 3x3 grid of chunks
        for x in -1..=1 {
            for z in -1..=1 {
                let chunk_position = Point3::new(x, 0, z);
                chunks.push(Chunk::new(display, uvs, chunk_position));
            }
        }

        World { chunks, chunk_size }
    }

    pub fn new_flat_world(display: &glium::Display<WindowSurface>, uvs: &FaceUVs, flat_height: i32) -> Self {
        let mut chunks = Vec::new();
        let chunk_size = 16;
    
        // Generate a 3x3 grid of flat chunks
        for x in -1..=1 {
            for z in -1..=1 {
                let chunk_position = Point3::new(x, 0, z);
                chunks.push(Chunk::new_flat(display, uvs, chunk_position, flat_height));
            }
        }
    
        World { chunks, chunk_size }
    }

    pub fn new_with_terrain(display: &glium::Display<WindowSurface>, uvs: &FaceUVs, flat_height: i32) -> Self {
        let mut chunks = Vec::new();
        let chunk_size = 16;

        // Generate a 3x3 grid of chunks with flat and mountainous terrain
        for x in -1..=1 {
            for z in -1..=1 {
                let chunk_position = Point3::new(x, 0, z);
                chunks.push(Chunk::new_with_terrain(display, uvs, chunk_position, flat_height));
            }
        }

        World { chunks, chunk_size }
    }

    pub fn render(&self, target: &mut glium::Frame, program: &glium::Program, camera: &Camera, perspective: Matrix4<f32>, sampler: Sampler<'_, glium::Texture2d>) {
        let view = camera.get_view_matrix();
        let light = [-1.0, 0.4, 0.9f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        for chunk in &self.chunks {
            let position_vector = Vector3::new(
                chunk.position.x as f32 * self.chunk_size as f32,
                chunk.position.y as f32 * self.chunk_size as f32,
                chunk.position.z as f32 * self.chunk_size as f32
            );
            let model = Matrix4::from_translation(position_vector);

            target
                .draw(
                    &chunk.vertex_buffer,
                    &chunk.index_buffer,
                    program,
                    &uniform! {
                        model: Into::<[[f32; 4]; 4]>::into(model),
                        view: Into::<[[f32; 4]; 4]>::into(view),
                        perspective: Into::<[[f32; 4]; 4]>::into(perspective),
                        u_light: light,
                        tex: sampler,
                    },
                    &params,
                )
                .unwrap();
        }
    }
}
