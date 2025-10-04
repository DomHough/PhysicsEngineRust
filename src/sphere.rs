use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct Sphere {
    radius: f32,
    position: Vec3
}

impl Sphere {
    pub fn new(radius: f32, position: Vec3) -> Self {
        Sphere { radius, position }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.position;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}