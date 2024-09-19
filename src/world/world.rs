use cgmath::{Matrix4, Point3, Vector3};
use glium::{glutin::surface::WindowSurface, uniform, uniforms::Sampler, Surface};

use crate::{camera::camera::Camera, graphics::cube::FaceUVs};

use super::chunk::Chunk;

pub struct World {
    pub chunks: Vec<Chunk>,
    chunk_size: i32,
    last_camera_chunk_position: Point3<i32>, // Track the last chunk position
    chunk_radius: i32,  
}

impl World {
    pub fn new(display: &glium::Display<WindowSurface>) -> Self {
        let mut chunks: Vec<Chunk> = Vec::new();
        let chunk_size = 16; // Define chunk size
        let grid_size = 13;  // Define grid size
        let chunk_radius = 5; // Set how far chunks will be generated around the camera

        // Calculate the starting and ending positions for the grid
        let half_grid_size = grid_size / 2;

        // Generate a grid of chunks initially
        for x in -half_grid_size..=half_grid_size {
            for z in -half_grid_size..=half_grid_size {
                let chunk_position = Point3::new(x, 0, z);
                chunks.push(Chunk::new(display, chunk_position));
            }
        }

        World {
            chunks,
            chunk_size,
            last_camera_chunk_position: Point3::new(0, 0, 0), // Initial position
            chunk_radius,
        }
    }

    pub fn update(&mut self, camera_position: Point3<f32>, display: &glium::Display<WindowSurface>) {
        // Convert the camera's world position to chunk coordinates
        let camera_chunk_x = (camera_position.x / self.chunk_size as f32).floor() as i32;
        let camera_chunk_z = (camera_position.z / self.chunk_size as f32).floor() as i32;
        let camera_chunk_position = Point3::new(camera_chunk_x, 0, camera_chunk_z);

        // Check if the camera has moved to a new chunk
        if camera_chunk_position != self.last_camera_chunk_position {
            // Generate and load new chunks around the new camera chunk position
            self.generate_chunks_around(display, camera_chunk_position);

            // Unload far chunks
            self.unload_distant_chunks(camera_chunk_position);

            // Update the last known camera chunk position
            self.last_camera_chunk_position = camera_chunk_position;
        }
    }

    // Generate chunks in a radius around the camera's current chunk
    fn generate_chunks_around(
        &mut self,
        display: &glium::Display<WindowSurface>,
        camera_chunk_position: Point3<i32>,
    ) {
        for x in -self.chunk_radius..=self.chunk_radius {
            for z in -self.chunk_radius..=self.chunk_radius {
                let chunk_position = Point3::new(
                    camera_chunk_position.x + x,
                    0,
                    camera_chunk_position.z + z,
                );

                // Check if the chunk already exists
                if !self.chunk_exists(chunk_position) {
                    self.chunks.push(Chunk::new(display, chunk_position));
                }
            }
        }
    }

    // Helper function to check if a chunk already exists at a given position
    fn chunk_exists(&self, chunk_position: Point3<i32>) -> bool {
        self.chunks.iter().any(|chunk| chunk.position == chunk_position)
    }

    // Unload chunks that are far away from the camera
    fn unload_distant_chunks(&mut self, camera_chunk_position: Point3<i32>) {
        self.chunks.retain(|chunk| {
            let distance_x = (chunk.position.x - camera_chunk_position.x).abs();
            let distance_z = (chunk.position.z - camera_chunk_position.z).abs();
            distance_x <= self.chunk_radius && distance_z <= self.chunk_radius
        });
    }

    pub fn render(&self, target: &mut glium::Frame, program: &glium::Program, camera: &Camera, perspective: Matrix4<f32>, textures: &Vec<glium::Texture2d>) {
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
                        tex0: textures[0].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        tex1: textures[1].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        tex2: textures[2].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        tex3: textures[3].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        tex4: textures[4].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        fog_color: [0.7, 0.85, 1.0f32],  // Slightly bluer, closer to sky color
                        fog_start: 50.0f32,  // Increased from 5.0
                        fog_end: 150.0f32,   // Increased from 60.0
                    },
                    &params,
                )
                .unwrap();
        }
    }
}
