use bevy::prelude::*;

use super::{data_components::RectFromTransform, marker_components::Pickable};
#[derive(Bundle, Default)]
pub struct PickableBundle {
    pub rect_from_transform: RectFromTransform,
    pub _pickable: Pickable,
}
