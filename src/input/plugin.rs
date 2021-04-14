use super::{resources::*, systems::*};
use bevy::prelude::*;
pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(MouseWorldPosition::default())
            .add_system(get_mouse_world_position.system());
    }
}
