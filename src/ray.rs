use crate::vec3::Vec3;

pub(crate) struct Ray {
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
}