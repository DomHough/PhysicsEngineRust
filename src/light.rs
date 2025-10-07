use crate::color::Color;
use crate::vec3::Vec3;

pub(crate) trait Light {
    fn position(&self) -> &Vec3;
    fn color(&self) -> &Color;
    fn intensity(&self) -> f32;
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct PointLight {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32, // new intensity scalar
}

impl PointLight {
    pub(crate) fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        PointLight { position, color, intensity }
    }
}
impl Light for PointLight {
    fn position(&self) -> &Vec3 { &self.position }
    fn color(&self) -> &Color { &self.color }
    fn intensity(&self) -> f32 { self.intensity }
}

pub(crate) struct AmbientLight {
    pub color: Color,
    pub intensity: f32,
}

impl AmbientLight {
    pub fn new(color: Color, intensity: f32) -> Self {
        AmbientLight { color, intensity }
    }
}