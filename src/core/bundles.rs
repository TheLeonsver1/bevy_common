use bevy::prelude::*;

use super::{
    data_components::{CameraMoveSpeed, CameraZoomSpeed},
    marker_components::MainCamera,
};
#[derive(Debug, Default, Bundle)]
pub struct CommonCameraBundle {
    move_speed: CameraMoveSpeed,
    zoom_speed: CameraZoomSpeed,
}
#[derive(Debug, Bundle, Default)]
pub struct MainCameraBundle {
    #[bundle]
    common_camera: CommonCameraBundle,
    main_camera: MainCamera,
}
