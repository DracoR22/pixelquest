# ![windows_subsystem = "windows"]

use glium::winit::event::{ElementState, MouseButton};
use glium::Surface;
use pixelquest::camera::camera::Camera;
use pixelquest::constants::world::{CHUNK_SIZE, CUBE_INDICES};
use pixelquest::graphics::cube::create_single_tx_cube_vertices;
use pixelquest::graphics::texture::{calculate_tile_uvs, create_texture, init_uvs, UVS};
use pixelquest::shaders::shaders::{FRAGMENT_SHADER_SRC, VERTEX_SHADER_SRC};
use device_query::{DeviceQuery, DeviceState, Keycode};
use cgmath::{perspective, Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
use pixelquest::world::chunk::{generate_chunk, Chunk, ChunkData};

use pixelquest::world::world::World;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("Pixel Quest").build(&event_loop);
    
    let mut current_mouse_position = (0.0, 0.0, 0.0);

    // initialize camera
    let mut camera = Camera::new(
        Point3::new(0.0, 0.0, 3.0),
        Vector3::new(0.0, 1.0, 0.0),
        -90.0,
        0.0,
    );

    // init_uvs();
    // let uvs =  UVS.get().and_then(|map| map.get("dark_grass")).cloned().expect("No uvs found");

    let offset = Vector3::new(0.0, 0.0, 0.0);

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

    let mut world = World::new(&display);

    let textures: Vec<glium::Texture2d> = vec![
    create_texture(&display, include_bytes!("../res/blocks/dark-grass.png")),
    create_texture(&display, include_bytes!("../res/blocks/light-grass.png")),
    create_texture(&display, include_bytes!("../res/blocks/light-sand.png")),
    create_texture(&display, include_bytes!("../res/blocks/rock-1.png")),
    create_texture(&display, include_bytes!("../res/blocks/brown.png")),
];
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
                    // Start drawing the frame
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.2510, 0.4745, 0.9608, 1.0), 1.0);

                    // target.clear_color_and_depth((0.0, 0.3176, 1.0, 1.0), 1.0);

                    // Get window dimensions and aspect ratio
                    let (width, height) = target.get_dimensions();
                    let aspect_ratio = width as f32 / height as f32;
                
                    // Create a perspective projection matrix
                    let perspective: Matrix4<f32> = perspective(Deg(45.0), aspect_ratio, 0.1, 100.0);
                
                    // Update world based on the camera's current position (for infinite terrain generation)
                    world.update(camera.position, &display); // <- Add this to update the world
                
                    // Render the world with the updated camera and perspective
                    world.render(&mut target, &program, &camera, perspective, &textures);
                
                    // Finalize drawing and display the frame
                    target.finish().unwrap();
                },
                glium::winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left && state == ElementState::Pressed {
                     // Use the camera's current position to spawn the cube
                     let cube_position: Point3<f32> = camera.position;
                     let cube_position_vec: Vector3<f32> = Vector3::new(camera.position.x, camera.position.y, 0.0);

                    // Access the chunks from World struct
                     for chunk in world.chunks.iter_mut() {
                    // Check if the cube's position belongs to this chunk
                    // if is_position_in_chunk(cube_position_vec, chunk) {
                    // Generate new cube vertices
                   
                    let new_cube_vertices = create_single_tx_cube_vertices(cube_position, offset, 1);
        
                    // Add vertices to chunk's vertex list
                   let base_index = chunk.chunk_data.vertices.len() as u32;
                   chunk.chunk_data.vertices.extend_from_slice(&new_cube_vertices);
        
                    // Generate indices for the new cube and add them to the chunk
                   let new_cube_indices: Vec<u32> = CUBE_INDICES.iter()
                   .map(|&idx| idx as u32 + base_index)
                   .collect();
                   chunk.chunk_data.indices.extend_from_slice(&new_cube_indices);
        
                   // Recreate the vertex and index buffers for the chunk
                  chunk.vertex_buffer = glium::VertexBuffer::new(&display, &chunk.chunk_data.vertices).unwrap();
                  chunk.index_buffer = glium::IndexBuffer::new(
                &display,
                glium::index::PrimitiveType::TrianglesList,
                &chunk.chunk_data.indices
                ).unwrap();
        
                println!("Added a new cube at camera position {:?}", cube_position);
                break;
                 }}
                    // }
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

fn is_position_in_chunk(position: Vector3<f32>, chunk: &Chunk) -> bool {
    let chunk_x = chunk.position.x as f32 * 16 as f32;
    let chunk_y = chunk.position.y as f32 * 16 as f32;
    let chunk_z = chunk.position.z as f32 * 16 as f32;

    position.x >= chunk_x && position.x < chunk_x + 16 as f32 &&
    position.y >= chunk_y && position.y < chunk_y + 16 as f32 &&
    position.z >= chunk_z && position.z < chunk_z + 16 as f32
}
