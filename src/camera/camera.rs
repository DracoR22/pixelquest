use cgmath::{InnerSpace, Matrix4, Point3, Vector3};

pub struct Camera {
   pub position: Point3<f32>,
   pub front: Vector3<f32>,
   pub up: Vector3<f32>,
   pub right: Vector3<f32>,
   pub world_up: Vector3<f32>,
   pub yaw: f32,
   pub pitch: f32,
   pub sensitivity: f32,
}

impl Camera {
   pub fn new(position: Point3<f32>, up: Vector3<f32>, yaw: f32, pitch: f32) -> Self {
        let mut camera = Camera {
            position,
            front: Vector3::new(0.0, 0.0, -1.0),
            up,
            right: Vector3::new(0.0, 0.0, 0.0),
            world_up: up,
            yaw,
            pitch,
            sensitivity: 0.05,
        };
        camera.update_camera_vectors();
        camera
    }

   pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }

   pub fn update_camera_vectors(&mut self) {
        let front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ).normalize();

        self.front = front;
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

   pub fn process_mouse_movement(&mut self, xoffset: f32, yoffset: f32) {
        self.yaw += xoffset * self.sensitivity;
        self.pitch += yoffset * self.sensitivity;

        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        // Update Front, Right and Up Vectors using the updated Euler angles
        self.update_camera_vectors();
    }
    
}