// filepath: src/objects/sphere.rs

use crate::consts::EPS;
use crate::ray::{Ray, Segment};
use crate::vec3::Vec3;
use crate::material::Material;
use crate::objects::hittable::Hittable;

#[derive(Debug)]
pub(crate) struct Sphere {
    radius: f32,
    position: Vec3,
    material: Material,
}

impl Sphere {
    pub(crate) fn new(radius: f32, position: Vec3, material: Material) -> Self {
        Sphere { radius, position, material }
    }

    fn material(&self) -> &Material { &self.material }
}

impl Hittable for Sphere {
    fn intersects_ray(&self, ray: &Ray) -> Option<(f32, Vec3, Vec3)> {
        let oc = ray.origin - self.position;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let mut discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            if discriminant > -EPS { discriminant = 0.0; } else { return None; }
        }
        let sqrtd = discriminant.sqrt();
        let mut t = (-half_b - sqrtd) / a;
        if t <= EPS {
            t = (-half_b + sqrtd) / a;
            if t <= EPS { return None; }
        }
        let point = ray.origin + ray.direction * t;
        let normal = (point - self.position).normalized();
        Some((t, point, normal))
    }

    fn intersects_segment(&self, segment: &Segment) -> Option<(f32, Vec3, Vec3)> {
        let dir = (segment.end - segment.start).normalized();
        let ray = Ray::new(segment.start, dir);
        if let Some((t, point, normal)) = self.intersects_ray(&ray) {
            let seg_len = (segment.end - segment.start).length();
            if t <= seg_len + EPS {
                return Some((t, point, normal));
            }
        }
        None
    }

    fn material(&self) -> &Material { &self.material }
}

