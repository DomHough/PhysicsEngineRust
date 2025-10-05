use crate::color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub shininess: f32,
}

impl Material {
    pub fn new(ambient: Color, diffuse: Color, specular: Color, shininess: f32) -> Self {
        Material { ambient, diffuse, specular, shininess }
    }
}

impl Default for Material {
    fn default() -> Self {
        // A soft blue default
        Material {
            ambient: Color::new(0.0, 0.0, 0.4, 1.0),
            diffuse: Color::new(0.0, 0.0, 0.7, 1.0),
            specular: Color::new(1.0, 1.0, 1.0, 1.0),
            shininess: 32.0,
        }
    }
}