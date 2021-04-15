use bevy::prelude::{Entity, Vec3};
///Name of the Entity
pub struct NameInGame(pub String);
#[derive(Default)]
pub struct TargetToMoveTo {
    pub target_position: Vec3,
}
pub struct TargetEntity {
    pub target_entity: Entity,
}
#[derive(Debug, Clone, Copy)]
pub struct CameraMoveSpeed {
    pub speed: f32,
}
impl Default for CameraMoveSpeed {
    fn default() -> Self {
        Self { speed: 800.0 }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CameraZoomSpeed {
    pub speed: f32,
}
impl Default for CameraZoomSpeed {
    fn default() -> Self {
        Self { speed: 2.0 }
    }
}
