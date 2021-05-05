///This entity's dimensions is a rect whose center is the entity's transform
#[derive(Debug, Default, Clone, Copy)]
pub struct RectFromTransform {
    pub width: u32,
    pub height: u32,
}
impl RectFromTransform {
    pub fn from_square_length(length: u32) -> Self {
        Self {
            width: length,
            height: length,
        }
    }
}
