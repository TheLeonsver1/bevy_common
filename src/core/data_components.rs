use bevy::prelude::{Entity, Vec3};
///Name of the Entity
pub struct NameInGame(pub String);
#[derive(Debug, Default)]
pub struct TargetToMoveTo {
    pub target_position: Vec3,
}
#[derive(Debug)]
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
///The Camera's Zooming settings
#[derive(Debug, Clone, Copy)]
pub struct CameraZoomLimit {
    ///This Vec3's x and y values should be bigger than max_zoom's. zoom is smaller scale
    pub min_zoom: Vec3,
    ///This Vec3's x and y values should be smaller than min_zoom's. zoom is smaller scale
    pub max_zoom: Vec3,
}
impl Default for CameraZoomLimit {
    fn default() -> Self {
        Self {
            min_zoom: Vec3::new(1.0, 1.0, 1.0),
            max_zoom: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}
