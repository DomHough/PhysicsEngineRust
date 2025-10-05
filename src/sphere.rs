use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material; // new import

#[derive(Debug)]
pub(crate) struct Sphere {
    radius: f32,
    position: Vec3,
    material: Material,
}

impl Sphere {
    pub fn new(radius: f32, position: Vec3, material: Material) -> Self {
        Sphere { radius, position, material }
    }

    pub fn material(&self) -> &Material { &self.material }

    pub fn intersects(&self, ray: &Ray) -> Option<(f32, Vec3, Vec3)> {
        let oc = ray.origin - self.position;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 { return None }
        let sqrtd = discriminant.sqrt();
        let t1 = (-b - sqrtd) / (2.0 * a);
        let t2 = (-b + sqrtd) / (2.0 * a);
        let t = if t1 > 0.0 { t1 } else if t2 > 0.0 { t2 } else { return None };
        let point = ray.origin + ray.direction * t;
        let normal = (point - self.position).normalized();
        Some((t, point, normal))
    }
}