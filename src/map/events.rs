use bevy::{input::mouse::MouseButton, prelude::Entity};
pub struct PressedOnEntity {
    pub entity: Entity,
    pub mouse_button: MouseButton,
}
