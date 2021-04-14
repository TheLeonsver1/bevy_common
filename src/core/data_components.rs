use bevy::prelude::{Entity, Vec3};
///Name of the Entity
pub struct NameInGame(pub String);
#[derive(Default)]
pub struct TargetToMoveTo {
    pub target_position: Vec3,
}
pub struct TargetEntity {
    pub target_entity: Entity,
}
