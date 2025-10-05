use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material; // new import

#[derive(Debug)]
pub(crate) struct InfinitePlane {
    position: Vec3,
    normal: Vec3,
    material: Material,
}

impl InfinitePlane {
    pub fn new(position: Vec3, normal: Vec3, material: Material) -> Self {
        InfinitePlane { position, normal: normal.normalized(), material }
    }
    pub fn intersects(&self, ray: &Ray) -> Option<(f32, Vec3, Vec3)> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < 1e-6 { return  None } // Ray is parallel to the plane
        let t = (self.position - ray.origin).dot(&self.normal) / denom;
        if t < 0.0 { return None } // Intersection is behind the ray's origin
        let point = ray.origin + ray.direction * t;
        Some((t, point, self.normal))
    }

    pub fn material(&self) -> &Material { &self.material }
}