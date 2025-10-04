use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        assert!(r >= 0.0 && r <= 1.0, "Red component out of range: {}", r);
        assert!(g >= 0.0 && g <= 1.0, "Green component out of range: {}", g);
        assert!(b >= 0.0 && b <= 1.0, "Blue component out of range: {}", b);
        assert!(a >= 0.0 && a <= 1.0, "Alpha component out of range: {}", a);
        Color { r, g, b, a }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, scalar: f32) -> Color {
        Color {
            r: (self.r * scalar).clamp(0.0, 1.0),
            g: (self.g * scalar).clamp(0.0, 1.0),
            b: (self.b * scalar).clamp(0.0, 1.0),
            a: self.a, // alpha is not affected by multiplication
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            r: (self.r * other.r).clamp(0.0, 1.0),
            g: (self.g * other.g).clamp(0.0, 1.0),
            b: (self.b * other.b).clamp(0.0, 1.0),
            a: self.a, // alpha is not affected by multiplication
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: (self.r + other.r).clamp(0.0, 1.0),
            g: (self.g + other.g).clamp(0.0, 1.0),
            b: (self.b + other.b).clamp(0.0, 1.0),
            a: self.a, // alpha is not affected by addition
        }
    }
}