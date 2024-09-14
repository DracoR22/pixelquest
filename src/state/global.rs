use std::sync::OnceLock;

use cgmath::Vector3;

pub static CAMERA_VIEW: OnceLock<Vector3<f32>> = OnceLock::new();

pub fn initialize_state() -> &'static Vector3<f32> {
    CAMERA_VIEW.get_or_init(|| Vector3::new(0.0, 0.0, 0.0))
}