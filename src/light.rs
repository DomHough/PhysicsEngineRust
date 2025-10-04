use crate::color::Color;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct PointLight {
    pub position: Vec3,
    pub color: Color
}

impl PointLight {
    pub fn new(position: Vec3, color: Color) -> Self {
        PointLight { position, color }
    }
}