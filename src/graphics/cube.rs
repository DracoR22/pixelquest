use cgmath::{Point3, Vector3};
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub texture_id: u32
}
implement_vertex!(Vertex, position, normal, tex_coords, texture_id);

#[derive(Clone)]
pub struct FaceUVs {
    pub front: [(f32, f32); 4],
    pub back: [(f32, f32); 4],
    pub top: [(f32, f32); 4],
    pub bottom: [(f32, f32); 4],
    pub right: [(f32, f32); 4],
    pub left: [(f32, f32); 4],
}

// pub fn create_multiple_tx_cube_vertices(uvs: &FaceUVs, camera_position: Point3<f32>, offset: Vector3<f32>) -> [Vertex; 24] {
//     let positions = [
//         // Front face
//         [[-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]],
//         // Back face
//         [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]],
//         // Top face
//         [[-0.5, 0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]],
//         // Bottom face
//         [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5]],
//         // Right face
//         [[0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5]],
//         // Left face
//         [[-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5]],
//     ];

//     let normals = [
//         [0.0, 0.0, 1.0],   // Front face
//         [0.0, 0.0, -1.0],  // Back face
//         [0.0, 1.0, 0.0],   // Top face
//         [0.0, -1.0, 0.0],  // Bottom face
//         [1.0, 0.0, 0.0],   // Right face
//         [-1.0, 0.0, 0.0],  // Left face
//     ];

//     let mut vertices = [Vertex {
//         position: [0.0; 3],
//         normal: [0.0; 3],
//         tex_coords: [0.0, 0.0],
//     }; 24];

//     for i in 0..6 {
//         let uvs = match i {
//             0 => uvs.front,
//             1 => uvs.back,
//             2 => uvs.top,
//             3 => uvs.bottom,
//             4 => uvs.right,
//             5 => uvs.left,
//             _ => unreachable!(),
//         };

//         for j in 0..4 {
            
//              // Adjust the cube's position relative to the camera
//              let pos = positions[i][j];
//              let adjusted_position = [
//                  pos[0] + camera_position[0] + offset.x,
//                  pos[1] + camera_position[1] +  offset.y,
//                  pos[2] + camera_position[2] + offset.z,
//              ];

//             vertices[i * 4 + j] = Vertex {
//                 position: adjusted_position,
//                 normal: normals[i],
//                 tex_coords: uvs[j].into(),
//             };
//         }
//     }

//     vertices
// }

pub fn create_single_tx_cube_vertices(camera_position: Point3<f32>, offset: Vector3<f32>, texture_id: u32) -> [Vertex; 24] {
    let positions = [
        // Front face
        [[-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]],
        // Back face
        [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]],
        // Top face
        [[-0.5, 0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]],
        // Bottom face
        [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5]],
        // Right face
        [[0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5]],
        // Left face
        [[-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5]],
    ];

    let normals = [
        [0.0, 0.0, 1.0],   // Front face
        [0.0, 0.0, -1.0],  // Back face
        [0.0, 1.0, 0.0],   // Top face
        [0.0, -1.0, 0.0],  // Bottom face
        [1.0, 0.0, 0.0],   // Right face
        [-1.0, 0.0, 0.0],  // Left face
    ];

    let tex_coords = [
        [0.0, 1.0], // Top-left
        [1.0, 1.0], // Top-right
        [0.0, 0.0], // Bottom-left
        [1.0, 0.0], // Bottom-right
    ];

    let mut vertices = [Vertex {
        position: [0.0; 3],
        normal: [0.0; 3],
        tex_coords: [0.0, 0.0],
        texture_id: 0
    }; 24];

    for i in 0..6 {
        let normal = normals[i];

        for j in 0..4 {
            // Adjust the cube's position relative to the camera
            let pos = positions[i][j];
            let adjusted_position = [
                pos[0] + camera_position[0] + offset.x,
                pos[1] + camera_position[1] + offset.y,
                pos[2] + camera_position[2] + offset.z,
            ];

            vertices[i * 4 + j] = Vertex {
                position: adjusted_position,
                normal,
                tex_coords: tex_coords[j],
                texture_id
            };
        }
    }

    vertices
}