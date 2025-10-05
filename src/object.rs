use crate::infinite_plane::InfinitePlane;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

#[derive(Debug)]
pub enum Object { Sphere(Sphere), InfinitePlane(InfinitePlane), }

impl Object {
    pub fn intersects(&self, ray: &Ray) -> Option<(f32, Vec3, Vec3)> {
        match self {
            Object::Sphere(s) => s.intersects(ray),
            Object::InfinitePlane(p) => p.intersects(ray),
        }
    }
    pub fn material(&self) -> &crate::material::Material {
        match self {
            Object::Sphere(s) => s.material(),
            Object::InfinitePlane(p) => p.material(),
        }
    }
}