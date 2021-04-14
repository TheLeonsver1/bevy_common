use super::input::plugin::PlayerInputPlugin;
use super::{events::*, systems::*};
use bevy::prelude::*;
pub struct NotTileMapPlugin;
impl Plugin for NotTileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(PlayerInputPlugin)
            .add_event::<PressedOnEntity>()
            .add_system(clicked_pickable_entity_system.system());
    }
}
