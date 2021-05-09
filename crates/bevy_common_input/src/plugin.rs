use crate::events::MouseDragEvent;

use super::{resources::*, systems::*};
use bevy::prelude::*;
#[derive(StageLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub struct BevyCommonInputStage;
#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum InputSystemsLables {
    GetMouseWorldPos,
    MoveCameraWithWasd,
    CameraZoomInScrollWheel,
    MoveZoomableCameraWithWasd,
    TrackMouseDragging,
    ApplyCameraDragging,
}
///Base plugin for bevy_common_input, adds a stage before [CoreStage::Update](CoreStage::Update)
pub struct BevyCommonInputPlugin;
impl Plugin for BevyCommonInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_before(
            CoreStage::Update,
            BevyCommonInputStage,
            SystemStage::parallel(),
        );
    }
}
///This plugin adds the resource [MouseWorldPosition](MouseWorldPosition) to track the mouse's position in world coords
///
///Depends on: [BevyCommonInputPlugin](BevyCommonInputPlugin)
pub struct MouseWorldPositionPlugin;
impl Plugin for MouseWorldPositionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MouseWorldPosition::default())
            .add_system_to_stage(
                BevyCommonInputStage,
                get_mouse_world_position
                    .system()
                    .label(InputSystemsLables::GetMouseWorldPos),
            );
    }
}
///This plugin enables moving a camera that has the components:
///
///[MainCamera](bevy_common_core::marker_components::MainCamera)
///
///[CameraMoveSpeed](bevy_common_core::data_components::CameraMoveSpeed)
///
///movement is with WASD keys
///
///Depends on: [BevyCommonInputPlugin](BevyCommonInputPlugin)
pub struct MoveableUnzoomableCameraPlugin;
impl Plugin for MoveableUnzoomableCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            BevyCommonInputStage,
            move_camera_with_wasd
                .system()
                .label(InputSystemsLables::MoveCameraWithWasd),
        );
    }
}
///This plugin enables zooming on cameras that have the components:
///
///[MainCamera](bevy_common_core::marker_components::MainCamera)
///
///[CameraZoomSpeed](bevy_common_core::data_components::CameraZoomSpeed)
///
///[CameraZoomLimit](bevy_common_core::data_components::CameraZoomLimit)
///
///Depends on: [BevyCommonInputPlugin](BevyCommonInputPlugin)
pub struct ZoomableCameraPlugin;
impl Plugin for ZoomableCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            BevyCommonInputStage,
            zoom_in_camera_with_mouse_scroll
                .system()
                .label(InputSystemsLables::CameraZoomInScrollWheel),
        );
    }
}
///This plugin enables moving a zoomable camera with speed scaled by zoom
///
///Depends on [ZoomableCameraPlugin](ZoomableCameraPlugin), [BevyCommonInputPlugin](BevyCommonInputPlugin)
///
///Requires Component [CameraMoveSpeed](bevy_common_core::data_components::CameraMoveSpeed)
pub struct MoveableZoomableCameraPlugin;
impl Plugin for MoveableZoomableCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            BevyCommonInputStage,
            move_camera_with_wasd_scaled_by_zoom
                .system()
                .label(InputSystemsLables::MoveZoomableCameraWithWasd)
                .after(InputSystemsLables::CameraZoomInScrollWheel),
        );
    }
}
//TODO:Make this more generic on button drag
///This plugin enables tracking of mouse dragging(currently only middle mouse)
///
///Depends on:
///
///[BevyCommonInputPlugin](BevyCommonInputPlugin)
///
///[MouseWorldPositionPlugin](MouseWorldPositionPlugin)
pub struct MonitorMouseDraggingPlugin;
impl Plugin for MonitorMouseDraggingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<MouseDragEvent>();
        app.add_system_to_stage(
            BevyCommonInputStage,
            track_middle_mouse_dragging
                .system()
                .label(InputSystemsLables::TrackMouseDragging),
        );
    }
}
///This plugin enables moving the camera based on middle mouse button dragging
///
///Depends on: [MonitorMouseDraggingPlugin](MonitorMouseDraggingPlugin)
///
pub struct DraggableCameraPlugin;
impl Plugin for DraggableCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            BevyCommonInputStage,
            move_camera_with_middle_mouse_drag
                .system()
                .label(InputSystemsLables::TrackMouseDragging),
        );
    }
}
