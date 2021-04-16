use super::{
    data_components::{CameraMoveSpeed, CameraZoomLimit, CameraZoomSpeed},
    events::MouseDragEvent,
    marker_components::MainCamera,
    resources::MouseWorldPosition,
};
use bevy::{input::mouse::MouseWheel, prelude::*};

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
///This function moves the camera based on W,A,S,D keys movement
pub fn move_camera_with_wasd(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraMoveSpeed), With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, camera_speed)) = query.single_mut() {
        translate_by_wasd(&mut transform, &keyboard_input, camera_speed.speed, time);
    }
}
///Moves the [MainCamera](MainCamera) with the W,A,S,D keys, scaled by the camera's [CameraZoomLimit](CameraZoomLimit)
pub fn move_camera_with_wasd_scaled_by_zoom(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraMoveSpeed, &CameraZoomLimit), With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, camera_speed, camera_zoom_limit)) = query.single_mut() {
        let dist_from_max = transform.scale.y - camera_zoom_limit.max_zoom.y;
        let percent = dist_from_max / (camera_zoom_limit.min_zoom.y - camera_zoom_limit.max_zoom.y);
        translate_by_wasd(
            &mut transform,
            &keyboard_input,
            camera_speed.speed * f32::max(0.15, percent),
            time,
        );
    }
}
///Translates an entity's transform with the W,A,S,D keys
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
    //We need to get the direction, and so we need to normalize it
    let normalized = dir.normalize_or_zero();
    //Get the position we want to move to
    let position_to_move_to =
        transform.translation + normalized.extend(0.0) * time.delta_seconds() * speed;
    //Lerp to the next best position so to not get jerky movement
    transform.translation = transform.translation.lerp(position_to_move_to, 0.4);
}
///This function zooms the view based on the [MainCamera](MainCamera)'s setting and the user's mouse wheel input
pub fn zoom_in_camera_with_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &CameraZoomSpeed, &CameraZoomLimit), With<MainCamera>>,
    time: Res<Time>,
) {
    //TODO: Maybe have a resource that says whether to invert the y
    for ev in mouse_wheel_events.iter() {
        if let Ok((mut transform, camera_speed, camera_zoom_limit)) = query.single_mut() {
            //invert the scroll wheel's Y
            let inverted_y_scale = -1.0 * ev.y;
            //The Scale's offset should be the same on both x and y
            let offset = Vec3::new(inverted_y_scale, inverted_y_scale, 0.0);
            let mut new_scale =
                transform.scale + offset * camera_speed.speed * time.delta_seconds();
            new_scale = new_scale.clamp(camera_zoom_limit.max_zoom, camera_zoom_limit.min_zoom);
            transform.scale = new_scale;
        }
    }
}
///This function checks if the user is dragging the mouse while holding the mouse button
///
///and sends [MouseDragEvent](MouseDragEvent) based on that movement
pub fn track_middle_mouse_dragging(
    mouse_input: Res<Input<MouseButton>>,
    mut dragging: Local<bool>,
    mut last_position: Local<Vec2>,
    mouse_position: Res<MouseWorldPosition>,
    mut mouse_drag_event_writer: EventWriter<MouseDragEvent>,
) {
    //If we are dragging
    if *dragging {
        //If we just released reset
        if mouse_input.just_released(MouseButton::Middle) {
            *dragging = false;
        }
        //Get the movement vector
        let movement_vector = mouse_position.position - *last_position;
        //Send the event
        mouse_drag_event_writer.send(MouseDragEvent { movement_vector });
        //Update our last_position
        *last_position = mouse_position.position;
    } else {
        //If we just pressed the middle mouse button
        if mouse_input.just_pressed(MouseButton::Middle) {
            //Make sure we know in the next frame we are dragging
            *dragging = true;
            //Set our last position to the current position of our mouse
            *last_position = mouse_position.position;
        }
    }
}
///This function moves the camera based on how the mouse was drgged while holding the middle mouse button
pub fn move_camera_with_middle_mouse_drag(
    mut query: Query<&mut Transform, With<MainCamera>>,
    mut mouse_drag_event_reader: EventReader<MouseDragEvent>,
) {
    if let Some(drag_event) = mouse_drag_event_reader.iter().next() {
        //we need to invert the mouse's movement because that's how most apps do it
        let inverted = drag_event.movement_vector * -1.0;
        if let Ok(mut camera_transform) = query.single_mut() {
            //Lerp to the next best position for the camera
            camera_transform.translation = camera_transform
                .translation
                .lerp(camera_transform.translation + inverted.extend(0.0), 0.4);
        }
    }
}
