use super::{marker_components::MainCamera, resources::CameraSpeed, resources::MouseWorldPosition};
use bevy::prelude::*;

///A function to get mouse position in world coords
///
///Credit to jamadazi's cheatbook and contributors for this function
pub fn get_mouse_world_position(
    mut evr_cursor: EventReader<CursorMoved>,
    wnds: Res<Windows>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut mouse_world_position: ResMut<MouseWorldPosition>,
) {
    //Assuming there is exactly one main camera entity, so this is OK
    let camera_transform = q_camera.iter().next().unwrap();

    for ev in evr_cursor.iter() {
        //Get the size of the window that the event is for
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        //The default orthographic projection is in pixels from the center;
        //Just undo the translation
        let p = ev.position - size / 2.0;

        //Apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        mouse_world_position.position = Vec2::new(pos_wld.x, pos_wld.y);
    }
}
///A function to move the camera with the W,A,S,D keys
pub fn move_camera_with_wasd(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraSpeed), With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, camera_speed)) = query.single_mut() {
        translate_by_wasd(&mut transform, input, camera_speed.speed, time);
    }
}
pub fn translate_by_wasd(
    transform: &mut Transform,
    input: Res<Input<KeyCode>>,
    speed: f32,
    time: Res<Time>,
) {
    let mut dir = Vec2::new(0.0, 0.0);
    if input.pressed(KeyCode::W) {
        dir += Vec2::new(0.0, 1.0);
    }
    if input.pressed(KeyCode::S) {
        dir += Vec2::new(0.0, -1.0);
    }
    if input.pressed(KeyCode::A) {
        dir += Vec2::new(-1.0, 0.0);
    }
    if input.pressed(KeyCode::D) {
        dir += Vec2::new(1.0, 0.0);
    }
    let normalized = dir.normalize_or_zero();
    transform.translation += normalized.extend(0.0) * time.delta_seconds() * speed;
}
