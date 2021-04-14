use bevy::prelude::*;
#[derive(Default)]
pub struct MouseWorldPosition {
    pub position: Vec2,
}
#[derive(Clone, Copy)]
pub struct CameraSpeed {
    pub speed: f32,
}
impl Default for CameraSpeed {
    fn default() -> Self {
        Self { speed: 2000.0 }
    }
}
