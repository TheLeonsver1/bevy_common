use bevy::prelude::*;
#[derive(Debug, Default)]
pub struct PathNodes {
    pub nodes: Vec<Vec2>,
}
impl PathNodes {
    pub fn fully_walkable() -> Self {
        Self {
            nodes: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.5, 0.5),
            ],
        }
    }
    pub fn only_corners() -> Self {
        Self {
            nodes: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
            ],
        }
    }
}
