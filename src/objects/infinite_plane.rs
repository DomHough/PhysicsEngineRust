// filepath: src/objects/infinite_plane.rs
use crate::consts::EPS;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;
use crate::objects::hittable::Hittable;

#[derive(Debug)]
pub(crate) struct InfinitePlane {
    position: Vec3,
    normal: Vec3,
    material: Material,
}

impl InfinitePlane {
    pub(crate) fn new(position: Vec3, normal: Vec3, material: Material) -> Self {
        InfinitePlane { position, normal: normal.normalized(), material }
    }

    fn material(&self) -> &Material { &self.material }
}

impl Hittable for InfinitePlane {
    fn intersects_ray(&self, ray: &Ray) -> Option<(f32, Vec3, Vec3)> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < EPS { return  None } // Ray is parallel to the plane
        let t = (self.position - ray.origin).dot(&self.normal) / denom;
        if t < EPS { return None } // Intersection is behind the ray's origin
        let point = ray.origin + ray.direction * t;
        Some((t, point, self.normal))
    }
    fn intersects_segment(&self, segment: &crate::ray::Segment) -> Option<(f32, Vec3, Vec3)> {
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

