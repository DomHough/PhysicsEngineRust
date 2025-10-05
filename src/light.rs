use crate::color::Color;
use crate::vec3::Vec3;

#[derive(Debug)]
pub enum LightSource { PointLight(PointLight) }

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct PointLight {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32, // new intensity scalar
}

impl PointLight {
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        PointLight { position, color, intensity }
    }
}