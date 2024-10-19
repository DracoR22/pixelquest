use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct TriangleVertex {
    pub position: [f32; 2],
}
implement_vertex!(TriangleVertex, position);


pub fn create_triangle() -> Vec<TriangleVertex> {
    let vertex1 = TriangleVertex { position: [-0.5, -0.5] };
    let vertex2 = TriangleVertex { position: [ 0.0,  0.5] };
    let vertex3 = TriangleVertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    
    return shape;
}