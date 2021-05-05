use bevy::prelude::*;

use super::{data_components::RectFromTransform, marker_components::Pickable};
#[derive(Default, Bundle)]
pub struct PickableBundle {
    pub rect_from_transform: RectFromTransform,
    pub _pickable: Pickable,
}
