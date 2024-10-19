use std::fs::read_to_string;

use cgmath::SquareMatrix;
use cgmath::{Matrix4, Vector3};
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::Surface;
use glium::uniform;

use crate::shapes::triangle::{create_triangle, TriangleVertex};
use crate::{camera::camera::Camera, graphics::texture::create_texture, world::world::World};

pub struct Renderer<'a> {
    cube_program: glium::Program,
    triangle_program: glium::Program,
    textures: Vec<glium::Texture2d>,
    params: glium::DrawParameters<'a>,
    triangle_vertex_buffer: glium::VertexBuffer<TriangleVertex>,
    triangle_indices: NoIndices,
}

impl Renderer<'_> {
    pub fn new(display: &glium::Display<WindowSurface>) -> Self {
        // Load shaders
        let vertex_shader_src = read_to_string("res/shaders/cube_vertex.glsl").expect("failed to read vertex shader");
        let fragment_shader_src = read_to_string("res/shaders/cube_fragment.glsl").expect("failed to read fragment shader");

        let triangle_vertex_shader_src = read_to_string("res/shaders/triangle_vertex.glsl").expect("failed to read vertex shader");
        let triangle_fragment_shader_src = read_to_string("res/shaders/triangle_fragment.glsl").expect("failed to read fragment shader");

        let cube_program = glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
        let triangle_program = glium::Program::from_source(display, &triangle_vertex_shader_src, &triangle_fragment_shader_src, None).unwrap();

        // Initialize textures
        let textures: Vec<glium::Texture2d> = vec![
            create_texture(display, include_bytes!("../../res/blocks/dark-grass.png")),
            create_texture(display, include_bytes!("../../res/blocks/light-grass.png")),
            create_texture(display, include_bytes!("../../res/blocks/light-sand.png")),
            create_texture(display, include_bytes!("../../res/blocks/rock-1.png")),
            create_texture(display, include_bytes!("../../res/blocks/brown.png")),
        ];

        // Initialize draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // triangle
        let shape = create_triangle();
        let triangle_vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let triangle_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        Renderer {
            cube_program,
            textures,
            params,
            triangle_program,
            triangle_vertex_buffer,
            triangle_indices
        }
    }

    pub fn render(&self, target: &mut glium::Frame, world: &World, camera: &Camera, perspective: Matrix4<f32>) {
        let view = camera.get_view_matrix();
        let light = [-1.0, 0.4, 0.9f32];

        for chunk in &world.chunks {
            let position_vector = Vector3::new(
                chunk.position.x as f32 * world.chunk_size as f32,
                chunk.position.y as f32 * world.chunk_size as f32,
                chunk.position.z as f32 * world.chunk_size as f32
            );
            let model = Matrix4::from_translation(position_vector);

            target
                .draw(
                    &chunk.vertex_buffer,
                    &chunk.index_buffer,
                    &self.cube_program,
                    &uniform! {
                        model: Into::<[[f32; 4]; 4]>::into(model),
                        view: Into::<[[f32; 4]; 4]>::into(view),
                        perspective: Into::<[[f32; 4]; 4]>::into(perspective),
                        u_light: light,
                        tex0: self.textures[0].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        tex1: self.textures[1].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        tex2: self.textures[2].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        tex3: self.textures[3].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        tex4: self.textures[4].sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                    },
                    &self.params,
                )
                .unwrap();
        }
        
        // draw triangle
        let triangle_model = Matrix4::from_translation(Vector3::new(2.0, 20.0, 5.0)); // Example transformation

target.draw(
    &self.triangle_vertex_buffer,
    &self.triangle_indices,
    &self.triangle_program,
    &uniform! {
        model: Into::<[[f32; 4]; 4]>::into(triangle_model),
        view: Into::<[[f32; 4]; 4]>::into(view),
        perspective: Into::<[[f32; 4]; 4]>::into(perspective),
    },
    &self.params,
).unwrap();
    }
}