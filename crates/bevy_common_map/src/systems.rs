use super::{
    data_components::RectFromTransform, events::PressedOnEntity, marker_components::Pickable,
};
use bevy::prelude::*;
use bevy_common_core::marker_components::MainCamera;
use bevy_common_input::resources::MouseWorldPosition;

///A naive 2d solution for mouse picking, based on rects
pub fn clicked_pickable_entity_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut pressed_on_ent_events: EventWriter<PressedOnEntity>,
    mouse_world_position: Res<MouseWorldPosition>,
    q_pickable: Query<(Entity, &Transform, &RectFromTransform), With<Pickable>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left)
        || mouse_button_input.just_pressed(MouseButton::Right)
    {
        for (entity, transform, rect) in q_pickable.iter() {
            if mouse_world_position.position.x < transform.translation.x + rect.width as f32 / 2.
                && mouse_world_position.position.y
                    < transform.translation.y + rect.height as f32 / 2.
                && mouse_world_position.position.x
                    > transform.translation.x - rect.width as f32 / 2.
                && mouse_world_position.position.y
                    > transform.translation.y - rect.height as f32 / 2.
            {
                if mouse_button_input.just_pressed(MouseButton::Right) {
                    pressed_on_ent_events.send(PressedOnEntity {
                        entity,
                        mouse_button: MouseButton::Right,
                    })
                } else if mouse_button_input.just_pressed(MouseButton::Left) {
                    pressed_on_ent_events.send(PressedOnEntity {
                        entity,
                        mouse_button: MouseButton::Left,
                    })
                }
            }
        }
    }
}
///A naive frustum culling solution
pub fn naive_frustum_culling(
    query: Query<&Transform, With<MainCamera>>,
    mut map_features: Query<(&mut Visible, &Transform), With<Pickable>>,
) {
    if let Ok(camera_transform) = query.single() {
        let camera_transform_z_zero = Vec3::new(
            camera_transform.translation.x,
            camera_transform.translation.y,
            0.0,
        );
        for (mut visible, transform) in map_features.iter_mut() {
            if transform.translation.distance(camera_transform_z_zero) > 1500.0 {
                visible.is_visible = false;
            } else {
                //count+=1;
                if !visible.is_visible {
                    visible.is_visible = true;
                }
            }
        }
    }
}
