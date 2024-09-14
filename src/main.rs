use glium::winit::event::{ElementState, MouseButton};
use glium::Surface;
use glium::{implement_vertex, uniform};
use minecraft_rust::camera::camera::Camera;
use minecraft_rust::graphics::cube::{create_cube_vertices, FaceUVs, Vertex};
use minecraft_rust::graphics::texture::{calculate_tile_uvs, init_uvs, UVS};
use minecraft_rust::shaders::shaders::{FRAGMENT_SHADER_SRC, VERTEX_SHADER_SRC};
use device_query::{DeviceQuery, DeviceState, Keycode};
use cgmath::{perspective, Deg, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
use minecraft_rust::world::chunk::{generate_chunk, CUBE_INDICES};


fn main() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("Minecraft").build(&event_loop);
    
    let mut current_mouse_position = (0.0, 0.0, 0.0);
    // initialize camera
    let mut camera = Camera::new(
        Point3::new(0.0, 0.0, 3.0),
        Vector3::new(0.0, 1.0, 0.0),
        -90.0,
        0.0,
    );

    // load images
    let image = image::load(std::io::Cursor::new(&include_bytes!("../assets/blocks/blocks.jpg")),
      image::ImageFormat::Jpeg).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::Texture2d::new(&display, image).unwrap();

    
    init_uvs();
    

    let uvs = UVS.get().unwrap();
    let offset = Vector3::new(0.0, -3.0, 0.0);
    
    // create cube
    // let vertices = create_cube_vertices(&uvs, camera.position, offset);
    // let vertices2 = create_cube_vertices(&uvs, camera.position, offset2);

    //     // Combine vertices into a single vector
    //     let mut combined_vertices = Vec::from(vertices);
    //     combined_vertices.extend_from_slice(&vertices2);

    let mut chunk_data = generate_chunk(uvs, camera.position);
    

    // Improve texture quality, idk if I see a change lol
    let sampler = glium::uniforms::Sampler::new(&texture)
        .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);

    // Create chunk buffers
    let mut vertex_buffer = glium::vertex::VertexBuffer::new(&display, &chunk_data.vertices).unwrap();
    let mut index_buffer = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &chunk_data.indices).unwrap();

    let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

    let device_state = DeviceState::new();
    let mut last_frame = std::time::Instant::now();

    // For mouse control
    let mut first_mouse = true;
    let mut last_x = 400.0;
    let mut last_y = 300.0;

    // Capture the cursor
    window.set_cursor_grab(glium::winit::window::CursorGrabMode::Confined).unwrap();
    window.set_cursor_visible(false);

    let _ = event_loop.run(move |event, window_target| {
        let current_frame = std::time::Instant::now();
        let delta_time = (current_frame - last_frame).as_secs_f32();
        last_frame = current_frame;

        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.53, 0.81, 0.92, 1.0), 1.0);

                    let model: Matrix4<f32> = Matrix4::identity();
                    let view = camera.get_view_matrix();
                    let (width, height) = target.get_dimensions();
                    let aspect_ratio = width as f32 / height as f32;
                    let perspective: Matrix4<f32> = perspective(Deg(45.0), aspect_ratio, 0.1, 100.0);

                    let light = [-1.0, 0.4, 0.9f32];

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        .. Default::default()
                    };
                    

                    target.draw(&vertex_buffer, &index_buffer, &program,
                        &uniform! { model: Into::<[[f32; 4]; 4]>::into(model),
                                    view: Into::<[[f32; 4]; 4]>::into(view),
                                    perspective: Into::<[[f32; 4]; 4]>::into(perspective),
                                    u_light: light, tex: sampler },
                        &params).unwrap();

                    target.finish().unwrap();
                },
                glium::winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left && state == ElementState::Pressed {
                        
                        println!("Left mouse clicked, {:?}", current_mouse_position);

                          // Hardcoded offset for the new cube
                        let offset = Vector3::new(0.0, 0.0, 0.0);

                        // Generate vertices for the new cube
                        let new_cube_vertices = create_cube_vertices(&uvs, camera.position, offset);

                        // Add new vertices to the chunk data
                        let base_index = chunk_data.vertices.len() as u32;
                        chunk_data.vertices.extend_from_slice(&new_cube_vertices);

                        // Generate and add indices for the new cube
                        let new_cube_indices: Vec<u32> = CUBE_INDICES.iter()
                            .map(|&idx| idx as u32 + base_index)
                            .collect();
                        chunk_data.indices.extend_from_slice(&new_cube_indices);

                        // Update vertex and index buffers
                        vertex_buffer = glium::vertex::VertexBuffer::new(&display, &chunk_data.vertices).unwrap();
                        index_buffer = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &chunk_data.indices).unwrap();
                    }
                },
                _ => (),
            },
            glium::winit::event::Event::DeviceEvent { event, .. } => match event {
                glium::winit::event::DeviceEvent::MouseMotion { delta } => {
                    let (x, y) = delta;
                    if first_mouse {
                        last_x = x as f32;
                        last_y = y as f32;
                        first_mouse = false;
                    }

                    current_mouse_position = (last_x + x as f32, last_y + y as f32, 0.0);

                    last_x += x as f32;
                    last_y += y as f32;

                    // println!("Mouse coords: ({}, {})", x, -y);

                    camera.process_mouse_movement(x as f32, -y as f32);
                },
                _ => (),            
            },
            glium::winit::event::Event::AboutToWait => {
                // Handle keyboard input
                let keys: Vec<Keycode> = device_state.get_keys();
                let camera_speed = 10.0 * delta_time;

                if keys.contains(&Keycode::W) {
                    camera.position += camera.front * camera_speed;
                }
                if keys.contains(&Keycode::S) {
                    camera.position -= camera.front * camera_speed;
                }
                if keys.contains(&Keycode::A) {
                    camera.position -= camera.right * camera_speed;
                }
                if keys.contains(&Keycode::D) {
                    camera.position += camera.right * camera_speed;
                }
                if keys.contains(&Keycode::W) && keys.contains(&Keycode::LShift) {
                    camera.position += camera.front * camera_speed * 2.0
                }
                

                // println!("Camera position: {:?}", camera.position);

                window.request_redraw();
            },
            _ => (),
        }
    });
}