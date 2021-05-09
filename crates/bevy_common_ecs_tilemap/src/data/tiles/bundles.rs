use super::components::*;
use bevy::prelude::*;
#[derive(Debug, Bundle)]
pub struct WalkableTileBundle {
    nodes: PathNodes,
}
