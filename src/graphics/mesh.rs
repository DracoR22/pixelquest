use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, normal, tex_coords);

pub fn create_cube_vertices(uvs: &[(f32, f32); 4], top_uvs: &[(f32, f32); 4]) -> [Vertex; 24] {
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

    let mut vertices = [Vertex {
        position: [0.0; 3],
        normal: [0.0; 3],
        tex_coords: [0.0, 0.0],
    }; 24];

    for i in 0..6 {
        for j in 0..4 {
            vertices[i * 4 + j] = Vertex {
                position: positions[i][j],
                normal: normals[i],
                tex_coords: if i == 2 { top_uvs[j].into() } else { uvs[j].into() },
            };
        }
    }

    vertices
}