use super::{events::*, systems::*};
use bevy::prelude::*;
use bevy_common_input::plugin::MouseWorldPositionPlugin;
pub struct NotTileMapPlugin;
impl Plugin for NotTileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MouseWorldPositionPlugin)
            .add_event::<PressedOnEntity>()
            .add_system(clicked_pickable_entity_system.system());
    }
}
