use super::{
    data_components::{CameraMoveSpeed, CameraZoomSpeed},
    marker_components::MainCamera,
    resources::MouseWorldPosition,
};
use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseWheel},
    prelude::*,
};

///A function to get mouse position in world coords
///
///Credit to jamadazi's cheatbook and contributors for this function
pub fn get_mouse_world_position(
    mut cursor_move_events: EventReader<CursorMoved>,
    windows: Res<Windows>,
    query: Query<&Transform, With<MainCamera>>,
    mut mouse_world_position: ResMut<MouseWorldPosition>,
) {
    //Assuming there is exactly one main camera entity, so this is OK
    let camera_transform = query.iter().next().unwrap();

    for ev in cursor_move_events.iter() {
        //Get the size of the window that the event is for
        let wnd = windows.get(ev.id).unwrap();
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
    mut keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraMoveSpeed), With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, camera_speed)) = query.single_mut() {
        translate_by_wasd(&mut transform, &keyboard_input, camera_speed.speed, time);
    }
}
///Translates the entity with the W,A,S,D keys
pub fn translate_by_wasd(
    transform: &mut Transform,
    keyboard_input: &Res<Input<KeyCode>>,
    speed: f32,
    time: Res<Time>,
) {
    let mut dir = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::W) {
        dir += Vec2::new(0.0, 1.0)
    }
    if keyboard_input.pressed(KeyCode::S) {
        dir += Vec2::new(0.0, -1.0)
    }
    if keyboard_input.pressed(KeyCode::A) {
        dir += Vec2::new(-1.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::D) {
        dir += Vec2::new(1.0, 0.0)
    }

    let normalized = dir.normalize_or_zero();
    let position_to_move_to =
        transform.translation + normalized.extend(0.0) * time.delta_seconds() * speed;
    transform.translation = transform.translation.lerp(position_to_move_to, 0.4);
}
pub fn zoom_in_camera_with_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &CameraZoomSpeed), With<MainCamera>>,
    time: Res<Time>,
) {
    //TODO: Maybe have a resource that says whether to invert the y
    for ev in mouse_wheel_events.iter() {
        if let Ok((mut transform, camera_speed)) = query.single_mut() {
            //invert the scroll wheel's Y
            let inverted_y_scale = -1.0 * ev.y;
            //The Scale's offset should be the same on both x and y
            let offset = Vec3::new(inverted_y_scale, inverted_y_scale, 0.0);
            let mut new_scale =
                transform.scale + offset * camera_speed.speed * time.delta_seconds();
            new_scale = new_scale.clamp(Vec3::new(0.05, 0.05, 1.0), Vec3::new(1.0, 1.0, 1.0));
            transform.scale = new_scale;
        }
    }
}
