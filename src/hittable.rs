use crate::material::Material;
use crate::ray::{Ray, Segment};
use crate::vec3::Vec3;

pub(crate) trait Hittable {
    fn intersects_ray(&self, ray: &Ray) -> Option<(f32, Vec3, Vec3)>; // (t, point, normal)
    fn intersects_segment(&self, segment: &Segment) -> Option<(f32, Vec3, Vec3)>;
    fn material(&self) -> &Material;


}