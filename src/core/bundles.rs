use bevy::prelude::*;

use super::{
    data_components::{CameraMoveSpeed, CameraZoomLimit, CameraZoomSpeed},
    marker_components::MainCamera,
};
#[derive(Debug, Default, Bundle)]
pub struct CommonCameraBundle {
    pub move_speed: CameraMoveSpeed,
    pub zoom_speed: CameraZoomSpeed,
    pub zoom_limit: CameraZoomLimit,
}
#[derive(Debug, Bundle, Default)]
pub struct MainCameraBundle {
    #[bundle]
    pub common_camera: CommonCameraBundle,
    pub main_camera: MainCamera,
}
