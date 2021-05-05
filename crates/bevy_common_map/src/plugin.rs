use super::{events::*, systems::*};
use bevy::prelude::*;
use bevy_common_input::plugin::PlayerInputPlugin;
pub struct NotTileMapPlugin;
impl Plugin for NotTileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(PlayerInputPlugin)
            .add_event::<PressedOnEntity>()
            .add_system(clicked_pickable_entity_system.system());
    }
}
