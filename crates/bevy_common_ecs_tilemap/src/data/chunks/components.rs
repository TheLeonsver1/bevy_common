use bevy::{prelude::*, utils::HashMap};
pub struct ChunkPathingNodes {
    pub node_spatial_map: HashMap<UVec2, Vec<Vec2>>,
}
